#![doc = "generated by AutoRust 0.1.0"]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CertificateVerificationDescription {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CertificateListDescription {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<CertificateDescription>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CertificateBodyDescription {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CertificateDescription {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<CertificateProperties>,
    #[serde(skip_serializing)]
    pub id: Option<String>,
    #[serde(skip_serializing)]
    pub name: Option<String>,
    #[serde(skip_serializing)]
    pub etag: Option<String>,
    #[serde(rename = "type", skip_serializing)]
    pub type_: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CertificateWithNonceDescription {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<CertificatePropertiesWithNonce>,
    #[serde(skip_serializing)]
    pub id: Option<String>,
    #[serde(skip_serializing)]
    pub name: Option<String>,
    #[serde(skip_serializing)]
    pub etag: Option<String>,
    #[serde(rename = "type", skip_serializing)]
    pub type_: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SharedAccessSignatureAuthorizationRule {
    #[serde(rename = "keyName")]
    pub key_name: String,
    #[serde(rename = "primaryKey", skip_serializing_if = "Option::is_none")]
    pub primary_key: Option<String>,
    #[serde(rename = "secondaryKey", skip_serializing_if = "Option::is_none")]
    pub secondary_key: Option<String>,
    pub rights: shared_access_signature_authorization_rule::Rights,
}
mod shared_access_signature_authorization_rule {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Rights {
        RegistryRead,
        RegistryWrite,
        ServiceConnect,
        DeviceConnect,
        #[serde(rename = "RegistryRead, RegistryWrite")]
        RegistryReadRegistryWrite,
        #[serde(rename = "RegistryRead, ServiceConnect")]
        RegistryReadServiceConnect,
        #[serde(rename = "RegistryRead, DeviceConnect")]
        RegistryReadDeviceConnect,
        #[serde(rename = "RegistryWrite, ServiceConnect")]
        RegistryWriteServiceConnect,
        #[serde(rename = "RegistryWrite, DeviceConnect")]
        RegistryWriteDeviceConnect,
        #[serde(rename = "ServiceConnect, DeviceConnect")]
        ServiceConnectDeviceConnect,
        #[serde(rename = "RegistryRead, RegistryWrite, ServiceConnect")]
        RegistryReadRegistryWriteServiceConnect,
        #[serde(rename = "RegistryRead, RegistryWrite, DeviceConnect")]
        RegistryReadRegistryWriteDeviceConnect,
        #[serde(rename = "RegistryRead, ServiceConnect, DeviceConnect")]
        RegistryReadServiceConnectDeviceConnect,
        #[serde(rename = "RegistryWrite, ServiceConnect, DeviceConnect")]
        RegistryWriteServiceConnectDeviceConnect,
        #[serde(rename = "RegistryRead, RegistryWrite, ServiceConnect, DeviceConnect")]
        RegistryReadRegistryWriteServiceConnectDeviceConnect,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CertificateProperties {
    #[serde(skip_serializing)]
    pub subject: Option<String>,
    #[serde(skip_serializing)]
    pub expiry: Option<String>,
    #[serde(skip_serializing)]
    pub thumbprint: Option<String>,
    #[serde(rename = "isVerified", skip_serializing)]
    pub is_verified: Option<bool>,
    #[serde(skip_serializing)]
    pub created: Option<String>,
    #[serde(skip_serializing)]
    pub updated: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CertificatePropertiesWithNonce {
    #[serde(skip_serializing)]
    pub subject: Option<String>,
    #[serde(skip_serializing)]
    pub expiry: Option<String>,
    #[serde(skip_serializing)]
    pub thumbprint: Option<String>,
    #[serde(rename = "isVerified", skip_serializing)]
    pub is_verified: Option<bool>,
    #[serde(skip_serializing)]
    pub created: Option<String>,
    #[serde(skip_serializing)]
    pub updated: Option<String>,
    #[serde(rename = "verificationCode", skip_serializing)]
    pub verification_code: Option<String>,
    #[serde(skip_serializing)]
    pub certificate: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IotHubProperties {
    #[serde(rename = "authorizationPolicies", skip_serializing_if = "Vec::is_empty")]
    pub authorization_policies: Vec<SharedAccessSignatureAuthorizationRule>,
    #[serde(rename = "ipFilterRules", skip_serializing_if = "Vec::is_empty")]
    pub ip_filter_rules: Vec<IpFilterRule>,
    #[serde(rename = "provisioningState", skip_serializing)]
    pub provisioning_state: Option<String>,
    #[serde(skip_serializing)]
    pub state: Option<String>,
    #[serde(rename = "hostName", skip_serializing)]
    pub host_name: Option<String>,
    #[serde(rename = "eventHubEndpoints", skip_serializing_if = "Option::is_none")]
    pub event_hub_endpoints: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub routing: Option<RoutingProperties>,
    #[serde(rename = "storageEndpoints", skip_serializing_if = "Option::is_none")]
    pub storage_endpoints: Option<serde_json::Value>,
    #[serde(rename = "messagingEndpoints", skip_serializing_if = "Option::is_none")]
    pub messaging_endpoints: Option<serde_json::Value>,
    #[serde(rename = "enableFileUploadNotifications", skip_serializing_if = "Option::is_none")]
    pub enable_file_upload_notifications: Option<bool>,
    #[serde(rename = "cloudToDevice", skip_serializing_if = "Option::is_none")]
    pub cloud_to_device: Option<CloudToDeviceProperties>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comments: Option<String>,
    #[serde(rename = "deviceStreams", skip_serializing_if = "Option::is_none")]
    pub device_streams: Option<iot_hub_properties::DeviceStreams>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub features: Option<iot_hub_properties::Features>,
    #[serde(skip_serializing)]
    pub locations: Vec<IotHubLocationDescription>,
}
mod iot_hub_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct DeviceStreams {
        #[serde(rename = "streamingEndpoints", skip_serializing_if = "Vec::is_empty")]
        pub streaming_endpoints: Vec<String>,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Features {
        None,
        DeviceManagement,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IotHubSkuInfo {
    pub name: iot_hub_sku_info::Name,
    #[serde(skip_serializing)]
    pub tier: Option<iot_hub_sku_info::Tier>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capacity: Option<i64>,
}
mod iot_hub_sku_info {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Name {
        F1,
        S1,
        S2,
        S3,
        B1,
        B2,
        B3,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Tier {
        Free,
        Standard,
        Basic,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EventHubProperties {
    #[serde(rename = "retentionTimeInDays", skip_serializing_if = "Option::is_none")]
    pub retention_time_in_days: Option<i64>,
    #[serde(rename = "partitionCount", skip_serializing_if = "Option::is_none")]
    pub partition_count: Option<i32>,
    #[serde(rename = "partitionIds", skip_serializing)]
    pub partition_ids: Vec<String>,
    #[serde(skip_serializing)]
    pub path: Option<String>,
    #[serde(skip_serializing)]
    pub endpoint: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StorageEndpointProperties {
    #[serde(rename = "sasTtlAsIso8601", skip_serializing_if = "Option::is_none")]
    pub sas_ttl_as_iso8601: Option<String>,
    #[serde(rename = "connectionString")]
    pub connection_string: String,
    #[serde(rename = "containerName")]
    pub container_name: String,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MessagingEndpointProperties {
    #[serde(rename = "lockDurationAsIso8601", skip_serializing_if = "Option::is_none")]
    pub lock_duration_as_iso8601: Option<String>,
    #[serde(rename = "ttlAsIso8601", skip_serializing_if = "Option::is_none")]
    pub ttl_as_iso8601: Option<String>,
    #[serde(rename = "maxDeliveryCount", skip_serializing_if = "Option::is_none")]
    pub max_delivery_count: Option<i32>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CloudToDeviceProperties {
    #[serde(rename = "maxDeliveryCount", skip_serializing_if = "Option::is_none")]
    pub max_delivery_count: Option<i32>,
    #[serde(rename = "defaultTtlAsIso8601", skip_serializing_if = "Option::is_none")]
    pub default_ttl_as_iso8601: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub feedback: Option<FeedbackProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IpFilterRule {
    #[serde(rename = "filterName")]
    pub filter_name: String,
    pub action: ip_filter_rule::Action,
    #[serde(rename = "ipMask")]
    pub ip_mask: String,
}
mod ip_filter_rule {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Action {
        Accept,
        Reject,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FeedbackProperties {
    #[serde(rename = "lockDurationAsIso8601", skip_serializing_if = "Option::is_none")]
    pub lock_duration_as_iso8601: Option<String>,
    #[serde(rename = "ttlAsIso8601", skip_serializing_if = "Option::is_none")]
    pub ttl_as_iso8601: Option<String>,
    #[serde(rename = "maxDeliveryCount", skip_serializing_if = "Option::is_none")]
    pub max_delivery_count: Option<i32>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RoutingProperties {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoints: Option<RoutingEndpoints>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub routes: Vec<RouteProperties>,
    #[serde(rename = "fallbackRoute", skip_serializing_if = "Option::is_none")]
    pub fallback_route: Option<FallbackRouteProperties>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub enrichments: Vec<EnrichmentProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RoutingEndpoints {
    #[serde(rename = "serviceBusQueues", skip_serializing_if = "Vec::is_empty")]
    pub service_bus_queues: Vec<RoutingServiceBusQueueEndpointProperties>,
    #[serde(rename = "serviceBusTopics", skip_serializing_if = "Vec::is_empty")]
    pub service_bus_topics: Vec<RoutingServiceBusTopicEndpointProperties>,
    #[serde(rename = "eventHubs", skip_serializing_if = "Vec::is_empty")]
    pub event_hubs: Vec<RoutingEventHubProperties>,
    #[serde(rename = "storageContainers", skip_serializing_if = "Vec::is_empty")]
    pub storage_containers: Vec<RoutingStorageContainerProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RoutingServiceBusQueueEndpointProperties {
    #[serde(rename = "connectionString")]
    pub connection_string: String,
    pub name: String,
    #[serde(rename = "subscriptionId", skip_serializing_if = "Option::is_none")]
    pub subscription_id: Option<String>,
    #[serde(rename = "resourceGroup", skip_serializing_if = "Option::is_none")]
    pub resource_group: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RoutingServiceBusTopicEndpointProperties {
    #[serde(rename = "connectionString")]
    pub connection_string: String,
    pub name: String,
    #[serde(rename = "subscriptionId", skip_serializing_if = "Option::is_none")]
    pub subscription_id: Option<String>,
    #[serde(rename = "resourceGroup", skip_serializing_if = "Option::is_none")]
    pub resource_group: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RoutingEventHubProperties {
    #[serde(rename = "connectionString")]
    pub connection_string: String,
    pub name: String,
    #[serde(rename = "subscriptionId", skip_serializing_if = "Option::is_none")]
    pub subscription_id: Option<String>,
    #[serde(rename = "resourceGroup", skip_serializing_if = "Option::is_none")]
    pub resource_group: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RoutingStorageContainerProperties {
    #[serde(rename = "connectionString")]
    pub connection_string: String,
    pub name: String,
    #[serde(rename = "subscriptionId", skip_serializing_if = "Option::is_none")]
    pub subscription_id: Option<String>,
    #[serde(rename = "resourceGroup", skip_serializing_if = "Option::is_none")]
    pub resource_group: Option<String>,
    #[serde(rename = "containerName")]
    pub container_name: String,
    #[serde(rename = "fileNameFormat", skip_serializing_if = "Option::is_none")]
    pub file_name_format: Option<String>,
    #[serde(rename = "batchFrequencyInSeconds", skip_serializing_if = "Option::is_none")]
    pub batch_frequency_in_seconds: Option<i32>,
    #[serde(rename = "maxChunkSizeInBytes", skip_serializing_if = "Option::is_none")]
    pub max_chunk_size_in_bytes: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encoding: Option<routing_storage_container_properties::Encoding>,
}
mod routing_storage_container_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Encoding {
        Avro,
        AvroDeflate,
        #[serde(rename = "JSON")]
        Json,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RouteProperties {
    pub name: String,
    pub source: route_properties::Source,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub condition: Option<String>,
    #[serde(rename = "endpointNames")]
    pub endpoint_names: Vec<String>,
    #[serde(rename = "isEnabled")]
    pub is_enabled: bool,
}
mod route_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Source {
        Invalid,
        DeviceMessages,
        TwinChangeEvents,
        DeviceLifecycleEvents,
        DeviceJobLifecycleEvents,
        DigitalTwinChangeEvents,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FallbackRouteProperties {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    pub source: fallback_route_properties::Source,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub condition: Option<String>,
    #[serde(rename = "endpointNames")]
    pub endpoint_names: Vec<String>,
    #[serde(rename = "isEnabled")]
    pub is_enabled: bool,
}
mod fallback_route_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Source {
        DeviceMessages,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EnrichmentProperties {
    pub key: String,
    pub value: String,
    #[serde(rename = "endpointNames")]
    pub endpoint_names: Vec<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IotHubDescription {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<IotHubProperties>,
    pub sku: IotHubSkuInfo,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Resource {
    #[serde(skip_serializing)]
    pub id: Option<String>,
    #[serde(skip_serializing)]
    pub name: Option<String>,
    #[serde(rename = "type", skip_serializing)]
    pub type_: Option<String>,
    pub location: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SharedAccessSignatureAuthorizationRuleListResult {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<SharedAccessSignatureAuthorizationRule>,
    #[serde(rename = "nextLink", skip_serializing)]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OperationListResult {
    #[serde(skip_serializing)]
    pub value: Vec<Operation>,
    #[serde(rename = "nextLink", skip_serializing)]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Operation {
    #[serde(skip_serializing)]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<operation::Display>,
}
mod operation {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct Display {
        #[serde(skip_serializing)]
        pub provider: Option<String>,
        #[serde(skip_serializing)]
        pub resource: Option<String>,
        #[serde(skip_serializing)]
        pub operation: Option<String>,
        #[serde(skip_serializing)]
        pub description: Option<String>,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ErrorDetails {
    #[serde(skip_serializing)]
    pub code: Option<String>,
    #[serde(rename = "httpStatusCode", skip_serializing)]
    pub http_status_code: Option<String>,
    #[serde(skip_serializing)]
    pub message: Option<String>,
    #[serde(skip_serializing)]
    pub details: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IotHubQuotaMetricInfoListResult {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<IotHubQuotaMetricInfo>,
    #[serde(rename = "nextLink", skip_serializing)]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EndpointHealthDataListResult {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<EndpointHealthData>,
    #[serde(rename = "nextLink", skip_serializing)]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EndpointHealthData {
    #[serde(rename = "endpointId", skip_serializing_if = "Option::is_none")]
    pub endpoint_id: Option<String>,
    #[serde(rename = "healthStatus", skip_serializing_if = "Option::is_none")]
    pub health_status: Option<endpoint_health_data::HealthStatus>,
}
mod endpoint_health_data {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum HealthStatus {
        #[serde(rename = "unknown")]
        Unknown,
        #[serde(rename = "healthy")]
        Healthy,
        #[serde(rename = "unhealthy")]
        Unhealthy,
        #[serde(rename = "dead")]
        Dead,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RegistryStatistics {
    #[serde(rename = "totalDeviceCount", skip_serializing)]
    pub total_device_count: Option<i64>,
    #[serde(rename = "enabledDeviceCount", skip_serializing)]
    pub enabled_device_count: Option<i64>,
    #[serde(rename = "disabledDeviceCount", skip_serializing)]
    pub disabled_device_count: Option<i64>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct JobResponseListResult {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<JobResponse>,
    #[serde(rename = "nextLink", skip_serializing)]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IotHubSkuDescription {
    #[serde(rename = "resourceType", skip_serializing)]
    pub resource_type: Option<String>,
    pub sku: IotHubSkuInfo,
    pub capacity: IotHubCapacity,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TagsResource {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IotHubCapacity {
    #[serde(skip_serializing)]
    pub minimum: Option<i64>,
    #[serde(skip_serializing)]
    pub maximum: Option<i64>,
    #[serde(skip_serializing)]
    pub default: Option<i64>,
    #[serde(rename = "scaleType", skip_serializing)]
    pub scale_type: Option<iot_hub_capacity::ScaleType>,
}
mod iot_hub_capacity {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ScaleType {
        Automatic,
        Manual,
        None,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EventHubConsumerGroupsListResult {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<EventHubConsumerGroupInfo>,
    #[serde(rename = "nextLink", skip_serializing)]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EventHubConsumerGroupInfo {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
    #[serde(skip_serializing)]
    pub id: Option<String>,
    #[serde(skip_serializing)]
    pub name: Option<String>,
    #[serde(rename = "type", skip_serializing)]
    pub type_: Option<String>,
    #[serde(skip_serializing)]
    pub etag: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IotHubSkuDescriptionListResult {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<IotHubSkuDescription>,
    #[serde(rename = "nextLink", skip_serializing)]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct JobResponse {
    #[serde(rename = "jobId", skip_serializing)]
    pub job_id: Option<String>,
    #[serde(rename = "startTimeUtc", skip_serializing)]
    pub start_time_utc: Option<String>,
    #[serde(rename = "endTimeUtc", skip_serializing)]
    pub end_time_utc: Option<String>,
    #[serde(rename = "type", skip_serializing)]
    pub type_: Option<job_response::Type>,
    #[serde(skip_serializing)]
    pub status: Option<job_response::Status>,
    #[serde(rename = "failureReason", skip_serializing)]
    pub failure_reason: Option<String>,
    #[serde(rename = "statusMessage", skip_serializing)]
    pub status_message: Option<String>,
    #[serde(rename = "parentJobId", skip_serializing)]
    pub parent_job_id: Option<String>,
}
mod job_response {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        #[serde(rename = "unknown")]
        Unknown,
        #[serde(rename = "export")]
        Export,
        #[serde(rename = "import")]
        Import,
        #[serde(rename = "backup")]
        Backup,
        #[serde(rename = "readDeviceProperties")]
        ReadDeviceProperties,
        #[serde(rename = "writeDeviceProperties")]
        WriteDeviceProperties,
        #[serde(rename = "updateDeviceConfiguration")]
        UpdateDeviceConfiguration,
        #[serde(rename = "rebootDevice")]
        RebootDevice,
        #[serde(rename = "factoryResetDevice")]
        FactoryResetDevice,
        #[serde(rename = "firmwareUpdate")]
        FirmwareUpdate,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Status {
        #[serde(rename = "unknown")]
        Unknown,
        #[serde(rename = "enqueued")]
        Enqueued,
        #[serde(rename = "running")]
        Running,
        #[serde(rename = "completed")]
        Completed,
        #[serde(rename = "failed")]
        Failed,
        #[serde(rename = "cancelled")]
        Cancelled,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IotHubDescriptionListResult {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<IotHubDescription>,
    #[serde(rename = "nextLink", skip_serializing)]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IotHubQuotaMetricInfo {
    #[serde(skip_serializing)]
    pub name: Option<String>,
    #[serde(rename = "currentValue", skip_serializing)]
    pub current_value: Option<i64>,
    #[serde(rename = "maxValue", skip_serializing)]
    pub max_value: Option<i64>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OperationInputs {
    pub name: String,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IotHubNameAvailabilityInfo {
    #[serde(rename = "nameAvailable", skip_serializing)]
    pub name_available: Option<bool>,
    #[serde(skip_serializing)]
    pub reason: Option<iot_hub_name_availability_info::Reason>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
mod iot_hub_name_availability_info {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Reason {
        Invalid,
        AlreadyExists,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserSubscriptionQuotaListResult {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<UserSubscriptionQuota>,
    #[serde(rename = "nextLink", skip_serializing)]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserSubscriptionQuota {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
    #[serde(rename = "currentValue", skip_serializing_if = "Option::is_none")]
    pub current_value: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<Name>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Name {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(rename = "localizedValue", skip_serializing_if = "Option::is_none")]
    pub localized_value: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TestAllRoutesInput {
    #[serde(rename = "routingSource", skip_serializing_if = "Option::is_none")]
    pub routing_source: Option<test_all_routes_input::RoutingSource>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<RoutingMessage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub twin: Option<RoutingTwin>,
}
mod test_all_routes_input {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum RoutingSource {
        Invalid,
        DeviceMessages,
        TwinChangeEvents,
        DeviceLifecycleEvents,
        DeviceJobLifecycleEvents,
        DigitalTwinChangeEvents,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RoutingTwin {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<routing_twin::Properties>,
}
mod routing_twin {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct Properties {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub desired: Option<serde_json::Value>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub reported: Option<serde_json::Value>,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RoutingMessage {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
    #[serde(rename = "appProperties", skip_serializing_if = "Option::is_none")]
    pub app_properties: Option<serde_json::Value>,
    #[serde(rename = "systemProperties", skip_serializing_if = "Option::is_none")]
    pub system_properties: Option<serde_json::Value>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TestAllRoutesResult {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub routes: Vec<MatchedRoute>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MatchedRoute {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<RouteProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TestRouteInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<RoutingMessage>,
    pub route: RouteProperties,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub twin: Option<RoutingTwin>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TestRouteResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<test_route_result::Result>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<TestRouteResultDetails>,
}
mod test_route_result {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Result {
        #[serde(rename = "undefined")]
        Undefined,
        #[serde(rename = "false")]
        False,
        #[serde(rename = "true")]
        True,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TestRouteResultDetails {
    #[serde(rename = "compilationErrors", skip_serializing_if = "Vec::is_empty")]
    pub compilation_errors: Vec<RouteCompilationError>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RouteCompilationError {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub severity: Option<route_compilation_error::Severity>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<RouteErrorRange>,
}
mod route_compilation_error {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Severity {
        #[serde(rename = "error")]
        Error,
        #[serde(rename = "warning")]
        Warning,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RouteErrorRange {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<RouteErrorPosition>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end: Option<RouteErrorPosition>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RouteErrorPosition {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub column: Option<i32>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ExportDevicesRequest {
    #[serde(rename = "exportBlobContainerUri")]
    pub export_blob_container_uri: String,
    #[serde(rename = "excludeKeys")]
    pub exclude_keys: bool,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ImportDevicesRequest {
    #[serde(rename = "inputBlobContainerUri")]
    pub input_blob_container_uri: String,
    #[serde(rename = "outputBlobContainerUri")]
    pub output_blob_container_uri: String,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FailoverInput {
    #[serde(rename = "failoverRegion")]
    pub failover_region: String,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IotHubLocationDescription {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<iot_hub_location_description::Role>,
}
mod iot_hub_location_description {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Role {
        #[serde(rename = "primary")]
        Primary,
        #[serde(rename = "secondary")]
        Secondary,
    }
}
