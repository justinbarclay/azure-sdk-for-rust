#![doc = "generated by AutoRust 0.1.0"]
#![allow(unused_mut)]
#![allow(unused_variables)]
#![allow(unused_imports)]
use crate::models::*;
use snafu::{ResultExt, Snafu};
pub mod operations {
    use crate::models::*;
    use snafu::{ResultExt, Snafu};
    pub async fn list(operation_config: &crate::OperationConfig) -> std::result::Result<OperationList, list::Error> {
        let http_client = operation_config.http_client();
        let url_str = &format!("{}/providers/Microsoft.WindowsESU/operations", operation_config.base_path(),);
        let mut url = url::Url::parse(url_str).context(list::ParseUrlError)?;
        let mut req_builder = http::request::Builder::new();
        req_builder = req_builder.method(http::Method::GET);
        if let Some(token_credential) = operation_config.token_credential() {
            let token_response = token_credential
                .get_token(operation_config.token_credential_resource())
                .await
                .context(list::GetTokenError)?;
            req_builder = req_builder.header(http::header::AUTHORIZATION, format!("Bearer {}", token_response.token.secret()));
        }
        url.query_pairs_mut().append_pair("api-version", operation_config.api_version());
        let req_body = bytes::Bytes::from_static(azure_core::EMPTY_BODY);
        req_builder = req_builder.uri(url.as_str());
        let req = req_builder.body(req_body).context(list::BuildRequestError)?;
        let rsp = http_client.execute_request(req).await.context(list::ExecuteRequestError)?;
        match rsp.status() {
            http::StatusCode::OK => {
                let rsp_body = rsp.body();
                let rsp_value: OperationList =
                    serde_json::from_slice(rsp_body).context(list::DeserializeError { body: rsp_body.clone() })?;
                Ok(rsp_value)
            }
            status_code => {
                let rsp_body = rsp.body();
                let rsp_value: ErrorResponse =
                    serde_json::from_slice(rsp_body).context(list::DeserializeError { body: rsp_body.clone() })?;
                list::DefaultResponse {
                    status_code,
                    value: rsp_value,
                }
                .fail()
            }
        }
    }
    pub mod list {
        use crate::{models, models::*};
        use snafu::Snafu;
        #[derive(Debug, Snafu)]
        #[snafu(visibility(pub(crate)))]
        pub enum Error {
            DefaultResponse {
                status_code: http::StatusCode,
                value: models::ErrorResponse,
            },
            ParseUrlError {
                source: url::ParseError,
            },
            BuildRequestError {
                source: http::Error,
            },
            ExecuteRequestError {
                source: Box<dyn std::error::Error + Sync + Send>,
            },
            SerializeError {
                source: Box<dyn std::error::Error + Sync + Send>,
            },
            DeserializeError {
                source: serde_json::Error,
                body: bytes::Bytes,
            },
            GetTokenError {
                source: azure_core::errors::AzureError,
            },
        }
    }
}
pub mod multiple_activation_keys {
    use crate::models::*;
    use snafu::{ResultExt, Snafu};
    pub async fn list(
        operation_config: &crate::OperationConfig,
        subscription_id: &str,
    ) -> std::result::Result<MultipleActivationKeyList, list::Error> {
        let http_client = operation_config.http_client();
        let url_str = &format!(
            "{}/subscriptions/{}/providers/Microsoft.WindowsESU/multipleActivationKeys",
            operation_config.base_path(),
            subscription_id
        );
        let mut url = url::Url::parse(url_str).context(list::ParseUrlError)?;
        let mut req_builder = http::request::Builder::new();
        req_builder = req_builder.method(http::Method::GET);
        if let Some(token_credential) = operation_config.token_credential() {
            let token_response = token_credential
                .get_token(operation_config.token_credential_resource())
                .await
                .context(list::GetTokenError)?;
            req_builder = req_builder.header(http::header::AUTHORIZATION, format!("Bearer {}", token_response.token.secret()));
        }
        url.query_pairs_mut().append_pair("api-version", operation_config.api_version());
        let req_body = bytes::Bytes::from_static(azure_core::EMPTY_BODY);
        req_builder = req_builder.uri(url.as_str());
        let req = req_builder.body(req_body).context(list::BuildRequestError)?;
        let rsp = http_client.execute_request(req).await.context(list::ExecuteRequestError)?;
        match rsp.status() {
            http::StatusCode::OK => {
                let rsp_body = rsp.body();
                let rsp_value: MultipleActivationKeyList =
                    serde_json::from_slice(rsp_body).context(list::DeserializeError { body: rsp_body.clone() })?;
                Ok(rsp_value)
            }
            status_code => {
                let rsp_body = rsp.body();
                let rsp_value: ErrorResponse =
                    serde_json::from_slice(rsp_body).context(list::DeserializeError { body: rsp_body.clone() })?;
                list::DefaultResponse {
                    status_code,
                    value: rsp_value,
                }
                .fail()
            }
        }
    }
    pub mod list {
        use crate::{models, models::*};
        use snafu::Snafu;
        #[derive(Debug, Snafu)]
        #[snafu(visibility(pub(crate)))]
        pub enum Error {
            DefaultResponse {
                status_code: http::StatusCode,
                value: models::ErrorResponse,
            },
            ParseUrlError {
                source: url::ParseError,
            },
            BuildRequestError {
                source: http::Error,
            },
            ExecuteRequestError {
                source: Box<dyn std::error::Error + Sync + Send>,
            },
            SerializeError {
                source: Box<dyn std::error::Error + Sync + Send>,
            },
            DeserializeError {
                source: serde_json::Error,
                body: bytes::Bytes,
            },
            GetTokenError {
                source: azure_core::errors::AzureError,
            },
        }
    }
    pub async fn list_by_resource_group(
        operation_config: &crate::OperationConfig,
        subscription_id: &str,
        resource_group_name: &str,
    ) -> std::result::Result<MultipleActivationKeyList, list_by_resource_group::Error> {
        let http_client = operation_config.http_client();
        let url_str = &format!(
            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.WindowsESU/multipleActivationKeys",
            operation_config.base_path(),
            subscription_id,
            resource_group_name
        );
        let mut url = url::Url::parse(url_str).context(list_by_resource_group::ParseUrlError)?;
        let mut req_builder = http::request::Builder::new();
        req_builder = req_builder.method(http::Method::GET);
        if let Some(token_credential) = operation_config.token_credential() {
            let token_response = token_credential
                .get_token(operation_config.token_credential_resource())
                .await
                .context(list_by_resource_group::GetTokenError)?;
            req_builder = req_builder.header(http::header::AUTHORIZATION, format!("Bearer {}", token_response.token.secret()));
        }
        url.query_pairs_mut().append_pair("api-version", operation_config.api_version());
        let req_body = bytes::Bytes::from_static(azure_core::EMPTY_BODY);
        req_builder = req_builder.uri(url.as_str());
        let req = req_builder.body(req_body).context(list_by_resource_group::BuildRequestError)?;
        let rsp = http_client
            .execute_request(req)
            .await
            .context(list_by_resource_group::ExecuteRequestError)?;
        match rsp.status() {
            http::StatusCode::OK => {
                let rsp_body = rsp.body();
                let rsp_value: MultipleActivationKeyList =
                    serde_json::from_slice(rsp_body).context(list_by_resource_group::DeserializeError { body: rsp_body.clone() })?;
                Ok(rsp_value)
            }
            status_code => {
                let rsp_body = rsp.body();
                let rsp_value: ErrorResponse =
                    serde_json::from_slice(rsp_body).context(list_by_resource_group::DeserializeError { body: rsp_body.clone() })?;
                list_by_resource_group::DefaultResponse {
                    status_code,
                    value: rsp_value,
                }
                .fail()
            }
        }
    }
    pub mod list_by_resource_group {
        use crate::{models, models::*};
        use snafu::Snafu;
        #[derive(Debug, Snafu)]
        #[snafu(visibility(pub(crate)))]
        pub enum Error {
            DefaultResponse {
                status_code: http::StatusCode,
                value: models::ErrorResponse,
            },
            ParseUrlError {
                source: url::ParseError,
            },
            BuildRequestError {
                source: http::Error,
            },
            ExecuteRequestError {
                source: Box<dyn std::error::Error + Sync + Send>,
            },
            SerializeError {
                source: Box<dyn std::error::Error + Sync + Send>,
            },
            DeserializeError {
                source: serde_json::Error,
                body: bytes::Bytes,
            },
            GetTokenError {
                source: azure_core::errors::AzureError,
            },
        }
    }
    pub async fn get(
        operation_config: &crate::OperationConfig,
        subscription_id: &str,
        resource_group_name: &str,
        multiple_activation_key_name: &str,
    ) -> std::result::Result<MultipleActivationKey, get::Error> {
        let http_client = operation_config.http_client();
        let url_str = &format!(
            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.WindowsESU/multipleActivationKeys/{}",
            operation_config.base_path(),
            subscription_id,
            resource_group_name,
            multiple_activation_key_name
        );
        let mut url = url::Url::parse(url_str).context(get::ParseUrlError)?;
        let mut req_builder = http::request::Builder::new();
        req_builder = req_builder.method(http::Method::GET);
        if let Some(token_credential) = operation_config.token_credential() {
            let token_response = token_credential
                .get_token(operation_config.token_credential_resource())
                .await
                .context(get::GetTokenError)?;
            req_builder = req_builder.header(http::header::AUTHORIZATION, format!("Bearer {}", token_response.token.secret()));
        }
        url.query_pairs_mut().append_pair("api-version", operation_config.api_version());
        let req_body = bytes::Bytes::from_static(azure_core::EMPTY_BODY);
        req_builder = req_builder.uri(url.as_str());
        let req = req_builder.body(req_body).context(get::BuildRequestError)?;
        let rsp = http_client.execute_request(req).await.context(get::ExecuteRequestError)?;
        match rsp.status() {
            http::StatusCode::OK => {
                let rsp_body = rsp.body();
                let rsp_value: MultipleActivationKey =
                    serde_json::from_slice(rsp_body).context(get::DeserializeError { body: rsp_body.clone() })?;
                Ok(rsp_value)
            }
            status_code => {
                let rsp_body = rsp.body();
                let rsp_value: ErrorResponse =
                    serde_json::from_slice(rsp_body).context(get::DeserializeError { body: rsp_body.clone() })?;
                get::DefaultResponse {
                    status_code,
                    value: rsp_value,
                }
                .fail()
            }
        }
    }
    pub mod get {
        use crate::{models, models::*};
        use snafu::Snafu;
        #[derive(Debug, Snafu)]
        #[snafu(visibility(pub(crate)))]
        pub enum Error {
            DefaultResponse {
                status_code: http::StatusCode,
                value: models::ErrorResponse,
            },
            ParseUrlError {
                source: url::ParseError,
            },
            BuildRequestError {
                source: http::Error,
            },
            ExecuteRequestError {
                source: Box<dyn std::error::Error + Sync + Send>,
            },
            SerializeError {
                source: Box<dyn std::error::Error + Sync + Send>,
            },
            DeserializeError {
                source: serde_json::Error,
                body: bytes::Bytes,
            },
            GetTokenError {
                source: azure_core::errors::AzureError,
            },
        }
    }
    pub async fn create(
        operation_config: &crate::OperationConfig,
        subscription_id: &str,
        resource_group_name: &str,
        multiple_activation_key_name: &str,
        multiple_activation_key: &MultipleActivationKey,
    ) -> std::result::Result<create::Response, create::Error> {
        let http_client = operation_config.http_client();
        let url_str = &format!(
            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.WindowsESU/multipleActivationKeys/{}",
            operation_config.base_path(),
            subscription_id,
            resource_group_name,
            multiple_activation_key_name
        );
        let mut url = url::Url::parse(url_str).context(create::ParseUrlError)?;
        let mut req_builder = http::request::Builder::new();
        req_builder = req_builder.method(http::Method::PUT);
        if let Some(token_credential) = operation_config.token_credential() {
            let token_response = token_credential
                .get_token(operation_config.token_credential_resource())
                .await
                .context(create::GetTokenError)?;
            req_builder = req_builder.header(http::header::AUTHORIZATION, format!("Bearer {}", token_response.token.secret()));
        }
        url.query_pairs_mut().append_pair("api-version", operation_config.api_version());
        let req_body = bytes::Bytes::from_static(azure_core::EMPTY_BODY);
        req_builder = req_builder.uri(url.as_str());
        let req = req_builder.body(req_body).context(create::BuildRequestError)?;
        let rsp = http_client.execute_request(req).await.context(create::ExecuteRequestError)?;
        match rsp.status() {
            http::StatusCode::OK => {
                let rsp_body = rsp.body();
                let rsp_value: MultipleActivationKey =
                    serde_json::from_slice(rsp_body).context(create::DeserializeError { body: rsp_body.clone() })?;
                Ok(create::Response::Ok200(rsp_value))
            }
            http::StatusCode::CREATED => {
                let rsp_body = rsp.body();
                let rsp_value: MultipleActivationKey =
                    serde_json::from_slice(rsp_body).context(create::DeserializeError { body: rsp_body.clone() })?;
                Ok(create::Response::Created201(rsp_value))
            }
            status_code => {
                let rsp_body = rsp.body();
                let rsp_value: ErrorResponse =
                    serde_json::from_slice(rsp_body).context(create::DeserializeError { body: rsp_body.clone() })?;
                create::DefaultResponse {
                    status_code,
                    value: rsp_value,
                }
                .fail()
            }
        }
    }
    pub mod create {
        use crate::{models, models::*};
        use snafu::Snafu;
        #[derive(Debug)]
        pub enum Response {
            Ok200(MultipleActivationKey),
            Created201(MultipleActivationKey),
        }
        #[derive(Debug, Snafu)]
        #[snafu(visibility(pub(crate)))]
        pub enum Error {
            DefaultResponse {
                status_code: http::StatusCode,
                value: models::ErrorResponse,
            },
            ParseUrlError {
                source: url::ParseError,
            },
            BuildRequestError {
                source: http::Error,
            },
            ExecuteRequestError {
                source: Box<dyn std::error::Error + Sync + Send>,
            },
            SerializeError {
                source: Box<dyn std::error::Error + Sync + Send>,
            },
            DeserializeError {
                source: serde_json::Error,
                body: bytes::Bytes,
            },
            GetTokenError {
                source: azure_core::errors::AzureError,
            },
        }
    }
    pub async fn update(
        operation_config: &crate::OperationConfig,
        subscription_id: &str,
        resource_group_name: &str,
        multiple_activation_key_name: &str,
        multiple_activation_key: &MultipleActivationKeyUpdate,
    ) -> std::result::Result<MultipleActivationKey, update::Error> {
        let http_client = operation_config.http_client();
        let url_str = &format!(
            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.WindowsESU/multipleActivationKeys/{}",
            operation_config.base_path(),
            subscription_id,
            resource_group_name,
            multiple_activation_key_name
        );
        let mut url = url::Url::parse(url_str).context(update::ParseUrlError)?;
        let mut req_builder = http::request::Builder::new();
        req_builder = req_builder.method(http::Method::PATCH);
        if let Some(token_credential) = operation_config.token_credential() {
            let token_response = token_credential
                .get_token(operation_config.token_credential_resource())
                .await
                .context(update::GetTokenError)?;
            req_builder = req_builder.header(http::header::AUTHORIZATION, format!("Bearer {}", token_response.token.secret()));
        }
        url.query_pairs_mut().append_pair("api-version", operation_config.api_version());
        let req_body = bytes::Bytes::from_static(azure_core::EMPTY_BODY);
        req_builder = req_builder.uri(url.as_str());
        let req = req_builder.body(req_body).context(update::BuildRequestError)?;
        let rsp = http_client.execute_request(req).await.context(update::ExecuteRequestError)?;
        match rsp.status() {
            http::StatusCode::OK => {
                let rsp_body = rsp.body();
                let rsp_value: MultipleActivationKey =
                    serde_json::from_slice(rsp_body).context(update::DeserializeError { body: rsp_body.clone() })?;
                Ok(rsp_value)
            }
            status_code => {
                let rsp_body = rsp.body();
                let rsp_value: ErrorResponse =
                    serde_json::from_slice(rsp_body).context(update::DeserializeError { body: rsp_body.clone() })?;
                update::DefaultResponse {
                    status_code,
                    value: rsp_value,
                }
                .fail()
            }
        }
    }
    pub mod update {
        use crate::{models, models::*};
        use snafu::Snafu;
        #[derive(Debug, Snafu)]
        #[snafu(visibility(pub(crate)))]
        pub enum Error {
            DefaultResponse {
                status_code: http::StatusCode,
                value: models::ErrorResponse,
            },
            ParseUrlError {
                source: url::ParseError,
            },
            BuildRequestError {
                source: http::Error,
            },
            ExecuteRequestError {
                source: Box<dyn std::error::Error + Sync + Send>,
            },
            SerializeError {
                source: Box<dyn std::error::Error + Sync + Send>,
            },
            DeserializeError {
                source: serde_json::Error,
                body: bytes::Bytes,
            },
            GetTokenError {
                source: azure_core::errors::AzureError,
            },
        }
    }
    pub async fn delete(
        operation_config: &crate::OperationConfig,
        subscription_id: &str,
        resource_group_name: &str,
        multiple_activation_key_name: &str,
    ) -> std::result::Result<delete::Response, delete::Error> {
        let http_client = operation_config.http_client();
        let url_str = &format!(
            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.WindowsESU/multipleActivationKeys/{}",
            operation_config.base_path(),
            subscription_id,
            resource_group_name,
            multiple_activation_key_name
        );
        let mut url = url::Url::parse(url_str).context(delete::ParseUrlError)?;
        let mut req_builder = http::request::Builder::new();
        req_builder = req_builder.method(http::Method::DELETE);
        if let Some(token_credential) = operation_config.token_credential() {
            let token_response = token_credential
                .get_token(operation_config.token_credential_resource())
                .await
                .context(delete::GetTokenError)?;
            req_builder = req_builder.header(http::header::AUTHORIZATION, format!("Bearer {}", token_response.token.secret()));
        }
        url.query_pairs_mut().append_pair("api-version", operation_config.api_version());
        let req_body = bytes::Bytes::from_static(azure_core::EMPTY_BODY);
        req_builder = req_builder.uri(url.as_str());
        let req = req_builder.body(req_body).context(delete::BuildRequestError)?;
        let rsp = http_client.execute_request(req).await.context(delete::ExecuteRequestError)?;
        match rsp.status() {
            http::StatusCode::OK => Ok(delete::Response::Ok200),
            http::StatusCode::NO_CONTENT => Ok(delete::Response::NoContent204),
            status_code => {
                let rsp_body = rsp.body();
                let rsp_value: ErrorResponse =
                    serde_json::from_slice(rsp_body).context(delete::DeserializeError { body: rsp_body.clone() })?;
                delete::DefaultResponse {
                    status_code,
                    value: rsp_value,
                }
                .fail()
            }
        }
    }
    pub mod delete {
        use crate::{models, models::*};
        use snafu::Snafu;
        #[derive(Debug)]
        pub enum Response {
            Ok200,
            NoContent204,
        }
        #[derive(Debug, Snafu)]
        #[snafu(visibility(pub(crate)))]
        pub enum Error {
            DefaultResponse {
                status_code: http::StatusCode,
                value: models::ErrorResponse,
            },
            ParseUrlError {
                source: url::ParseError,
            },
            BuildRequestError {
                source: http::Error,
            },
            ExecuteRequestError {
                source: Box<dyn std::error::Error + Sync + Send>,
            },
            SerializeError {
                source: Box<dyn std::error::Error + Sync + Send>,
            },
            DeserializeError {
                source: serde_json::Error,
                body: bytes::Bytes,
            },
            GetTokenError {
                source: azure_core::errors::AzureError,
            },
        }
    }
}
