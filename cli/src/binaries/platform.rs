use std::env;
use std::ops::Add;
use std::process::Command;

pub fn name() -> String {
    let os = env::consts::OS;

    match os {
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
