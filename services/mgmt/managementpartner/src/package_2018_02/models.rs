#![doc = "generated by AutoRust 0.1.0"]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PartnerResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub etag: Option<i64>,
    #[serde(skip_serializing)]
    pub id: Option<String>,
    #[serde(skip_serializing)]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<PartnerProperties>,
    #[serde(rename = "type", skip_serializing)]
    pub type_: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PartnerProperties {
    #[serde(rename = "partnerId", skip_serializing_if = "Option::is_none")]
    pub partner_id: Option<String>,
    #[serde(rename = "partnerName", skip_serializing_if = "Option::is_none")]
    pub partner_name: Option<String>,
    #[serde(rename = "tenantId", skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    #[serde(rename = "objectId", skip_serializing_if = "Option::is_none")]
    pub object_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<i32>,
    #[serde(rename = "updatedTime", skip_serializing_if = "Option::is_none")]
    pub updated_time: Option<String>,
    #[serde(rename = "createdTime", skip_serializing_if = "Option::is_none")]
    pub created_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<ManagementPartnerState>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ManagementPartnerState {
    Active,
    Deleted,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ErrorResponseCode {
    NotFound,
    Conflict,
    BadRequest,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Error {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<ExtendedErrorInfo>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ExtendedErrorInfo {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<ErrorResponseCode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OperationList {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<OperationResponse>,
    #[serde(rename = "nextLink", skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OperationResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<OperationDisplay>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OperationDisplay {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
