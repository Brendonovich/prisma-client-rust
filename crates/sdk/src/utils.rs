use std::{path::PathBuf, process::Command};

pub fn rustfmt(paths: &[PathBuf]) {
    Command::new("rustfmt")
        .arg("--edition=2021")
        .args(paths.iter().map(|p| p.to_str().unwrap()))
        .output()
        .ok();
}
