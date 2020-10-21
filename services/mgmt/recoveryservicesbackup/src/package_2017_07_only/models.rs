#![doc = "generated by AutoRust 0.1.0"]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureBackupGoalFeatureSupportRequest {
    #[serde(flatten)]
    pub feature_support_request: FeatureSupportRequest,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureIaaSvmErrorInfo {
    #[serde(rename = "errorCode", skip_serializing)]
    pub error_code: Option<i32>,
    #[serde(rename = "errorTitle", skip_serializing)]
    pub error_title: Option<String>,
    #[serde(rename = "errorString", skip_serializing)]
    pub error_string: Option<String>,
    #[serde(skip_serializing)]
    pub recommendations: Vec<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureIaaSvmJob {
    #[serde(flatten)]
    pub job: Job,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<String>,
    #[serde(rename = "actionsInfo", skip_serializing_if = "Vec::is_empty")]
    pub actions_info: Vec<String>,
    #[serde(rename = "errorDetails", skip_serializing_if = "Vec::is_empty")]
    pub error_details: Vec<AzureIaaSvmErrorInfo>,
    #[serde(rename = "virtualMachineVersion", skip_serializing_if = "Option::is_none")]
    pub virtual_machine_version: Option<String>,
    #[serde(rename = "extendedInfo", skip_serializing_if = "Option::is_none")]
    pub extended_info: Option<AzureIaaSvmJobExtendedInfo>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureIaaSvmJobExtendedInfo {
    #[serde(rename = "tasksList", skip_serializing_if = "Vec::is_empty")]
    pub tasks_list: Vec<AzureIaaSvmJobTaskDetails>,
    #[serde(rename = "propertyBag", skip_serializing_if = "Option::is_none")]
    pub property_bag: Option<serde_json::Value>,
    #[serde(rename = "internalPropertyBag", skip_serializing_if = "Option::is_none")]
    pub internal_property_bag: Option<serde_json::Value>,
    #[serde(rename = "progressPercentage", skip_serializing_if = "Option::is_none")]
    pub progress_percentage: Option<f64>,
    #[serde(rename = "estimatedRemainingDuration", skip_serializing_if = "Option::is_none")]
    pub estimated_remaining_duration: Option<String>,
    #[serde(rename = "dynamicErrorMessage", skip_serializing_if = "Option::is_none")]
    pub dynamic_error_message: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureIaaSvmJobTaskDetails {
    #[serde(rename = "taskId", skip_serializing_if = "Option::is_none")]
    pub task_id: Option<String>,
    #[serde(rename = "startTime", skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    #[serde(rename = "endTime", skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    #[serde(rename = "instanceId", skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "progressPercentage", skip_serializing_if = "Option::is_none")]
    pub progress_percentage: Option<f64>,
    #[serde(rename = "taskExecutionDetails", skip_serializing_if = "Option::is_none")]
    pub task_execution_details: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureRecoveryServiceVaultProtectionIntent {
    #[serde(flatten)]
    pub protection_intent: ProtectionIntent,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureResourceProtectionIntent {
    #[serde(flatten)]
    pub protection_intent: ProtectionIntent,
    #[serde(rename = "friendlyName", skip_serializing_if = "Option::is_none")]
    pub friendly_name: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureStorageErrorInfo {
    #[serde(rename = "errorCode", skip_serializing_if = "Option::is_none")]
    pub error_code: Option<i32>,
    #[serde(rename = "errorString", skip_serializing_if = "Option::is_none")]
    pub error_string: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub recommendations: Vec<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureStorageJob {
    #[serde(flatten)]
    pub job: Job,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<String>,
    #[serde(rename = "actionsInfo", skip_serializing_if = "Vec::is_empty")]
    pub actions_info: Vec<String>,
    #[serde(rename = "errorDetails", skip_serializing_if = "Vec::is_empty")]
    pub error_details: Vec<AzureStorageErrorInfo>,
    #[serde(rename = "storageAccountName", skip_serializing_if = "Option::is_none")]
    pub storage_account_name: Option<String>,
    #[serde(rename = "storageAccountVersion", skip_serializing_if = "Option::is_none")]
    pub storage_account_version: Option<String>,
    #[serde(rename = "extendedInfo", skip_serializing_if = "Option::is_none")]
    pub extended_info: Option<AzureStorageJobExtendedInfo>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureStorageJobExtendedInfo {
    #[serde(rename = "tasksList", skip_serializing_if = "Vec::is_empty")]
    pub tasks_list: Vec<AzureStorageJobTaskDetails>,
    #[serde(rename = "propertyBag", skip_serializing_if = "Option::is_none")]
    pub property_bag: Option<serde_json::Value>,
    #[serde(rename = "dynamicErrorMessage", skip_serializing_if = "Option::is_none")]
    pub dynamic_error_message: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureStorageJobTaskDetails {
    #[serde(rename = "taskId", skip_serializing_if = "Option::is_none")]
    pub task_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureVmResourceFeatureSupportRequest {
    #[serde(flatten)]
    pub feature_support_request: FeatureSupportRequest,
    #[serde(rename = "vmSize", skip_serializing_if = "Option::is_none")]
    pub vm_size: Option<String>,
    #[serde(rename = "vmSku", skip_serializing_if = "Option::is_none")]
    pub vm_sku: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureVmResourceFeatureSupportResponse {
    #[serde(rename = "supportStatus", skip_serializing_if = "Option::is_none")]
    pub support_status: Option<azure_vm_resource_feature_support_response::SupportStatus>,
}
mod azure_vm_resource_feature_support_response {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum SupportStatus {
        Invalid,
        Supported,
        #[serde(rename = "DefaultOFF")]
        DefaultOff,
        #[serde(rename = "DefaultON")]
        DefaultOn,
        NotSupported,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureWorkloadAutoProtectionIntent {
    #[serde(flatten)]
    pub azure_recovery_service_vault_protection_intent: AzureRecoveryServiceVaultProtectionIntent,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureWorkloadErrorInfo {
    #[serde(rename = "errorCode", skip_serializing_if = "Option::is_none")]
    pub error_code: Option<i32>,
    #[serde(rename = "errorString", skip_serializing_if = "Option::is_none")]
    pub error_string: Option<String>,
    #[serde(rename = "errorTitle", skip_serializing_if = "Option::is_none")]
    pub error_title: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub recommendations: Vec<String>,
    #[serde(rename = "additionalDetails", skip_serializing_if = "Option::is_none")]
    pub additional_details: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureWorkloadJob {
    #[serde(flatten)]
    pub job: Job,
    #[serde(rename = "workloadType", skip_serializing_if = "Option::is_none")]
    pub workload_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<String>,
    #[serde(rename = "actionsInfo", skip_serializing_if = "Vec::is_empty")]
    pub actions_info: Vec<String>,
    #[serde(rename = "errorDetails", skip_serializing_if = "Vec::is_empty")]
    pub error_details: Vec<AzureWorkloadErrorInfo>,
    #[serde(rename = "extendedInfo", skip_serializing_if = "Option::is_none")]
    pub extended_info: Option<AzureWorkloadJobExtendedInfo>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureWorkloadJobExtendedInfo {
    #[serde(rename = "tasksList", skip_serializing_if = "Vec::is_empty")]
    pub tasks_list: Vec<AzureWorkloadJobTaskDetails>,
    #[serde(rename = "propertyBag", skip_serializing_if = "Option::is_none")]
    pub property_bag: Option<serde_json::Value>,
    #[serde(rename = "dynamicErrorMessage", skip_serializing_if = "Option::is_none")]
    pub dynamic_error_message: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureWorkloadJobTaskDetails {
    #[serde(rename = "taskId", skip_serializing_if = "Option::is_none")]
    pub task_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureWorkloadSqlAutoProtectionIntent {
    #[serde(flatten)]
    pub azure_workload_auto_protection_intent: AzureWorkloadAutoProtectionIntent,
    #[serde(rename = "workloadItemType", skip_serializing_if = "Option::is_none")]
    pub workload_item_type: Option<azure_workload_sql_auto_protection_intent::WorkloadItemType>,
}
mod azure_workload_sql_auto_protection_intent {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum WorkloadItemType {
        Invalid,
        #[serde(rename = "SQLInstance")]
        SqlInstance,
        #[serde(rename = "SQLDataBase")]
        SqlDataBase,
        #[serde(rename = "SAPHanaSystem")]
        SapHanaSystem,
        #[serde(rename = "SAPHanaDatabase")]
        SapHanaDatabase,
        #[serde(rename = "SAPAseSystem")]
        SapAseSystem,
        #[serde(rename = "SAPAseDatabase")]
        SapAseDatabase,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BackupManagementUsage {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<backup_management_usage::Unit>,
    #[serde(rename = "quotaPeriod", skip_serializing_if = "Option::is_none")]
    pub quota_period: Option<String>,
    #[serde(rename = "nextResetTime", skip_serializing_if = "Option::is_none")]
    pub next_reset_time: Option<String>,
    #[serde(rename = "currentValue", skip_serializing_if = "Option::is_none")]
    pub current_value: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<NameInfo>,
}
mod backup_management_usage {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Unit {
        Count,
        Bytes,
        Seconds,
        Percent,
        CountPerSecond,
        BytesPerSecond,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BackupManagementUsageList {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<BackupManagementUsage>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BackupStatusRequest {
    #[serde(rename = "resourceType", skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<backup_status_request::ResourceType>,
    #[serde(rename = "resourceId", skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    #[serde(rename = "poLogicalName", skip_serializing_if = "Option::is_none")]
    pub po_logical_name: Option<String>,
}
mod backup_status_request {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ResourceType {
        Invalid,
        #[serde(rename = "VM")]
        Vm,
        FileFolder,
        AzureSqlDb,
        #[serde(rename = "SQLDB")]
        Sqldb,
        Exchange,
        Sharepoint,
        #[serde(rename = "VMwareVM")]
        VMwareVm,
        SystemState,
        Client,
        GenericDataSource,
        #[serde(rename = "SQLDataBase")]
        SqlDataBase,
        AzureFileShare,
        #[serde(rename = "SAPHanaDatabase")]
        SapHanaDatabase,
        #[serde(rename = "SAPAseDatabase")]
        SapAseDatabase,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BackupStatusResponse {
    #[serde(rename = "protectionStatus", skip_serializing_if = "Option::is_none")]
    pub protection_status: Option<backup_status_response::ProtectionStatus>,
    #[serde(rename = "vaultId", skip_serializing_if = "Option::is_none")]
    pub vault_id: Option<String>,
    #[serde(rename = "fabricName", skip_serializing_if = "Option::is_none")]
    pub fabric_name: Option<backup_status_response::FabricName>,
    #[serde(rename = "containerName", skip_serializing_if = "Option::is_none")]
    pub container_name: Option<String>,
    #[serde(rename = "protectedItemName", skip_serializing_if = "Option::is_none")]
    pub protected_item_name: Option<String>,
    #[serde(rename = "errorCode", skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    #[serde(rename = "errorMessage", skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    #[serde(rename = "policyName", skip_serializing_if = "Option::is_none")]
    pub policy_name: Option<String>,
    #[serde(rename = "registrationStatus", skip_serializing_if = "Option::is_none")]
    pub registration_status: Option<String>,
}
mod backup_status_response {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ProtectionStatus {
        Invalid,
        NotProtected,
        Protecting,
        Protected,
        ProtectionFailed,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum FabricName {
        Invalid,
        Azure,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BmsBackupSummariesQueryObject {
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: Option<bms_backup_summaries_query_object::Type>,
}
mod bms_backup_summaries_query_object {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        Invalid,
        BackupProtectedItemCountSummary,
        BackupProtectionContainerCountSummary,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DpmErrorInfo {
    #[serde(rename = "errorString", skip_serializing_if = "Option::is_none")]
    pub error_string: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub recommendations: Vec<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DpmJob {
    #[serde(flatten)]
    pub job: Job,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<String>,
    #[serde(rename = "dpmServerName", skip_serializing_if = "Option::is_none")]
    pub dpm_server_name: Option<String>,
    #[serde(rename = "containerName", skip_serializing_if = "Option::is_none")]
    pub container_name: Option<String>,
    #[serde(rename = "containerType", skip_serializing_if = "Option::is_none")]
    pub container_type: Option<String>,
    #[serde(rename = "workloadType", skip_serializing_if = "Option::is_none")]
    pub workload_type: Option<String>,
    #[serde(rename = "actionsInfo", skip_serializing_if = "Vec::is_empty")]
    pub actions_info: Vec<String>,
    #[serde(rename = "errorDetails", skip_serializing_if = "Vec::is_empty")]
    pub error_details: Vec<DpmErrorInfo>,
    #[serde(rename = "extendedInfo", skip_serializing_if = "Option::is_none")]
    pub extended_info: Option<DpmJobExtendedInfo>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DpmJobExtendedInfo {
    #[serde(rename = "tasksList", skip_serializing_if = "Vec::is_empty")]
    pub tasks_list: Vec<DpmJobTaskDetails>,
    #[serde(rename = "propertyBag", skip_serializing_if = "Option::is_none")]
    pub property_bag: Option<serde_json::Value>,
    #[serde(rename = "dynamicErrorMessage", skip_serializing_if = "Option::is_none")]
    pub dynamic_error_message: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DpmJobTaskDetails {
    #[serde(rename = "taskId", skip_serializing_if = "Option::is_none")]
    pub task_id: Option<String>,
    #[serde(rename = "startTime", skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    #[serde(rename = "endTime", skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FeatureSupportRequest {
    #[serde(rename = "featureType", skip_serializing_if = "Option::is_none")]
    pub feature_type: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InstantRpAdditionalDetails {
    #[serde(rename = "azureBackupRGNamePrefix", skip_serializing_if = "Option::is_none")]
    pub azure_backup_rg_name_prefix: Option<String>,
    #[serde(rename = "azureBackupRGNameSuffix", skip_serializing_if = "Option::is_none")]
    pub azure_backup_rg_name_suffix: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Job {
    #[serde(rename = "entityFriendlyName", skip_serializing_if = "Option::is_none")]
    pub entity_friendly_name: Option<String>,
    #[serde(rename = "backupManagementType", skip_serializing_if = "Option::is_none")]
    pub backup_management_type: Option<job::BackupManagementType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "startTime", skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    #[serde(rename = "endTime", skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    #[serde(rename = "activityId", skip_serializing_if = "Option::is_none")]
    pub activity_id: Option<String>,
    #[serde(rename = "jobType")]
    pub job_type: String,
}
mod job {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum BackupManagementType {
        Invalid,
        #[serde(rename = "AzureIaasVM")]
        AzureIaasVm,
        #[serde(rename = "MAB")]
        Mab,
        #[serde(rename = "DPM")]
        Dpm,
        AzureBackupServer,
        AzureSql,
        AzureStorage,
        AzureWorkload,
        DefaultBackup,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct JobResource {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<Job>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MabErrorInfo {
    #[serde(rename = "errorString", skip_serializing)]
    pub error_string: Option<String>,
    #[serde(skip_serializing)]
    pub recommendations: Vec<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MabJob {
    #[serde(flatten)]
    pub job: Job,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<String>,
    #[serde(rename = "actionsInfo", skip_serializing_if = "Vec::is_empty")]
    pub actions_info: Vec<String>,
    #[serde(rename = "mabServerName", skip_serializing_if = "Option::is_none")]
    pub mab_server_name: Option<String>,
    #[serde(rename = "mabServerType", skip_serializing_if = "Option::is_none")]
    pub mab_server_type: Option<mab_job::MabServerType>,
    #[serde(rename = "workloadType", skip_serializing_if = "Option::is_none")]
    pub workload_type: Option<mab_job::WorkloadType>,
    #[serde(rename = "errorDetails", skip_serializing_if = "Vec::is_empty")]
    pub error_details: Vec<MabErrorInfo>,
    #[serde(rename = "extendedInfo", skip_serializing_if = "Option::is_none")]
    pub extended_info: Option<MabJobExtendedInfo>,
}
mod mab_job {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum MabServerType {
        Invalid,
        Unknown,
        #[serde(rename = "IaasVMContainer")]
        IaasVmContainer,
        #[serde(rename = "IaasVMServiceContainer")]
        IaasVmServiceContainer,
        #[serde(rename = "DPMContainer")]
        DpmContainer,
        AzureBackupServerContainer,
        #[serde(rename = "MABContainer")]
        MabContainer,
        Cluster,
        AzureSqlContainer,
        Windows,
        VCenter,
        #[serde(rename = "VMAppContainer")]
        VmAppContainer,
        #[serde(rename = "SQLAGWorkLoadContainer")]
        SqlagWorkLoadContainer,
        StorageContainer,
        GenericContainer,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum WorkloadType {
        Invalid,
        #[serde(rename = "VM")]
        Vm,
        FileFolder,
        AzureSqlDb,
        #[serde(rename = "SQLDB")]
        Sqldb,
        Exchange,
        Sharepoint,
        #[serde(rename = "VMwareVM")]
        VMwareVm,
        SystemState,
        Client,
        GenericDataSource,
        #[serde(rename = "SQLDataBase")]
        SqlDataBase,
        AzureFileShare,
        #[serde(rename = "SAPHanaDatabase")]
        SapHanaDatabase,
        #[serde(rename = "SAPAseDatabase")]
        SapAseDatabase,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MabJobExtendedInfo {
    #[serde(rename = "tasksList", skip_serializing_if = "Vec::is_empty")]
    pub tasks_list: Vec<MabJobTaskDetails>,
    #[serde(rename = "propertyBag", skip_serializing_if = "Option::is_none")]
    pub property_bag: Option<serde_json::Value>,
    #[serde(rename = "dynamicErrorMessage", skip_serializing_if = "Option::is_none")]
    pub dynamic_error_message: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MabJobTaskDetails {
    #[serde(rename = "taskId", skip_serializing_if = "Option::is_none")]
    pub task_id: Option<String>,
    #[serde(rename = "startTime", skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    #[serde(rename = "endTime", skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NameInfo {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(rename = "localizedValue", skip_serializing_if = "Option::is_none")]
    pub localized_value: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PreValidateEnableBackupRequest {
    #[serde(rename = "resourceType", skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<pre_validate_enable_backup_request::ResourceType>,
    #[serde(rename = "resourceId", skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    #[serde(rename = "vaultId", skip_serializing_if = "Option::is_none")]
    pub vault_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<String>,
}
mod pre_validate_enable_backup_request {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ResourceType {
        Invalid,
        #[serde(rename = "VM")]
        Vm,
        FileFolder,
        AzureSqlDb,
        #[serde(rename = "SQLDB")]
        Sqldb,
        Exchange,
        Sharepoint,
        #[serde(rename = "VMwareVM")]
        VMwareVm,
        SystemState,
        Client,
        GenericDataSource,
        #[serde(rename = "SQLDataBase")]
        SqlDataBase,
        AzureFileShare,
        #[serde(rename = "SAPHanaDatabase")]
        SapHanaDatabase,
        #[serde(rename = "SAPAseDatabase")]
        SapAseDatabase,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PreValidateEnableBackupResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<pre_validate_enable_backup_response::Status>,
    #[serde(rename = "errorCode", skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    #[serde(rename = "errorMessage", skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recommendation: Option<String>,
    #[serde(rename = "containerName", skip_serializing_if = "Option::is_none")]
    pub container_name: Option<String>,
    #[serde(rename = "protectedItemName", skip_serializing_if = "Option::is_none")]
    pub protected_item_name: Option<String>,
}
mod pre_validate_enable_backup_response {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Status {
        Invalid,
        Succeeded,
        Failed,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProtectionIntent {
    #[serde(rename = "protectionIntentItemType", skip_serializing_if = "Option::is_none")]
    pub protection_intent_item_type: Option<String>,
    #[serde(rename = "backupManagementType", skip_serializing_if = "Option::is_none")]
    pub backup_management_type: Option<protection_intent::BackupManagementType>,
    #[serde(rename = "sourceResourceId", skip_serializing_if = "Option::is_none")]
    pub source_resource_id: Option<String>,
    #[serde(rename = "itemId", skip_serializing_if = "Option::is_none")]
    pub item_id: Option<String>,
    #[serde(rename = "policyId", skip_serializing_if = "Option::is_none")]
    pub policy_id: Option<String>,
    #[serde(rename = "protectionState", skip_serializing_if = "Option::is_none")]
    pub protection_state: Option<protection_intent::ProtectionState>,
}
mod protection_intent {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum BackupManagementType {
        Invalid,
        #[serde(rename = "AzureIaasVM")]
        AzureIaasVm,
        #[serde(rename = "MAB")]
        Mab,
        #[serde(rename = "DPM")]
        Dpm,
        AzureBackupServer,
        AzureSql,
        AzureStorage,
        AzureWorkload,
        DefaultBackup,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ProtectionState {
        Invalid,
        NotProtected,
        Protecting,
        Protected,
        ProtectionFailed,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProtectionIntentQueryObject {
    #[serde(rename = "backupManagementType", skip_serializing_if = "Option::is_none")]
    pub backup_management_type: Option<protection_intent_query_object::BackupManagementType>,
    #[serde(rename = "itemType", skip_serializing_if = "Option::is_none")]
    pub item_type: Option<protection_intent_query_object::ItemType>,
    #[serde(rename = "parentName", skip_serializing_if = "Option::is_none")]
    pub parent_name: Option<String>,
    #[serde(rename = "itemName", skip_serializing_if = "Option::is_none")]
    pub item_name: Option<String>,
}
mod protection_intent_query_object {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum BackupManagementType {
        Invalid,
        #[serde(rename = "AzureIaasVM")]
        AzureIaasVm,
        #[serde(rename = "MAB")]
        Mab,
        #[serde(rename = "DPM")]
        Dpm,
        AzureBackupServer,
        AzureSql,
        AzureStorage,
        AzureWorkload,
        DefaultBackup,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ItemType {
        Invalid,
        #[serde(rename = "SQLInstance")]
        SqlInstance,
        #[serde(rename = "SQLAvailabilityGroupContainer")]
        SqlAvailabilityGroupContainer,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProtectionIntentResource {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<ProtectionIntent>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProtectionIntentResourceList {
    #[serde(flatten)]
    pub resource_list: ResourceList,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ProtectionIntentResource>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Resource {
    #[serde(skip_serializing)]
    pub id: Option<String>,
    #[serde(skip_serializing)]
    pub name: Option<String>,
    #[serde(rename = "type", skip_serializing)]
    pub type_: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[serde(rename = "eTag", skip_serializing_if = "Option::is_none")]
    pub e_tag: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResourceList {
    #[serde(rename = "nextLink", skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
