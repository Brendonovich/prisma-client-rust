use regex::Regex;
use std::env;
use std::ops::Add;
use std::process::Command;

pub fn binary_platform_name() -> String {
    let platform = name();

    let distro = match platform.as_str() {
        "linux" => match arch().as_str() {
            "arm64" => "linux-arm64".to_string(),
            _ => match get_linux_distro().as_str() {
                "alpine" => return "linux-musl".to_string(),
                distro => distro.to_string(),
            },
        },
        _ => return platform.to_string(),
    };

    let ssl = get_openssl();

    let name = format!("{}-openssl-{}", distro, ssl);

    name
}

pub fn arch() -> String {
    match env::consts::ARCH {
        "x86_64" => "x64".to_string(),
        "aarch64" => "arm64".to_string(),
        arch => panic!("Architecture {arch} is not yet supported"),
    }
}

fn get_linux_distro() -> String {
    let out = Command::new("cat").arg("/etc/os-release").output().unwrap();

    let stdout = String::from_utf8(out.stdout).unwrap();
    let stderr = String::from_utf8(out.stderr).unwrap();
    let combined_output = stdout + &stderr;

    parse_linux_distro(&combined_output)
}

fn parse_linux_distro(output: &str) -> String {
    let id = Regex::new("(?m)^ID=\"?([^\"\n]*)\"?")
        .unwrap()
        .captures(output)
        .and_then(|matches| {
            if matches.len() > 1 {
                Some(matches[1].to_string())
            } else {
                None
            }
        });

    let id_like = Regex::new("(?m)^ID_LIKE=\"?([^\"\n]*)\"?")
        .unwrap()
        .captures(output)
        .and_then(|matches| {
            if matches.len() > 1 {
                Some(matches[1].to_string())
            } else {
                None
            }
        });

    if let Some(id) = id {
        if id == "alpine" {
            return "alpine".to_string();
        }

        if let Some(id_like) = id_like {
            if id_like.contains("centos")
                || id_like.contains("fedora")
                || id_like.contains("rhel")
                || id == "fedora"
            {
                return "alpine".to_string();
            }

            if id_like.contains("debian") || id_like.contains("ubuntu") || id == "debian" {
                return "debian".to_string();
            }
        }
    }

    "debian".to_string()
}

pub fn name() -> String {
    match env::consts::OS {
        "macos" => "darwin".to_string(),
        os => os.to_string(),
    }
}

pub fn check_for_extension(platform: &str, path: &str) -> String {
    let path = path.to_string();

    if platform == "windows" {
        if path.contains(".gz") {
            return path.replace(".gz", ".exe.gz");
        }
        return path.add(".exe");
    }

    path
}

fn get_openssl() -> String {
    let out = Command::new("openssl")
        .arg("version")
        .arg("-v")
        .output()
        .expect("Unable to run determine openssl version; is the openssl binary installed?");

    let stdout = String::from_utf8(out.stdout).unwrap();
    let stderr = String::from_utf8(out.stderr).unwrap();
    let combined = stdout + &stderr;

    parse_openssl_version(&combined)
}

fn parse_openssl_version(v: &str) -> String {
    let r = Regex::new(r"^OpenSSL\s(\d+\.\d+)\.\d");
    let matches = r.unwrap().captures(v).unwrap();
    if matches.len() > 0 {
        match matches.get(1).unwrap().as_str() {
            "3.1" => "3.0.x".to_string(),
            version => format!("{version}.x"),
        }
    } else {
        "1.1.x".to_string()
    }
}
