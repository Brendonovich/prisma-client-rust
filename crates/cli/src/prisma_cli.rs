use crate::binaries::{self, platform, ENGINES};
use std::env;
use std::process::Command;

pub fn main(args: &Vec<String>) {
    let dir = binaries::global_cache_dir();

    binaries::download_cli(&dir).unwrap();
    let prisma = binaries::prisma_cli_name();
    let binary_name =
        platform::check_for_extension(&platform::name(), &platform::binary_platform_name());

    let mut cmd = Command::new(dir.join(prisma));

    cmd.args(args);

    cmd.envs(env::vars());
    cmd.env("PRISMA_HIDE_UPDATE_MESSAGE", "true");
    cmd.env("PRISMA_CLI_QUERY_ENGINE_TYPE", "binary");

    for e in ENGINES {
        match env::var(e.env) {
            Ok(path) => {
                cmd.env(e.env, path);
            }
            Err(_) => {
                binaries::download_engine(&e.name, &dir).unwrap();
                let path = dir
                    .join(binaries::ENGINE_VERSION)
                    .join(format!("prisma-{}-{}", e.name, binary_name));
                cmd.env(e.env, path);
            }
        }
    }

    cmd.stdout(std::process::Stdio::inherit());
    cmd.stdin(std::process::Stdio::inherit());
    cmd.stderr(std::process::Stdio::inherit());

    cmd.output().unwrap();
}
