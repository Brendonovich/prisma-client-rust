use crate::binaries;
use std::env;
use std::process::Command;

pub fn main(args: &Vec<String>) {
    let dir = binaries::global_cache_dir();

    binaries::fetch_native(&dir).unwrap();

    let prisma = binaries::prisma_cli_name();

    let mut cmd = Command::new(dir.join(prisma));

    cmd.args(args);

    cmd.envs(env::vars());
    cmd.env("PRISMA_HIDE_UPDATE_MESSAGE", "true");

    cmd.stdout(std::process::Stdio::inherit());
    cmd.stdin(std::process::Stdio::inherit());
    cmd.stderr(std::process::Stdio::inherit());

    cmd.output().unwrap();
}
