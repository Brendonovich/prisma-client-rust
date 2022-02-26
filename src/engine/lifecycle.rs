use crate::{
    binaries::{self, platform},
    engine::protocol::GQLResponse,
};
use async_trait::async_trait;
use futures::TryFutureExt;
use http::Method;
use serde::Deserialize;
use std::{env, fs, path::Path, process::Command};

use super::{port, Engine, GQLRequest, QueryEngine, QueryEngineState};

#[derive(Deserialize)]
struct StatusResponse {
    status: String,
}

impl QueryEngine {
    pub fn new(schema: String, has_binary_targets: bool) -> Self {
        Self {
            schema,
            has_binary_targets,
            http: reqwest::Client::new(),
            state: QueryEngineState::NotRunning,
        }
    }

    async fn spawn(&mut self, file: String) {
        let port = port::get_port();

        let url = format!("http://localhost:{}", port);

        let child = Command::new(file)
            .args(["-p", &port, "--enable-raw-queries"])
            .envs([
                ("PRISMA_DML", &self.schema),
                ("RUST_LOG", &"error".to_string()),
                ("RUST_LOG_FORMAT", &"json".to_string()),
            ])
            .spawn()
            .unwrap();

        println!("starting engine...");

        self.state = QueryEngineState::Running { url, child };

        for _ in 0..100 {
            let body = self
                .request("GET", "/status", serde_json::Value::Null)
                .await;

            let body = match body {
                Some(body) => body,
                None => {
                    println!("could not connect; retrying...");
                    std::thread::sleep(std::time::Duration::from_millis(1000));
                    continue;
                }
            };

            let response = serde_json::from_str(&body);

            let _: StatusResponse = match response {
                Ok(response) => response,
                Err(_) => {
                    println!("could not unmarshal response; retrying...");
                    std::thread::sleep(std::time::Duration::from_millis(1000));
                    continue;
                }
            };

            break;
        }
    }

    async fn request(
        &self,
        method: &str,
        path: &str,
        payload: serde_json::Value,
    ) -> Option<String> {
        match &self.state {
            QueryEngineState::NotRunning => None,
            QueryEngineState::Running { url, .. } => {
                self.http
                    .request(
                        Method::from_bytes(method.as_bytes()).unwrap(),
                        format!("{}{}", url, path),
                    )
                    .header("content-type", "application/json")
                    .body(serde_json::to_string(&payload).unwrap())
                    .send()
                    .and_then(|res| res.text())
                    .await
                    .ok()
            }
        }
    }

    fn ensure() -> String {
        let binaries_path = binaries::global_unpack_dir();
        let binary_name = platform::check_for_extension(platform::name(), platform::name());
        let exact_binary_name =
            platform::check_for_extension(platform::name(), platform::binary_platform_name());

        let mut force_version = true;
        let name = "prisma-query-engine-";
        let local_path = Path::new("./").join(format!("{}{}", name, &binary_name));
        let local_exact_path = Path::new("./").join(format!("{}{}", name, &exact_binary_name));
        let global_path = Path::new(&binaries_path).join(format!("{}{}", name, &binary_name));
        let global_exact_path =
            Path::new(&binaries_path).join(format!("{}{}", name, &exact_binary_name));

        let mut file = None;

        let prisma_query_engine_binary = env::var("PRISMA_QUERY_ENGINE_BINARY");

        if let Ok(prisma_query_engine_binary) = prisma_query_engine_binary {
            println!(
                "PRISMA_QUERY_ENGINE_BINARY is defined, using {}",
                prisma_query_engine_binary
            );

            // might need to be let Ok()
            if let Err(_) = fs::metadata(&prisma_query_engine_binary) {
                panic!(
                    "PRISMA_QUERY_ENGINE_BINARY was provided, but no query engine was found at {}",
                    prisma_query_engine_binary
                );
            }

            file = Some(prisma_query_engine_binary.into());
        };

        if let Ok(_) = fs::metadata(&local_exact_path) {
            println!("exact query engine found in working directory");
            file = Some(local_exact_path.to_string_lossy().to_string());
        } else if let Ok(_) = fs::metadata(&local_path) {
            println!("query engine found in working directory");
            file = Some(local_path.to_string_lossy().to_string());
        } else if let Ok(_) = fs::metadata(&global_exact_path) {
            println!("exact query engine found in global directory");
            file = Some(global_exact_path.to_string_lossy().to_string());
        } else if let Ok(_) = fs::metadata(&global_path) {
            println!("query engine found in global directory");
            file = Some(global_path.to_string_lossy().to_string());
        } else {
            panic!("no query engine found");
        }

        let output = Command::new(file.clone().unwrap())
            .arg("--version")
            .output()
            .unwrap();

        let version = String::from_utf8_lossy(&output.stdout)
            .trim()
            .to_string()
            .replacen("query-engine", "", 1);

        // TODO Force version

        file.expect("bruh")
    }
}

#[async_trait]
impl Engine for QueryEngine {
    async fn connect(&mut self) {
        let file = Self::ensure();

        self.spawn(file).await;
    }

    fn disconnect(&mut self) {}

    async fn perform(&self, request: GQLRequest) -> GQLResponse {
        // TODO: handle errors
        let body = self
            .request("POST", "/", serde_json::to_value(request).unwrap())
            .await
            .unwrap();

        let response = serde_json::from_str(&body).unwrap();

        response
    }

    fn batch(&mut self) {}

    fn name(&self) -> String {
        "query-engine".to_string()
    }
}
