use serde::Deserialize;

#[derive(Default, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ClientFormat {
    #[default]
    File,
    Folder,
}

#[derive(Deserialize)]
pub struct SharedConfig {
    #[serde(default)]
    pub client_format: ClientFormat,
}
