#![doc = "generated by AutoRust 0.1.0"]
#![allow(unused_mut)]
#![allow(unused_variables)]
#![allow(unused_imports)]
use crate::models::*;
use snafu::{ResultExt, Snafu};
pub mod recommendations {
    use crate::models::*;
    use snafu::{ResultExt, Snafu};
    pub async fn generate(operation_config: &crate::OperationConfig, subscription_id: &str) -> std::result::Result<(), generate::Error> {
        let http_client = operation_config.http_client();
        let url_str = &format!(
            "{}/subscriptions/{}/providers/Microsoft.Advisor/generateRecommendations",
            operation_config.base_path(),
            subscription_id
        );
        let mut url = url::Url::parse(url_str).context(generate::ParseUrlError)?;
        let mut req_builder = http::request::Builder::new();
        req_builder = req_builder.method(http::Method::POST);
        if let Some(token_credential) = operation_config.token_credential() {
            let token_response = token_credential
                .get_token(operation_config.token_credential_resource())
                .await
                .context(generate::GetTokenError)?;
            req_builder = req_builder.header(http::header::AUTHORIZATION, format!("Bearer {}", token_response.token.secret()));
        }
        url.query_pairs_mut().append_pair("api-version", operation_config.api_version());
        let req_body = bytes::Bytes::from_static(azure_core::EMPTY_BODY);
        req_builder = req_builder.header(http::header::CONTENT_LENGTH, 0);
        req_builder = req_builder.uri(url.as_str());
        let req = req_builder.body(req_body).context(generate::BuildRequestError)?;
        let rsp = http_client.execute_request(req).await.context(generate::ExecuteRequestError)?;
        match rsp.status() {
            http::StatusCode::ACCEPTED => Ok(()),
            status_code => {
                let rsp_body = rsp.body();
                generate::UnexpectedResponse {
                    status_code,
                    body: rsp_body.clone(),
                }
                .fail()
            }
        }
    }
    pub mod generate {
        use crate::{models, models::*};
        use snafu::Snafu;
        #[derive(Debug, Snafu)]
        #[snafu(visibility(pub(crate)))]
        pub enum Error {
            UnexpectedResponse { status_code: http::StatusCode, body: bytes::Bytes },
            ParseUrlError { source: url::ParseError },
            BuildRequestError { source: http::Error },
            ExecuteRequestError { source: Box<dyn std::error::Error + Sync + Send> },
            SerializeError { source: Box<dyn std::error::Error + Sync + Send> },
            DeserializeError { source: serde_json::Error, body: bytes::Bytes },
            GetTokenError { source: azure_core::errors::AzureError },
        }
    }
    pub async fn get_generate_recommendations_status(
        operation_config: &crate::OperationConfig,
        subscription_id: &str,
        operation_id: &str,
    ) -> std::result::Result<get_generate_recommendations_status::Response, get_generate_recommendations_status::Error> {
        let http_client = operation_config.http_client();
        let url_str = &format!(
            "{}/subscriptions/{}/providers/Microsoft.Advisor/generateRecommendations/{}",
            operation_config.base_path(),
            subscription_id,
            operation_id
        );
        let mut url = url::Url::parse(url_str).context(get_generate_recommendations_status::ParseUrlError)?;
        let mut req_builder = http::request::Builder::new();
        req_builder = req_builder.method(http::Method::GET);
        if let Some(token_credential) = operation_config.token_credential() {
            let token_response = token_credential
                .get_token(operation_config.token_credential_resource())
                .await
                .context(get_generate_recommendations_status::GetTokenError)?;
            req_builder = req_builder.header(http::header::AUTHORIZATION, format!("Bearer {}", token_response.token.secret()));
        }
        url.query_pairs_mut().append_pair("api-version", operation_config.api_version());
        let req_body = bytes::Bytes::from_static(azure_core::EMPTY_BODY);
        req_builder = req_builder.uri(url.as_str());
        let req = req_builder
            .body(req_body)
            .context(get_generate_recommendations_status::BuildRequestError)?;
        let rsp = http_client
            .execute_request(req)
            .await
            .context(get_generate_recommendations_status::ExecuteRequestError)?;
        match rsp.status() {
            http::StatusCode::ACCEPTED => Ok(get_generate_recommendations_status::Response::Accepted202),
            http::StatusCode::NO_CONTENT => Ok(get_generate_recommendations_status::Response::NoContent204),
            status_code => {
                let rsp_body = rsp.body();
                get_generate_recommendations_status::UnexpectedResponse {
                    status_code,
                    body: rsp_body.clone(),
                }
                .fail()
            }
        }
    }
    pub mod get_generate_recommendations_status {
        use crate::{models, models::*};
        use snafu::Snafu;
        #[derive(Debug)]
        pub enum Response {
            Accepted202,
            NoContent204,
        }
        #[derive(Debug, Snafu)]
        #[snafu(visibility(pub(crate)))]
        pub enum Error {
            UnexpectedResponse { status_code: http::StatusCode, body: bytes::Bytes },
            ParseUrlError { source: url::ParseError },
            BuildRequestError { source: http::Error },
            ExecuteRequestError { source: Box<dyn std::error::Error + Sync + Send> },
            SerializeError { source: Box<dyn std::error::Error + Sync + Send> },
            DeserializeError { source: serde_json::Error, body: bytes::Bytes },
            GetTokenError { source: azure_core::errors::AzureError },
        }
    }
    pub async fn list(
        operation_config: &crate::OperationConfig,
        subscription_id: &str,
        filter: Option<&str>,
        top: Option<i64>,
        skip_token: Option<&str>,
    ) -> std::result::Result<ResourceRecommendationBaseListResult, list::Error> {
        let http_client = operation_config.http_client();
        let url_str = &format!(
            "{}/subscriptions/{}/providers/Microsoft.Advisor/recommendations",
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
        if let Some(filter) = filter {
            url.query_pairs_mut().append_pair("$filter", filter);
        }
        if let Some(top) = top {
            url.query_pairs_mut().append_pair("$top", top.to_string().as_str());
        }
        if let Some(skip_token) = skip_token {
            url.query_pairs_mut().append_pair("$skipToken", skip_token);
        }
        let req_body = bytes::Bytes::from_static(azure_core::EMPTY_BODY);
        req_builder = req_builder.uri(url.as_str());
        let req = req_builder.body(req_body).context(list::BuildRequestError)?;
        let rsp = http_client.execute_request(req).await.context(list::ExecuteRequestError)?;
        match rsp.status() {
            http::StatusCode::OK => {
                let rsp_body = rsp.body();
                let rsp_value: ResourceRecommendationBaseListResult =
                    serde_json::from_slice(rsp_body).context(list::DeserializeError { body: rsp_body.clone() })?;
                Ok(rsp_value)
            }
            status_code => {
                let rsp_body = rsp.body();
                list::UnexpectedResponse {
                    status_code,
                    body: rsp_body.clone(),
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
            UnexpectedResponse { status_code: http::StatusCode, body: bytes::Bytes },
            ParseUrlError { source: url::ParseError },
            BuildRequestError { source: http::Error },
            ExecuteRequestError { source: Box<dyn std::error::Error + Sync + Send> },
            SerializeError { source: Box<dyn std::error::Error + Sync + Send> },
            DeserializeError { source: serde_json::Error, body: bytes::Bytes },
            GetTokenError { source: azure_core::errors::AzureError },
        }
    }
    pub async fn get(
        operation_config: &crate::OperationConfig,
        resource_uri: &str,
        recommendation_id: &str,
    ) -> std::result::Result<ResourceRecommendationBase, get::Error> {
        let http_client = operation_config.http_client();
        let url_str = &format!(
            "{}/{}/providers/Microsoft.Advisor/recommendations/{}",
            operation_config.base_path(),
            resource_uri,
            recommendation_id
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
                let rsp_value: ResourceRecommendationBase =
                    serde_json::from_slice(rsp_body).context(get::DeserializeError { body: rsp_body.clone() })?;
                Ok(rsp_value)
            }
            status_code => {
                let rsp_body = rsp.body();
                get::UnexpectedResponse {
                    status_code,
                    body: rsp_body.clone(),
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
            UnexpectedResponse { status_code: http::StatusCode, body: bytes::Bytes },
            ParseUrlError { source: url::ParseError },
            BuildRequestError { source: http::Error },
            ExecuteRequestError { source: Box<dyn std::error::Error + Sync + Send> },
            SerializeError { source: Box<dyn std::error::Error + Sync + Send> },
            DeserializeError { source: serde_json::Error, body: bytes::Bytes },
            GetTokenError { source: azure_core::errors::AzureError },
        }
    }
}
pub mod operations {
    use crate::models::*;
    use snafu::{ResultExt, Snafu};
    pub async fn list(operation_config: &crate::OperationConfig) -> std::result::Result<OperationEntityListResult, list::Error> {
        let http_client = operation_config.http_client();
        let url_str = &format!("{}/providers/Microsoft.Advisor/operations", operation_config.base_path(),);
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
                let rsp_value: OperationEntityListResult =
                    serde_json::from_slice(rsp_body).context(list::DeserializeError { body: rsp_body.clone() })?;
                Ok(rsp_value)
            }
            status_code => {
                let rsp_body = rsp.body();
                list::UnexpectedResponse {
                    status_code,
                    body: rsp_body.clone(),
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
            UnexpectedResponse { status_code: http::StatusCode, body: bytes::Bytes },
            ParseUrlError { source: url::ParseError },
            BuildRequestError { source: http::Error },
            ExecuteRequestError { source: Box<dyn std::error::Error + Sync + Send> },
            SerializeError { source: Box<dyn std::error::Error + Sync + Send> },
            DeserializeError { source: serde_json::Error, body: bytes::Bytes },
            GetTokenError { source: azure_core::errors::AzureError },
        }
    }
}
pub mod suppressions {
    use crate::models::*;
    use snafu::{ResultExt, Snafu};
    pub async fn get(
        operation_config: &crate::OperationConfig,
        resource_uri: &str,
        recommendation_id: &str,
        name: &str,
    ) -> std::result::Result<SuppressionContract, get::Error> {
        let http_client = operation_config.http_client();
        let url_str = &format!(
            "{}/{}/providers/Microsoft.Advisor/recommendations/{}/suppressions/{}",
            operation_config.base_path(),
            resource_uri,
            recommendation_id,
            name
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
                let rsp_value: SuppressionContract =
                    serde_json::from_slice(rsp_body).context(get::DeserializeError { body: rsp_body.clone() })?;
                Ok(rsp_value)
            }
            status_code => {
                let rsp_body = rsp.body();
                get::UnexpectedResponse {
                    status_code,
                    body: rsp_body.clone(),
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
            UnexpectedResponse { status_code: http::StatusCode, body: bytes::Bytes },
            ParseUrlError { source: url::ParseError },
            BuildRequestError { source: http::Error },
            ExecuteRequestError { source: Box<dyn std::error::Error + Sync + Send> },
            SerializeError { source: Box<dyn std::error::Error + Sync + Send> },
            DeserializeError { source: serde_json::Error, body: bytes::Bytes },
            GetTokenError { source: azure_core::errors::AzureError },
        }
    }
    pub async fn create(
        operation_config: &crate::OperationConfig,
        resource_uri: &str,
        recommendation_id: &str,
        name: &str,
        suppression_contract: &SuppressionContract,
    ) -> std::result::Result<SuppressionContract, create::Error> {
        let http_client = operation_config.http_client();
        let url_str = &format!(
            "{}/{}/providers/Microsoft.Advisor/recommendations/{}/suppressions/{}",
            operation_config.base_path(),
            resource_uri,
            recommendation_id,
            name
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
                let rsp_value: SuppressionContract =
                    serde_json::from_slice(rsp_body).context(create::DeserializeError { body: rsp_body.clone() })?;
                Ok(rsp_value)
            }
            status_code => {
                let rsp_body = rsp.body();
                create::UnexpectedResponse {
                    status_code,
                    body: rsp_body.clone(),
                }
                .fail()
            }
        }
    }
    pub mod create {
        use crate::{models, models::*};
        use snafu::Snafu;
        #[derive(Debug, Snafu)]
        #[snafu(visibility(pub(crate)))]
        pub enum Error {
            UnexpectedResponse { status_code: http::StatusCode, body: bytes::Bytes },
            ParseUrlError { source: url::ParseError },
            BuildRequestError { source: http::Error },
            ExecuteRequestError { source: Box<dyn std::error::Error + Sync + Send> },
            SerializeError { source: Box<dyn std::error::Error + Sync + Send> },
            DeserializeError { source: serde_json::Error, body: bytes::Bytes },
            GetTokenError { source: azure_core::errors::AzureError },
        }
    }
    pub async fn delete(
        operation_config: &crate::OperationConfig,
        resource_uri: &str,
        recommendation_id: &str,
        name: &str,
    ) -> std::result::Result<(), delete::Error> {
        let http_client = operation_config.http_client();
        let url_str = &format!(
            "{}/{}/providers/Microsoft.Advisor/recommendations/{}/suppressions/{}",
            operation_config.base_path(),
            resource_uri,
            recommendation_id,
            name
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
            http::StatusCode::NO_CONTENT => Ok(()),
            status_code => {
                let rsp_body = rsp.body();
                delete::UnexpectedResponse {
                    status_code,
                    body: rsp_body.clone(),
                }
                .fail()
            }
        }
    }
    pub mod delete {
        use crate::{models, models::*};
        use snafu::Snafu;
        #[derive(Debug, Snafu)]
        #[snafu(visibility(pub(crate)))]
        pub enum Error {
            UnexpectedResponse { status_code: http::StatusCode, body: bytes::Bytes },
            ParseUrlError { source: url::ParseError },
            BuildRequestError { source: http::Error },
            ExecuteRequestError { source: Box<dyn std::error::Error + Sync + Send> },
            SerializeError { source: Box<dyn std::error::Error + Sync + Send> },
            DeserializeError { source: serde_json::Error, body: bytes::Bytes },
            GetTokenError { source: azure_core::errors::AzureError },
        }
    }
    pub async fn list(
        operation_config: &crate::OperationConfig,
        subscription_id: &str,
    ) -> std::result::Result<Vec<SuppressionContract>, list::Error> {
        let http_client = operation_config.http_client();
        let url_str = &format!(
            "{}/subscriptions/{}/providers/Microsoft.Advisor/suppressions",
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
                let rsp_value: Vec<SuppressionContract> =
                    serde_json::from_slice(rsp_body).context(list::DeserializeError { body: rsp_body.clone() })?;
                Ok(rsp_value)
            }
            status_code => {
                let rsp_body = rsp.body();
                list::UnexpectedResponse {
                    status_code,
                    body: rsp_body.clone(),
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
            UnexpectedResponse { status_code: http::StatusCode, body: bytes::Bytes },
            ParseUrlError { source: url::ParseError },
            BuildRequestError { source: http::Error },
            ExecuteRequestError { source: Box<dyn std::error::Error + Sync + Send> },
            SerializeError { source: Box<dyn std::error::Error + Sync + Send> },
            DeserializeError { source: serde_json::Error, body: bytes::Bytes },
            GetTokenError { source: azure_core::errors::AzureError },
        }
    }
}
