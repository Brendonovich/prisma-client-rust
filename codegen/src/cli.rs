use prisma_client_rust_core::binaries::{self, platform};
use std::env;
use std::process::Command;

pub fn main(args: &Vec<String>) {
    let dir = binaries::global_cache_dir();

    binaries::fetch_native(dir.clone()).expect("could not fetch binaries");

    let prisma = binaries::prisma_cli_name();

    let mut cmd = Command::new(dir.join(prisma));

    cmd.args(args);

    let binary_name =
        platform::check_for_extension(&platform::name(), &platform::binary_platform_name());

    cmd.envs(env::vars());
    cmd.env("PRISMA_HIDE_UPDATE_MESSAGE", "true");

    for e in &binaries::ENGINES {
        let value: String;

        match env::var(e.env.to_string()) {
            Ok(var) => value = var,
            Err(_) => {
                value = dir
                    .join(binaries::ENGINE_VERSION)
                    .join(format!("prisma-{}-{}", e.name.to_string(), binary_name))
                    .into_os_string()
                    .into_string()
                    .unwrap()
            }
        }

        cmd.env(e.env.to_string(), value);
    }

    cmd.stdout(std::process::Stdio::inherit());
    cmd.stdin(std::process::Stdio::inherit());
    cmd.stderr(std::process::Stdio::inherit());
    
    cmd.output().unwrap();
}
