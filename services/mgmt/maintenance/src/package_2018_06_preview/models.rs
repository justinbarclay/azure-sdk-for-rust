#![doc = "generated by AutoRust 0.1.0"]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApplyUpdateProperties {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<apply_update_properties::Status>,
    #[serde(rename = "resourceId", skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    #[serde(rename = "lastUpdateTime", skip_serializing_if = "Option::is_none")]
    pub last_update_time: Option<String>,
}
mod apply_update_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Status {
        Pending,
        InProgress,
        Completed,
        RetryNow,
        RetryLater,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Resource {
    #[serde(skip_serializing)]
    pub id: Option<String>,
    #[serde(skip_serializing)]
    pub name: Option<String>,
    #[serde(rename = "type", skip_serializing)]
    pub type_: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApplyUpdate {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<ApplyUpdateProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ConfigurationAssignmentProperties {
    #[serde(rename = "maintenanceConfigurationId", skip_serializing_if = "Option::is_none")]
    pub maintenance_configuration_id: Option<String>,
    #[serde(rename = "resourceId", skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ConfigurationAssignment {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<ConfigurationAssignmentProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ListConfigurationAssignmentsResult {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ConfigurationAssignment>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MaintenanceConfigurationProperties {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    #[serde(rename = "extensionProperties", skip_serializing_if = "Option::is_none")]
    pub extension_properties: Option<serde_json::Value>,
    #[serde(rename = "maintenanceScope", skip_serializing_if = "Option::is_none")]
    pub maintenance_scope: Option<maintenance_configuration_properties::MaintenanceScope>,
}
mod maintenance_configuration_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum MaintenanceScope {
        All,
        Host,
        Resource,
        InResource,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MaintenanceConfiguration {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<MaintenanceConfigurationProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MaintenanceError {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<ErrorDetails>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ErrorDetails {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ListMaintenanceConfigurationsResult {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<MaintenanceConfiguration>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OperationsListResult {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Operation>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Operation {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<OperationInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OperationInfo {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ListUpdatesResult {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Update>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Update {
    #[serde(rename = "maintenanceScope", skip_serializing_if = "Option::is_none")]
    pub maintenance_scope: Option<update::MaintenanceScope>,
    #[serde(rename = "impactType", skip_serializing_if = "Option::is_none")]
    pub impact_type: Option<update::ImpactType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<update::Status>,
    #[serde(rename = "impactDurationInSec", skip_serializing_if = "Option::is_none")]
    pub impact_duration_in_sec: Option<i32>,
    #[serde(rename = "notBefore", skip_serializing_if = "Option::is_none")]
    pub not_before: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<UpdateProperties>,
}
mod update {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum MaintenanceScope {
        All,
        Host,
        Resource,
        InResource,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ImpactType {
        None,
        Freeze,
        Restart,
        Redeploy,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Status {
        Pending,
        InProgress,
        Completed,
        RetryNow,
        RetryLater,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UpdateProperties {
    #[serde(rename = "resourceId", skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
}
