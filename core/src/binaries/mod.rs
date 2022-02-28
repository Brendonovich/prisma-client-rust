pub mod bindata;
pub mod platform;
pub mod unpack;

pub use unpack::unpack;

use directories::BaseDirs;
use flate2::read::GzDecoder;
use http::StatusCode;
use reqwest::blocking as reqwest;
use std::fs::{copy, create_dir_all, metadata, File};
use std::io;
use std::io::prelude::*;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::SystemTime;

pub static PRISMA_VERSION: &str = "3.9.1";
pub static ENGINE_VERSION: &str = "bcc2ff906db47790ee902e7bbc76d7ffb1893009";

pub static BASE_DIR_NAME: &str = "prisma/binaries";

pub struct Engine<'a> {
    pub name: &'a str,
    pub env: &'a str,
}

pub const ENGINES: [Engine; 4] = [
    Engine {
        name: "query-engine",
        env: "PRISMA_QUERY_ENGINE_BINARY",
    },
    Engine {
        name: "migration-engine",
        env: "PRISMA_MIGRATION_ENGINE_BINARY",
    },
    Engine {
        name: "introspection-engine",
        env: "PRISMA_INTROSPECTION_ENGINE_BINARY",
    },
    Engine {
        name: "prisma-fmt",
        env: "PRISMA_FMT_BINARY",
    },
];

pub fn prisma_cli_name() -> String {
    let variation = platform::name();

    format!("prisma-cli-{}", variation)
}

pub fn global_temp_dir() -> PathBuf {
    std::env::temp_dir()
        .join(BASE_DIR_NAME)
        .join("engines")
        .join(ENGINE_VERSION)
}

pub fn global_unpack_dir() -> PathBuf {
    global_temp_dir().join("unpacked")
}

pub fn global_cache_dir() -> PathBuf {
    let base_dirs = BaseDirs::new().unwrap();
    let cache_dir = base_dirs.cache_dir();

    cache_dir
        .join(BASE_DIR_NAME)
        .join("cli")
        .join(PRISMA_VERSION)
}

pub fn fetch_engine(
    to_dir: PathBuf,
    engine_name: String,
    binary_platform_name: String,
) -> Result<(), ()> {
    let to = platform::check_for_extension(
        &binary_platform_name,
        &to_dir
            .join(ENGINE_VERSION)
            .join(format!("prisma-{}-{}", engine_name, binary_platform_name))
            .into_os_string()
            .into_string()
            .unwrap(),
    );

    let binary_platform_remote_name = match binary_platform_name.as_str() {
        "linux" => "linux-musl",
        name => name,
    };

    let url = platform::check_for_extension(
        &binary_platform_name,
        &format!(
            "https://binaries.prisma.sh/all_commits/{}/{}/{}.gz",
            ENGINE_VERSION, binary_platform_remote_name, engine_name
        ),
    );

    match metadata(&to) {
        Err(_) => (),
        Ok(_) => {
            // println!("{} is cached", to.to_string());
            return Ok(());
        }
    };

    download(url.clone(), to.clone()).expect(&format!("could not download {} to {}", &url, &to));

    Ok(())
}

pub fn fetch_native(to_dir: PathBuf) -> Result<(), ()> {
    if !to_dir.is_absolute() {
        panic!("to_dir must be absolute")
    }

    download_cli(to_dir.clone()).expect("could not download engines");

    for e in &ENGINES {
        download_engine(e.name.to_string(), to_dir.clone()).expect("could not download engines");
    }

    Ok(())
}

fn download_cli(to_dir: PathBuf) -> Result<(), ()> {
    let cli = prisma_cli_name();

    let to = platform::check_for_extension(&platform::name(), &to_dir.join(cli).to_str().unwrap());

    let url = platform::check_for_extension(
        &platform::name(),
        &format!(
            "https://prisma-photongo.s3-eu-west-1.amazonaws.com/{}-{}-{}.gz",
            "prisma-cli",
            PRISMA_VERSION,
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

fn download_engine(engine_name: String, to_dir: PathBuf) -> Result<String, ()> {
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
            // println!("{} is cached", to.to_string());
            return Ok(to);
        }
    };

    let start_download = SystemTime::now();

    download(url.clone(), to.clone()).expect(&format!("could not download {} to {}", url, to));

    println!(
        "download() took {}",
        SystemTime::now()
            .duration_since(start_download)
            .unwrap()
            .as_millis()
    );

    Ok(to.to_string())
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

pub fn get_engine_path(dir: PathBuf, engine: &str, binary_name: &str) -> String {
    platform::check_for_extension(
        binary_name,
        &dir.join(ENGINE_VERSION)
            .join(&format!(
                "prisma-{}-{}",
                engine,
                platform::binary_platform_name()
            ))
            .to_str()
            .unwrap(),
    )
}
