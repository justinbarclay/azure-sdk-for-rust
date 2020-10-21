#![doc = "generated by AutoRust 0.1.0"]
#[cfg(feature = "package-2019-06")]
mod package_2019_06;
#[cfg(feature = "package-2019-06")]
pub use package_2019_06::{models, operations, API_VERSION};
#[cfg(feature = "package-2018-02-preview")]
mod package_2018_02_preview;
#[cfg(feature = "package-2018-02-preview")]
pub use package_2018_02_preview::{models, operations, API_VERSION};
pub struct OperationConfig {
    pub api_version: String,
    pub client: reqwest::Client,
    pub base_path: String,
    pub bearer_access_token: Option<String>,
}
impl OperationConfig {
    pub fn new(bearer_access_token: &str) -> Self {
        Self {
            bearer_access_token: Some(bearer_access_token.to_owned()),
            ..Default::default()
        }
    }
}
impl Default for OperationConfig {
    fn default() -> Self {
        Self {
            api_version: API_VERSION.to_owned(),
            client: reqwest::Client::new(),
            base_path: "https://management.azure.com".to_owned(),
            bearer_access_token: None,
        }
    }
}
