use regex::{Match, Regex};
use std::env;
use std::ops::Add;
use std::process::Command;

// TODO: openssl binary
pub fn binary_platform_name() -> String {
    let platform = name();

    match platform.as_str() {
        "linux" => match get_linux_distro().as_str() {
            "alpine" => "linux-musl".to_string(),
            distro => panic!("unimplemented distro: {}", distro),
        },
        _ => platform,
    }
}

pub fn name() -> String {
    let os = env::consts::OS;

    match os {
        "macos" => "darwin".to_string(),
        os => os.to_string(),
    }
}

fn get_linux_distro() -> String {
    let out = Command::new("cat").arg("/etc/os-release").output().unwrap();

    let stdout = String::from_utf8(out.stdout).unwrap();
    let stderr = String::from_utf8(out.stderr).unwrap();

    if stdout != "" {
    } else if stderr != "" {
    }

    "debian".to_string()
}

fn parse_linux_distro(str: String) -> String {
    let id_matches = Regex::new(r#"(?m)^ID="?([^"\n]*)"?"#)
        .unwrap()
        .find_iter(&str)
        .collect::<Vec<Match>>();

    let id = if id_matches.len() > 0 {
        id_matches[0].as_str().to_string()
    } else {
        "".to_string()
    };

    let id_like_matches = Regex::new(r#"(?m)^ID_LIKE="?([^"\n]*)"?"#)
        .unwrap()
        .find_iter(&str)
        .collect::<Vec<Match>>();

    let id_like = if id_like_matches.len() > 0 {
        id_like_matches[0].as_str().to_string()
    } else {
        "".to_string()
    };

    if id == "alpine" {
        return "alpine".to_string();
    };

    if id_like.contains("centos")
        || id_like.contains("fedora")
        || id_like.contains("rhel")
        || id == "fedora"
    {
        return "rhel".to_string();
    };

    if id_like.contains("debian") || id_like.contains("ubuntu") || id == "debian" {
        return "debian".to_string();
    }

    "debian".to_string()
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
