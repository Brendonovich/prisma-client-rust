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

pub static PRISMA_CLI_VERSION: &str = "3.10.0";
pub static BASE_DIR_NAME: &str = "prisma/binaries";

pub fn prisma_cli_name() -> String {
    let variation = platform::name();

    format!("prisma-cli-{}", variation)
}

pub fn global_cache_dir() -> PathBuf {
    let base_dirs = BaseDirs::new().unwrap();
    let cache_dir = base_dirs.cache_dir();

    cache_dir
        .join(BASE_DIR_NAME)
        .join("cli")
        .join(PRISMA_CLI_VERSION)
}

pub fn download_cli(to_dir: &PathBuf) -> Result<(), ()> {
    let cli = prisma_cli_name();

    let to = platform::check_for_extension(&platform::name(), &to_dir.join(cli).to_str().unwrap());

    let url = platform::check_for_extension(
        &platform::name(),
        &format!(
            "https://prisma-photongo.s3-eu-west-1.amazonaws.com/{}-{}-{}.gz",
            "prisma-cli",
            PRISMA_CLI_VERSION,
            platform::name()
        ),
    );

    match metadata(&to) {
        Err(_) => (),
        Ok(_) => {
            // println!("{} is cached", to.to_string());
            return Ok(());
        }
    };

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
