use std::env;
use std::ops::Add;

pub fn arch() -> String {
    match env::consts::ARCH {
        "x86_64" => "x64".to_string(),
        "aarch64" => "arm64".to_string(),
        arch => panic!("Architecture {arch} is not yet supported"),
    }
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
