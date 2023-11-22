pub mod platform;

use directories::BaseDirs;
use flate2::read::GzDecoder;
use http::StatusCode;
use reqwest::blocking as reqwest;
use std::fs::{copy, create_dir_all, metadata, File};
use std::io;
use std::io::prelude::*;
use std::path::{Path, PathBuf};
use std::process::Command;

pub static PRISMA_CLI_VERSION: &str = "5.1.0";
// commit hash of prisma/prisma-engines, not brendonovich/prisma-engines
pub static ENGINE_VERSION: &str = "a9b7003df90aa623086e4d6f4e43c72468e6339b";
pub static BASE_DIR_NAME: &str = "prisma/binaries";

pub struct Engine<'a> {
    pub name: &'a str,
    pub env: &'a str,
}

pub const ENGINES: [Engine; 2] = [
    Engine {
        name: "query-engine",
        env: "PRISMA_QUERY_ENGINE_BINARY",
    },
    Engine {
        name: "schema-engine",
        env: "PRISMA_SCHEMA_ENGINE_BINARY",
    },
];

pub fn prisma_cli_name() -> String {
    let variation = platform::name();
    let arch = platform::arch();

    format!("prisma-cli-{variation}-{arch}")
}

pub fn global_cache_dir() -> PathBuf {
    let base_dirs = BaseDirs::new().unwrap();
    let cache_dir = base_dirs.cache_dir();

    cache_dir
        .join(BASE_DIR_NAME)
        .join("cli")
        .join(PRISMA_CLI_VERSION)
}

pub fn download_cli(to_dir: &PathBuf) -> Result<(), String> {
    let cli = prisma_cli_name();

    let to = platform::check_for_extension(&platform::name(), &to_dir.join(cli).to_str().unwrap());

    let url = platform::check_for_extension(
        &platform::name(),
        &format!(
            "https://prisma-photongo.s3-eu-west-1.amazonaws.com/{}-{}-{}-{}.gz",
            "prisma-cli",
            PRISMA_CLI_VERSION,
            platform::name(),
            platform::arch()
        ),
    );

    match metadata(&to) {
        Err(_) => (),
        Ok(_) => {
            return Ok(());
        }
    };

    println!("Downloading {} to {}", url, to);

    download(url.clone(), to.clone()).expect(&format!("could not download {} to {}", url, to));

    Ok(())
}

pub fn download_engine(engine_name: &str, to_dir: &PathBuf) -> Result<(), String> {
    let os_name = platform::binary_platform_name();

    let to = platform::check_for_extension(
        &os_name.to_string(),
        &to_dir
            .join(ENGINE_VERSION)
            .join(format!("prisma-{}-{}", engine_name, os_name))
            .into_os_string()
            .into_string()
            .unwrap(),
    );

    let url = platform::check_for_extension(
        &os_name.to_string(),
        &format!(
            "https://binaries.prisma.sh/all_commits/{}/{}/{}.gz",
            ENGINE_VERSION, &os_name, engine_name
        ),
    );

    match metadata(&to) {
        Err(_) => {}
        Ok(_) => {
            return Ok(());
        }
    };

    println!("Downloading {} to {}", url, to);
    download(url.clone(), to.clone()).expect(&format!("could not download {} to {}", url, to));

    Ok(())
}

fn download(url: String, to: String) -> Result<(), ()> {
    create_dir_all(Path::new(&to).parent().unwrap()).unwrap();

    let tmp = &(to.clone() + ".tmp");

    let resp = reqwest::get(&url).unwrap();

    if resp.status() != StatusCode::OK {
        panic!("received code {} from {}", resp.status(), &url);
    };

    let mut tmp_file = File::create(tmp).expect(&format!("could not create {}", tmp));

    if !cfg!(target_os = "windows") {
        Command::new("chmod")
            .arg("+x")
            .arg(tmp)
            .output()
            .expect("failed to make file executable");
    }

    let mut buffer = Vec::new();
    io::BufReader::new(GzDecoder::new(resp))
        .read_to_end(&mut buffer)
        .unwrap();

    tmp_file
        .write(buffer.as_slice())
        .expect("could not write to .tmp file");

    copy(tmp, to).expect(&format!("could not copy file {}", url));

    Ok(())
}
