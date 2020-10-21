#![doc = "generated by AutoRust 0.1.0"]
#![allow(unused_mut)]
#![allow(unused_variables)]
#![allow(unused_imports)]
use crate::models::*;
use reqwest::StatusCode;
use snafu::{ResultExt, Snafu};
pub async fn get_user_settings_with_location(
    operation_config: &crate::OperationConfig,
    user_settings_name: &str,
    location: &str,
) -> std::result::Result<UserSettingsResponse, get_user_settings_with_location::Error> {
    let client = &operation_config.client;
    let uri_str = &format!(
        "{}/providers/Microsoft.Portal/locations/{}/userSettings/{}",
        &operation_config.base_path, location, user_settings_name
    );
    let mut req_builder = client.get(uri_str);
    if let Some(token) = &operation_config.bearer_access_token {
        req_builder = req_builder.bearer_auth(token);
    }
    req_builder = req_builder.query(&[("api-version", &operation_config.api_version)]);
    let req = req_builder.build().context(get_user_settings_with_location::BuildRequestError)?;
    let rsp = client
        .execute(req)
        .await
        .context(get_user_settings_with_location::ExecuteRequestError)?;
    match rsp.status() {
        StatusCode::OK => {
            let body: bytes::Bytes = rsp.bytes().await.context(get_user_settings_with_location::ResponseBytesError)?;
            let rsp_value: UserSettingsResponse =
                serde_json::from_slice(&body).context(get_user_settings_with_location::DeserializeError { body })?;
            Ok(rsp_value)
        }
        status_code => {
            let body: bytes::Bytes = rsp.bytes().await.context(get_user_settings_with_location::ResponseBytesError)?;
            let rsp_value: ErrorResponse =
                serde_json::from_slice(&body).context(get_user_settings_with_location::DeserializeError { body })?;
            get_user_settings_with_location::DefaultResponse {
                status_code,
                value: rsp_value,
            }
            .fail()
        }
    }
}
pub mod get_user_settings_with_location {
    use crate::{models, models::*};
    use reqwest::StatusCode;
    use snafu::Snafu;
    #[derive(Debug, Snafu)]
    #[snafu(visibility(pub(crate)))]
    pub enum Error {
        DefaultResponse {
            status_code: StatusCode,
            value: models::ErrorResponse,
        },
        BuildRequestError {
            source: reqwest::Error,
        },
        ExecuteRequestError {
            source: reqwest::Error,
        },
        ResponseBytesError {
            source: reqwest::Error,
        },
        DeserializeError {
            source: serde_json::Error,
            body: bytes::Bytes,
        },
    }
}
pub async fn put_user_settings_with_location(
    operation_config: &crate::OperationConfig,
    user_settings_name: &str,
    location: &str,
    parameters: &CloudShellUserSettings,
) -> std::result::Result<UserSettingsResponse, put_user_settings_with_location::Error> {
    let client = &operation_config.client;
    let uri_str = &format!(
        "{}/providers/Microsoft.Portal/locations/{}/userSettings/{}",
        &operation_config.base_path, location, user_settings_name
    );
    let mut req_builder = client.put(uri_str);
    if let Some(token) = &operation_config.bearer_access_token {
        req_builder = req_builder.bearer_auth(token);
    }
    req_builder = req_builder.query(&[("api-version", &operation_config.api_version)]);
    req_builder = req_builder.json(parameters);
    let req = req_builder.build().context(put_user_settings_with_location::BuildRequestError)?;
    let rsp = client
        .execute(req)
        .await
        .context(put_user_settings_with_location::ExecuteRequestError)?;
    match rsp.status() {
        StatusCode::OK => {
            let body: bytes::Bytes = rsp.bytes().await.context(put_user_settings_with_location::ResponseBytesError)?;
            let rsp_value: UserSettingsResponse =
                serde_json::from_slice(&body).context(put_user_settings_with_location::DeserializeError { body })?;
            Ok(rsp_value)
        }
        status_code => {
            let body: bytes::Bytes = rsp.bytes().await.context(put_user_settings_with_location::ResponseBytesError)?;
            let rsp_value: ErrorResponse =
                serde_json::from_slice(&body).context(put_user_settings_with_location::DeserializeError { body })?;
            put_user_settings_with_location::DefaultResponse {
                status_code,
                value: rsp_value,
            }
            .fail()
        }
    }
}
pub mod put_user_settings_with_location {
    use crate::{models, models::*};
    use reqwest::StatusCode;
    use snafu::Snafu;
    #[derive(Debug, Snafu)]
    #[snafu(visibility(pub(crate)))]
    pub enum Error {
        DefaultResponse {
            status_code: StatusCode,
            value: models::ErrorResponse,
        },
        BuildRequestError {
            source: reqwest::Error,
        },
        ExecuteRequestError {
            source: reqwest::Error,
        },
        ResponseBytesError {
            source: reqwest::Error,
        },
        DeserializeError {
            source: serde_json::Error,
            body: bytes::Bytes,
        },
    }
}
pub async fn patch_user_settings_with_location(
    operation_config: &crate::OperationConfig,
    user_settings_name: &str,
    location: &str,
    parameters: &CloudShellPatchUserSettings,
) -> std::result::Result<UserSettingsResponse, patch_user_settings_with_location::Error> {
    let client = &operation_config.client;
    let uri_str = &format!(
        "{}/providers/Microsoft.Portal/locations/{}/userSettings/{}",
        &operation_config.base_path, location, user_settings_name
    );
    let mut req_builder = client.patch(uri_str);
    if let Some(token) = &operation_config.bearer_access_token {
        req_builder = req_builder.bearer_auth(token);
    }
    req_builder = req_builder.query(&[("api-version", &operation_config.api_version)]);
    req_builder = req_builder.json(parameters);
    let req = req_builder.build().context(patch_user_settings_with_location::BuildRequestError)?;
    let rsp = client
        .execute(req)
        .await
        .context(patch_user_settings_with_location::ExecuteRequestError)?;
    match rsp.status() {
        StatusCode::OK => {
            let body: bytes::Bytes = rsp.bytes().await.context(patch_user_settings_with_location::ResponseBytesError)?;
            let rsp_value: UserSettingsResponse =
                serde_json::from_slice(&body).context(patch_user_settings_with_location::DeserializeError { body })?;
            Ok(rsp_value)
        }
        status_code => {
            let body: bytes::Bytes = rsp.bytes().await.context(patch_user_settings_with_location::ResponseBytesError)?;
            let rsp_value: ErrorResponse =
                serde_json::from_slice(&body).context(patch_user_settings_with_location::DeserializeError { body })?;
            patch_user_settings_with_location::DefaultResponse {
                status_code,
                value: rsp_value,
            }
            .fail()
        }
    }
}
pub mod patch_user_settings_with_location {
    use crate::{models, models::*};
    use reqwest::StatusCode;
    use snafu::Snafu;
    #[derive(Debug, Snafu)]
    #[snafu(visibility(pub(crate)))]
    pub enum Error {
        DefaultResponse {
            status_code: StatusCode,
            value: models::ErrorResponse,
        },
        BuildRequestError {
            source: reqwest::Error,
        },
        ExecuteRequestError {
            source: reqwest::Error,
        },
        ResponseBytesError {
            source: reqwest::Error,
        },
        DeserializeError {
            source: serde_json::Error,
            body: bytes::Bytes,
        },
    }
}
pub async fn delete_user_settings_with_location(
    operation_config: &crate::OperationConfig,
    user_settings_name: &str,
    location: &str,
) -> std::result::Result<delete_user_settings_with_location::Response, delete_user_settings_with_location::Error> {
    let client = &operation_config.client;
    let uri_str = &format!(
        "{}/providers/Microsoft.Portal/locations/{}/userSettings/{}",
        &operation_config.base_path, location, user_settings_name
    );
    let mut req_builder = client.delete(uri_str);
    if let Some(token) = &operation_config.bearer_access_token {
        req_builder = req_builder.bearer_auth(token);
    }
    req_builder = req_builder.query(&[("api-version", &operation_config.api_version)]);
    let req = req_builder.build().context(delete_user_settings_with_location::BuildRequestError)?;
    let rsp = client
        .execute(req)
        .await
        .context(delete_user_settings_with_location::ExecuteRequestError)?;
    match rsp.status() {
        StatusCode::OK => Ok(delete_user_settings_with_location::Response::Ok200),
        StatusCode::NO_CONTENT => Ok(delete_user_settings_with_location::Response::NoContent204),
        status_code => {
            let body: bytes::Bytes = rsp.bytes().await.context(delete_user_settings_with_location::ResponseBytesError)?;
            let rsp_value: ErrorResponse =
                serde_json::from_slice(&body).context(delete_user_settings_with_location::DeserializeError { body })?;
            delete_user_settings_with_location::DefaultResponse {
                status_code,
                value: rsp_value,
            }
            .fail()
        }
    }
}
pub mod delete_user_settings_with_location {
    use crate::{models, models::*};
    use reqwest::StatusCode;
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
            status_code: StatusCode,
            value: models::ErrorResponse,
        },
        BuildRequestError {
            source: reqwest::Error,
        },
        ExecuteRequestError {
            source: reqwest::Error,
        },
        ResponseBytesError {
            source: reqwest::Error,
        },
        DeserializeError {
            source: serde_json::Error,
            body: bytes::Bytes,
        },
    }
}
pub async fn get_console_with_location(
    operation_config: &crate::OperationConfig,
    console_name: &str,
    location: &str,
) -> std::result::Result<CloudShellConsole, get_console_with_location::Error> {
    let client = &operation_config.client;
    let uri_str = &format!(
        "{}/providers/Microsoft.Portal/locations/{}/consoles/{}",
        &operation_config.base_path, location, console_name
    );
    let mut req_builder = client.get(uri_str);
    if let Some(token) = &operation_config.bearer_access_token {
        req_builder = req_builder.bearer_auth(token);
    }
    req_builder = req_builder.query(&[("api-version", &operation_config.api_version)]);
    let req = req_builder.build().context(get_console_with_location::BuildRequestError)?;
    let rsp = client.execute(req).await.context(get_console_with_location::ExecuteRequestError)?;
    match rsp.status() {
        StatusCode::OK => {
            let body: bytes::Bytes = rsp.bytes().await.context(get_console_with_location::ResponseBytesError)?;
            let rsp_value: CloudShellConsole =
                serde_json::from_slice(&body).context(get_console_with_location::DeserializeError { body })?;
            Ok(rsp_value)
        }
        status_code => {
            let body: bytes::Bytes = rsp.bytes().await.context(get_console_with_location::ResponseBytesError)?;
            let rsp_value: ErrorResponse = serde_json::from_slice(&body).context(get_console_with_location::DeserializeError { body })?;
            get_console_with_location::DefaultResponse {
                status_code,
                value: rsp_value,
            }
            .fail()
        }
    }
}
pub mod get_console_with_location {
    use crate::{models, models::*};
    use reqwest::StatusCode;
    use snafu::Snafu;
    #[derive(Debug, Snafu)]
    #[snafu(visibility(pub(crate)))]
    pub enum Error {
        DefaultResponse {
            status_code: StatusCode,
            value: models::ErrorResponse,
        },
        BuildRequestError {
            source: reqwest::Error,
        },
        ExecuteRequestError {
            source: reqwest::Error,
        },
        ResponseBytesError {
            source: reqwest::Error,
        },
        DeserializeError {
            source: serde_json::Error,
            body: bytes::Bytes,
        },
    }
}
pub async fn put_console_with_location(
    operation_config: &crate::OperationConfig,
    console_name: &str,
    location: &str,
) -> std::result::Result<put_console_with_location::Response, put_console_with_location::Error> {
    let client = &operation_config.client;
    let uri_str = &format!(
        "{}/providers/Microsoft.Portal/locations/{}/consoles/{}",
        &operation_config.base_path, location, console_name
    );
    let mut req_builder = client.put(uri_str);
    if let Some(token) = &operation_config.bearer_access_token {
        req_builder = req_builder.bearer_auth(token);
    }
    req_builder = req_builder.query(&[("api-version", &operation_config.api_version)]);
    let req = req_builder.build().context(put_console_with_location::BuildRequestError)?;
    let rsp = client.execute(req).await.context(put_console_with_location::ExecuteRequestError)?;
    match rsp.status() {
        StatusCode::OK => {
            let body: bytes::Bytes = rsp.bytes().await.context(put_console_with_location::ResponseBytesError)?;
            let rsp_value: CloudShellConsole =
                serde_json::from_slice(&body).context(put_console_with_location::DeserializeError { body })?;
            Ok(put_console_with_location::Response::Ok200(rsp_value))
        }
        StatusCode::CREATED => {
            let body: bytes::Bytes = rsp.bytes().await.context(put_console_with_location::ResponseBytesError)?;
            let rsp_value: CloudShellConsole =
                serde_json::from_slice(&body).context(put_console_with_location::DeserializeError { body })?;
            Ok(put_console_with_location::Response::Created201(rsp_value))
        }
        status_code => {
            let body: bytes::Bytes = rsp.bytes().await.context(put_console_with_location::ResponseBytesError)?;
            let rsp_value: ErrorResponse = serde_json::from_slice(&body).context(put_console_with_location::DeserializeError { body })?;
            put_console_with_location::DefaultResponse {
                status_code,
                value: rsp_value,
            }
            .fail()
        }
    }
}
pub mod put_console_with_location {
    use crate::{models, models::*};
    use reqwest::StatusCode;
    use snafu::Snafu;
    #[derive(Debug)]
    pub enum Response {
        Ok200(CloudShellConsole),
        Created201(CloudShellConsole),
    }
    #[derive(Debug, Snafu)]
    #[snafu(visibility(pub(crate)))]
    pub enum Error {
        DefaultResponse {
            status_code: StatusCode,
            value: models::ErrorResponse,
        },
        BuildRequestError {
            source: reqwest::Error,
        },
        ExecuteRequestError {
            source: reqwest::Error,
        },
        ResponseBytesError {
            source: reqwest::Error,
        },
        DeserializeError {
            source: serde_json::Error,
            body: bytes::Bytes,
        },
    }
}
pub async fn delete_console_with_location(
    operation_config: &crate::OperationConfig,
    console_name: &str,
    location: &str,
) -> std::result::Result<delete_console_with_location::Response, delete_console_with_location::Error> {
    let client = &operation_config.client;
    let uri_str = &format!(
        "{}/providers/Microsoft.Portal/locations/{}/consoles/{}",
        &operation_config.base_path, location, console_name
    );
    let mut req_builder = client.delete(uri_str);
    if let Some(token) = &operation_config.bearer_access_token {
        req_builder = req_builder.bearer_auth(token);
    }
    req_builder = req_builder.query(&[("api-version", &operation_config.api_version)]);
    let req = req_builder.build().context(delete_console_with_location::BuildRequestError)?;
    let rsp = client
        .execute(req)
        .await
        .context(delete_console_with_location::ExecuteRequestError)?;
    match rsp.status() {
        StatusCode::OK => Ok(delete_console_with_location::Response::Ok200),
        StatusCode::NO_CONTENT => Ok(delete_console_with_location::Response::NoContent204),
        status_code => {
            let body: bytes::Bytes = rsp.bytes().await.context(delete_console_with_location::ResponseBytesError)?;
            let rsp_value: ErrorResponse =
                serde_json::from_slice(&body).context(delete_console_with_location::DeserializeError { body })?;
            delete_console_with_location::DefaultResponse {
                status_code,
                value: rsp_value,
            }
            .fail()
        }
    }
}
pub mod delete_console_with_location {
    use crate::{models, models::*};
    use reqwest::StatusCode;
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
            status_code: StatusCode,
            value: models::ErrorResponse,
        },
        BuildRequestError {
            source: reqwest::Error,
        },
        ExecuteRequestError {
            source: reqwest::Error,
        },
        ResponseBytesError {
            source: reqwest::Error,
        },
        DeserializeError {
            source: serde_json::Error,
            body: bytes::Bytes,
        },
    }
}
pub async fn keep_alive_with_location(
    operation_config: &crate::OperationConfig,
    console_name: &str,
    location: &str,
) -> std::result::Result<(), keep_alive_with_location::Error> {
    let client = &operation_config.client;
    let uri_str = &format!(
        "{}/providers/Microsoft.Portal/locations/{}/consoles/{}/keepAlive",
        &operation_config.base_path, location, console_name
    );
    let mut req_builder = client.post(uri_str);
    if let Some(token) = &operation_config.bearer_access_token {
        req_builder = req_builder.bearer_auth(token);
    }
    let req = req_builder.build().context(keep_alive_with_location::BuildRequestError)?;
    let rsp = client.execute(req).await.context(keep_alive_with_location::ExecuteRequestError)?;
    match rsp.status() {
        StatusCode::OK => Ok(()),
        status_code => {
            let body: bytes::Bytes = rsp.bytes().await.context(keep_alive_with_location::ResponseBytesError)?;
            let rsp_value: ErrorResponse = serde_json::from_slice(&body).context(keep_alive_with_location::DeserializeError { body })?;
            keep_alive_with_location::DefaultResponse {
                status_code,
                value: rsp_value,
            }
            .fail()
        }
    }
}
pub mod keep_alive_with_location {
    use crate::{models, models::*};
    use reqwest::StatusCode;
    use snafu::Snafu;
    #[derive(Debug, Snafu)]
    #[snafu(visibility(pub(crate)))]
    pub enum Error {
        DefaultResponse {
            status_code: StatusCode,
            value: models::ErrorResponse,
        },
        BuildRequestError {
            source: reqwest::Error,
        },
        ExecuteRequestError {
            source: reqwest::Error,
        },
        ResponseBytesError {
            source: reqwest::Error,
        },
        DeserializeError {
            source: serde_json::Error,
            body: bytes::Bytes,
        },
    }
}
pub async fn get_user_settings(
    operation_config: &crate::OperationConfig,
    user_settings_name: &str,
) -> std::result::Result<UserSettingsResponse, get_user_settings::Error> {
    let client = &operation_config.client;
    let uri_str = &format!(
        "{}/providers/Microsoft.Portal/userSettings/{}",
        &operation_config.base_path, user_settings_name
    );
    let mut req_builder = client.get(uri_str);
    if let Some(token) = &operation_config.bearer_access_token {
        req_builder = req_builder.bearer_auth(token);
    }
    req_builder = req_builder.query(&[("api-version", &operation_config.api_version)]);
    let req = req_builder.build().context(get_user_settings::BuildRequestError)?;
    let rsp = client.execute(req).await.context(get_user_settings::ExecuteRequestError)?;
    match rsp.status() {
        StatusCode::OK => {
            let body: bytes::Bytes = rsp.bytes().await.context(get_user_settings::ResponseBytesError)?;
            let rsp_value: UserSettingsResponse = serde_json::from_slice(&body).context(get_user_settings::DeserializeError { body })?;
            Ok(rsp_value)
        }
        status_code => {
            let body: bytes::Bytes = rsp.bytes().await.context(get_user_settings::ResponseBytesError)?;
            let rsp_value: ErrorResponse = serde_json::from_slice(&body).context(get_user_settings::DeserializeError { body })?;
            get_user_settings::DefaultResponse {
                status_code,
                value: rsp_value,
            }
            .fail()
        }
    }
}
pub mod get_user_settings {
    use crate::{models, models::*};
    use reqwest::StatusCode;
    use snafu::Snafu;
    #[derive(Debug, Snafu)]
    #[snafu(visibility(pub(crate)))]
    pub enum Error {
        DefaultResponse {
            status_code: StatusCode,
            value: models::ErrorResponse,
        },
        BuildRequestError {
            source: reqwest::Error,
        },
        ExecuteRequestError {
            source: reqwest::Error,
        },
        ResponseBytesError {
            source: reqwest::Error,
        },
        DeserializeError {
            source: serde_json::Error,
            body: bytes::Bytes,
        },
    }
}
pub async fn put_user_settings(
    operation_config: &crate::OperationConfig,
    user_settings_name: &str,
    parameters: &CloudShellUserSettings,
) -> std::result::Result<UserSettingsResponse, put_user_settings::Error> {
    let client = &operation_config.client;
    let uri_str = &format!(
        "{}/providers/Microsoft.Portal/userSettings/{}",
        &operation_config.base_path, user_settings_name
    );
    let mut req_builder = client.put(uri_str);
    if let Some(token) = &operation_config.bearer_access_token {
        req_builder = req_builder.bearer_auth(token);
    }
    req_builder = req_builder.query(&[("api-version", &operation_config.api_version)]);
    req_builder = req_builder.json(parameters);
    let req = req_builder.build().context(put_user_settings::BuildRequestError)?;
    let rsp = client.execute(req).await.context(put_user_settings::ExecuteRequestError)?;
    match rsp.status() {
        StatusCode::OK => {
            let body: bytes::Bytes = rsp.bytes().await.context(put_user_settings::ResponseBytesError)?;
            let rsp_value: UserSettingsResponse = serde_json::from_slice(&body).context(put_user_settings::DeserializeError { body })?;
            Ok(rsp_value)
        }
        status_code => {
            let body: bytes::Bytes = rsp.bytes().await.context(put_user_settings::ResponseBytesError)?;
            let rsp_value: ErrorResponse = serde_json::from_slice(&body).context(put_user_settings::DeserializeError { body })?;
            put_user_settings::DefaultResponse {
                status_code,
                value: rsp_value,
            }
            .fail()
        }
    }
}
pub mod put_user_settings {
    use crate::{models, models::*};
    use reqwest::StatusCode;
    use snafu::Snafu;
    #[derive(Debug, Snafu)]
    #[snafu(visibility(pub(crate)))]
    pub enum Error {
        DefaultResponse {
            status_code: StatusCode,
            value: models::ErrorResponse,
        },
        BuildRequestError {
            source: reqwest::Error,
        },
        ExecuteRequestError {
            source: reqwest::Error,
        },
        ResponseBytesError {
            source: reqwest::Error,
        },
        DeserializeError {
            source: serde_json::Error,
            body: bytes::Bytes,
        },
    }
}
pub async fn patch_user_settings(
    operation_config: &crate::OperationConfig,
    user_settings_name: &str,
    parameters: &CloudShellPatchUserSettings,
) -> std::result::Result<UserSettingsResponse, patch_user_settings::Error> {
    let client = &operation_config.client;
    let uri_str = &format!(
        "{}/providers/Microsoft.Portal/userSettings/{}",
        &operation_config.base_path, user_settings_name
    );
    let mut req_builder = client.patch(uri_str);
    if let Some(token) = &operation_config.bearer_access_token {
        req_builder = req_builder.bearer_auth(token);
    }
    req_builder = req_builder.query(&[("api-version", &operation_config.api_version)]);
    req_builder = req_builder.json(parameters);
    let req = req_builder.build().context(patch_user_settings::BuildRequestError)?;
    let rsp = client.execute(req).await.context(patch_user_settings::ExecuteRequestError)?;
    match rsp.status() {
        StatusCode::OK => {
            let body: bytes::Bytes = rsp.bytes().await.context(patch_user_settings::ResponseBytesError)?;
            let rsp_value: UserSettingsResponse = serde_json::from_slice(&body).context(patch_user_settings::DeserializeError { body })?;
            Ok(rsp_value)
        }
        status_code => {
            let body: bytes::Bytes = rsp.bytes().await.context(patch_user_settings::ResponseBytesError)?;
            let rsp_value: ErrorResponse = serde_json::from_slice(&body).context(patch_user_settings::DeserializeError { body })?;
            patch_user_settings::DefaultResponse {
                status_code,
                value: rsp_value,
            }
            .fail()
        }
    }
}
pub mod patch_user_settings {
    use crate::{models, models::*};
    use reqwest::StatusCode;
    use snafu::Snafu;
    #[derive(Debug, Snafu)]
    #[snafu(visibility(pub(crate)))]
    pub enum Error {
        DefaultResponse {
            status_code: StatusCode,
            value: models::ErrorResponse,
        },
        BuildRequestError {
            source: reqwest::Error,
        },
        ExecuteRequestError {
            source: reqwest::Error,
        },
        ResponseBytesError {
            source: reqwest::Error,
        },
        DeserializeError {
            source: serde_json::Error,
            body: bytes::Bytes,
        },
    }
}
pub async fn delete_user_settings(
    operation_config: &crate::OperationConfig,
    user_settings_name: &str,
) -> std::result::Result<delete_user_settings::Response, delete_user_settings::Error> {
    let client = &operation_config.client;
    let uri_str = &format!(
        "{}/providers/Microsoft.Portal/userSettings/{}",
        &operation_config.base_path, user_settings_name
    );
    let mut req_builder = client.delete(uri_str);
    if let Some(token) = &operation_config.bearer_access_token {
        req_builder = req_builder.bearer_auth(token);
    }
    req_builder = req_builder.query(&[("api-version", &operation_config.api_version)]);
    let req = req_builder.build().context(delete_user_settings::BuildRequestError)?;
    let rsp = client.execute(req).await.context(delete_user_settings::ExecuteRequestError)?;
    match rsp.status() {
        StatusCode::OK => Ok(delete_user_settings::Response::Ok200),
        StatusCode::NO_CONTENT => Ok(delete_user_settings::Response::NoContent204),
        status_code => {
            let body: bytes::Bytes = rsp.bytes().await.context(delete_user_settings::ResponseBytesError)?;
            let rsp_value: ErrorResponse = serde_json::from_slice(&body).context(delete_user_settings::DeserializeError { body })?;
            delete_user_settings::DefaultResponse {
                status_code,
                value: rsp_value,
            }
            .fail()
        }
    }
}
pub mod delete_user_settings {
    use crate::{models, models::*};
    use reqwest::StatusCode;
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
            status_code: StatusCode,
            value: models::ErrorResponse,
        },
        BuildRequestError {
            source: reqwest::Error,
        },
        ExecuteRequestError {
            source: reqwest::Error,
        },
        ResponseBytesError {
            source: reqwest::Error,
        },
        DeserializeError {
            source: serde_json::Error,
            body: bytes::Bytes,
        },
    }
}
pub async fn get_console(
    operation_config: &crate::OperationConfig,
    console_name: &str,
) -> std::result::Result<CloudShellConsole, get_console::Error> {
    let client = &operation_config.client;
    let uri_str = &format!(
        "{}/providers/Microsoft.Portal/consoles/{}",
        &operation_config.base_path, console_name
    );
    let mut req_builder = client.get(uri_str);
    if let Some(token) = &operation_config.bearer_access_token {
        req_builder = req_builder.bearer_auth(token);
    }
    req_builder = req_builder.query(&[("api-version", &operation_config.api_version)]);
    let req = req_builder.build().context(get_console::BuildRequestError)?;
    let rsp = client.execute(req).await.context(get_console::ExecuteRequestError)?;
    match rsp.status() {
        StatusCode::OK => {
            let body: bytes::Bytes = rsp.bytes().await.context(get_console::ResponseBytesError)?;
            let rsp_value: CloudShellConsole = serde_json::from_slice(&body).context(get_console::DeserializeError { body })?;
            Ok(rsp_value)
        }
        status_code => {
            let body: bytes::Bytes = rsp.bytes().await.context(get_console::ResponseBytesError)?;
            let rsp_value: ErrorResponse = serde_json::from_slice(&body).context(get_console::DeserializeError { body })?;
            get_console::DefaultResponse {
                status_code,
                value: rsp_value,
            }
            .fail()
        }
    }
}
pub mod get_console {
    use crate::{models, models::*};
    use reqwest::StatusCode;
    use snafu::Snafu;
    #[derive(Debug, Snafu)]
    #[snafu(visibility(pub(crate)))]
    pub enum Error {
        DefaultResponse {
            status_code: StatusCode,
            value: models::ErrorResponse,
        },
        BuildRequestError {
            source: reqwest::Error,
        },
        ExecuteRequestError {
            source: reqwest::Error,
        },
        ResponseBytesError {
            source: reqwest::Error,
        },
        DeserializeError {
            source: serde_json::Error,
            body: bytes::Bytes,
        },
    }
}
pub async fn put_console(
    operation_config: &crate::OperationConfig,
    console_name: &str,
    parameters: &ConsoleDefinition,
) -> std::result::Result<put_console::Response, put_console::Error> {
    let client = &operation_config.client;
    let uri_str = &format!(
        "{}/providers/Microsoft.Portal/consoles/{}",
        &operation_config.base_path, console_name
    );
    let mut req_builder = client.put(uri_str);
    if let Some(token) = &operation_config.bearer_access_token {
        req_builder = req_builder.bearer_auth(token);
    }
    req_builder = req_builder.query(&[("api-version", &operation_config.api_version)]);
    req_builder = req_builder.json(parameters);
    let req = req_builder.build().context(put_console::BuildRequestError)?;
    let rsp = client.execute(req).await.context(put_console::ExecuteRequestError)?;
    match rsp.status() {
        StatusCode::OK => {
            let body: bytes::Bytes = rsp.bytes().await.context(put_console::ResponseBytesError)?;
            let rsp_value: CloudShellConsole = serde_json::from_slice(&body).context(put_console::DeserializeError { body })?;
            Ok(put_console::Response::Ok200(rsp_value))
        }
        StatusCode::CREATED => {
            let body: bytes::Bytes = rsp.bytes().await.context(put_console::ResponseBytesError)?;
            let rsp_value: CloudShellConsole = serde_json::from_slice(&body).context(put_console::DeserializeError { body })?;
            Ok(put_console::Response::Created201(rsp_value))
        }
        status_code => {
            let body: bytes::Bytes = rsp.bytes().await.context(put_console::ResponseBytesError)?;
            let rsp_value: ErrorResponse = serde_json::from_slice(&body).context(put_console::DeserializeError { body })?;
            put_console::DefaultResponse {
                status_code,
                value: rsp_value,
            }
            .fail()
        }
    }
}
pub mod put_console {
    use crate::{models, models::*};
    use reqwest::StatusCode;
    use snafu::Snafu;
    #[derive(Debug)]
    pub enum Response {
        Ok200(CloudShellConsole),
        Created201(CloudShellConsole),
    }
    #[derive(Debug, Snafu)]
    #[snafu(visibility(pub(crate)))]
    pub enum Error {
        DefaultResponse {
            status_code: StatusCode,
            value: models::ErrorResponse,
        },
        BuildRequestError {
            source: reqwest::Error,
        },
        ExecuteRequestError {
            source: reqwest::Error,
        },
        ResponseBytesError {
            source: reqwest::Error,
        },
        DeserializeError {
            source: serde_json::Error,
            body: bytes::Bytes,
        },
    }
}
pub async fn delete_console(
    operation_config: &crate::OperationConfig,
    console_name: &str,
) -> std::result::Result<delete_console::Response, delete_console::Error> {
    let client = &operation_config.client;
    let uri_str = &format!(
        "{}/providers/Microsoft.Portal/consoles/{}",
        &operation_config.base_path, console_name
    );
    let mut req_builder = client.delete(uri_str);
    if let Some(token) = &operation_config.bearer_access_token {
        req_builder = req_builder.bearer_auth(token);
    }
    req_builder = req_builder.query(&[("api-version", &operation_config.api_version)]);
    let req = req_builder.build().context(delete_console::BuildRequestError)?;
    let rsp = client.execute(req).await.context(delete_console::ExecuteRequestError)?;
    match rsp.status() {
        StatusCode::OK => Ok(delete_console::Response::Ok200),
        StatusCode::NO_CONTENT => Ok(delete_console::Response::NoContent204),
        status_code => {
            let body: bytes::Bytes = rsp.bytes().await.context(delete_console::ResponseBytesError)?;
            let rsp_value: ErrorResponse = serde_json::from_slice(&body).context(delete_console::DeserializeError { body })?;
            delete_console::DefaultResponse {
                status_code,
                value: rsp_value,
            }
            .fail()
        }
    }
}
pub mod delete_console {
    use crate::{models, models::*};
    use reqwest::StatusCode;
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
            status_code: StatusCode,
            value: models::ErrorResponse,
        },
        BuildRequestError {
            source: reqwest::Error,
        },
        ExecuteRequestError {
            source: reqwest::Error,
        },
        ResponseBytesError {
            source: reqwest::Error,
        },
        DeserializeError {
            source: serde_json::Error,
            body: bytes::Bytes,
        },
    }
}
pub async fn keep_alive(operation_config: &crate::OperationConfig, console_name: &str) -> std::result::Result<(), keep_alive::Error> {
    let client = &operation_config.client;
    let uri_str = &format!(
        "{}/providers/Microsoft.Portal/consoles/{}/keepAlive",
        &operation_config.base_path, console_name
    );
    let mut req_builder = client.post(uri_str);
    if let Some(token) = &operation_config.bearer_access_token {
        req_builder = req_builder.bearer_auth(token);
    }
    let req = req_builder.build().context(keep_alive::BuildRequestError)?;
    let rsp = client.execute(req).await.context(keep_alive::ExecuteRequestError)?;
    match rsp.status() {
        StatusCode::OK => Ok(()),
        status_code => {
            let body: bytes::Bytes = rsp.bytes().await.context(keep_alive::ResponseBytesError)?;
            let rsp_value: ErrorResponse = serde_json::from_slice(&body).context(keep_alive::DeserializeError { body })?;
            keep_alive::DefaultResponse {
                status_code,
                value: rsp_value,
            }
            .fail()
        }
    }
}
pub mod keep_alive {
    use crate::{models, models::*};
    use reqwest::StatusCode;
    use snafu::Snafu;
    #[derive(Debug, Snafu)]
    #[snafu(visibility(pub(crate)))]
    pub enum Error {
        DefaultResponse {
            status_code: StatusCode,
            value: models::ErrorResponse,
        },
        BuildRequestError {
            source: reqwest::Error,
        },
        ExecuteRequestError {
            source: reqwest::Error,
        },
        ResponseBytesError {
            source: reqwest::Error,
        },
        DeserializeError {
            source: serde_json::Error,
            body: bytes::Bytes,
        },
    }
}
