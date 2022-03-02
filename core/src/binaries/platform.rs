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
