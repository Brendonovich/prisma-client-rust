use std::{path::Path, process::Command};

pub fn rustfmt(path: &Path) {
    Command::new("rustfmt")
        .arg("--edition=2021")
        .arg(path.to_str().unwrap())
        .output()
        .ok();
}
