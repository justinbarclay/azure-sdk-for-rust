#![doc = "generated by AutoRust 0.1.0"]
#[cfg(feature = "package-2020-03-20")]
mod package_2020_03_20;
#[cfg(feature = "package-2020-03-20")]
pub use package_2020_03_20::{models, operations, API_VERSION};
#[cfg(feature = "package-2019-08-09-preview")]
mod package_2019_08_09_preview;
#[cfg(feature = "package-2019-08-09-preview")]
pub use package_2019_08_09_preview::{models, operations, API_VERSION};
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
