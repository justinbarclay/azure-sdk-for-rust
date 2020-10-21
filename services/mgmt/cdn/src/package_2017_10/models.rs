#![doc = "generated by AutoRust 0.1.0"]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Profile {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    pub sku: Sku,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<ProfileProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProfileProperties {
    #[serde(rename = "resourceState", skip_serializing)]
    pub resource_state: Option<profile_properties::ResourceState>,
    #[serde(rename = "provisioningState", skip_serializing)]
    pub provisioning_state: Option<String>,
}
mod profile_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ResourceState {
        Creating,
        Active,
        Deleting,
        Disabled,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProfileListResult {
    #[serde(skip_serializing)]
    pub value: Vec<Profile>,
    #[serde(rename = "nextLink", skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProfileUpdateParameters {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SsoUri {
    #[serde(rename = "ssoUriValue", skip_serializing)]
    pub sso_uri_value: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SupportedOptimizationTypesListResult {
    #[serde(rename = "supportedOptimizationTypes", skip_serializing)]
    pub supported_optimization_types: Vec<OptimizationType>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Endpoint {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<EndpointProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EndpointProperties {
    #[serde(flatten)]
    pub endpoint_properties_update_parameters: EndpointPropertiesUpdateParameters,
    #[serde(rename = "hostName", skip_serializing)]
    pub host_name: Option<String>,
    pub origins: Vec<DeepCreatedOrigin>,
    #[serde(rename = "resourceState", skip_serializing)]
    pub resource_state: Option<endpoint_properties::ResourceState>,
    #[serde(rename = "provisioningState", skip_serializing)]
    pub provisioning_state: Option<String>,
}
mod endpoint_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ResourceState {
        Creating,
        Deleting,
        Running,
        Starting,
        Stopped,
        Stopping,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EndpointListResult {
    #[serde(skip_serializing)]
    pub value: Vec<Endpoint>,
    #[serde(rename = "nextLink", skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EndpointUpdateParameters {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<EndpointPropertiesUpdateParameters>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EndpointPropertiesUpdateParameters {
    #[serde(rename = "originHostHeader", skip_serializing_if = "Option::is_none")]
    pub origin_host_header: Option<String>,
    #[serde(rename = "originPath", skip_serializing_if = "Option::is_none")]
    pub origin_path: Option<String>,
    #[serde(rename = "contentTypesToCompress", skip_serializing_if = "Vec::is_empty")]
    pub content_types_to_compress: Vec<String>,
    #[serde(rename = "isCompressionEnabled", skip_serializing_if = "Option::is_none")]
    pub is_compression_enabled: Option<bool>,
    #[serde(rename = "isHttpAllowed", skip_serializing_if = "Option::is_none")]
    pub is_http_allowed: Option<bool>,
    #[serde(rename = "isHttpsAllowed", skip_serializing_if = "Option::is_none")]
    pub is_https_allowed: Option<bool>,
    #[serde(rename = "queryStringCachingBehavior", skip_serializing_if = "Option::is_none")]
    pub query_string_caching_behavior: Option<QueryStringCachingBehavior>,
    #[serde(rename = "optimizationType", skip_serializing_if = "Option::is_none")]
    pub optimization_type: Option<OptimizationType>,
    #[serde(rename = "probePath", skip_serializing_if = "Option::is_none")]
    pub probe_path: Option<String>,
    #[serde(rename = "geoFilters", skip_serializing_if = "Vec::is_empty")]
    pub geo_filters: Vec<GeoFilter>,
    #[serde(rename = "deliveryPolicy", skip_serializing_if = "Option::is_none")]
    pub delivery_policy: Option<endpoint_properties_update_parameters::DeliveryPolicy>,
}
mod endpoint_properties_update_parameters {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct DeliveryPolicy {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub description: Option<String>,
        pub rules: Vec<DeliveryRule>,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeliveryRule {
    pub order: i64,
    pub actions: Vec<DeliveryRuleAction>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub conditions: Vec<DeliveryRuleCondition>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeliveryRuleCondition {
    pub name: delivery_rule_condition::Name,
}
mod delivery_rule_condition {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Name {
        UrlPath,
        UrlFileExtension,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeliveryRuleUrlPathCondition {
    #[serde(flatten)]
    pub delivery_rule_condition: DeliveryRuleCondition,
    pub parameters: UrlPathConditionParameters,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UrlPathConditionParameters {
    #[serde(rename = "@odata.type")]
    pub odata_type: url_path_condition_parameters::Odata_type,
    pub path: String,
    #[serde(rename = "matchType")]
    pub match_type: url_path_condition_parameters::MatchType,
}
mod url_path_condition_parameters {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Odata_type {
        #[serde(rename = "Microsoft.Azure.Cdn.Models.DeliveryRuleUrlPathConditionParameters")]
        Microsoft_Azure_Cdn_Models_DeliveryRuleUrlPathConditionParameters,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum MatchType {
        Literal,
        Wildcard,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeliveryRuleUrlFileExtensionCondition {
    #[serde(flatten)]
    pub delivery_rule_condition: DeliveryRuleCondition,
    pub parameters: UrlFileExtensionConditionParameters,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UrlFileExtensionConditionParameters {
    #[serde(rename = "@odata.type")]
    pub odata_type: url_file_extension_condition_parameters::Odata_type,
    pub extensions: Vec<String>,
}
mod url_file_extension_condition_parameters {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Odata_type {
        #[serde(rename = "Microsoft.Azure.Cdn.Models.DeliveryRuleUrlFileExtensionConditionParameters")]
        Microsoft_Azure_Cdn_Models_DeliveryRuleUrlFileExtensionConditionParameters,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeliveryRuleAction {
    pub name: delivery_rule_action::Name,
}
mod delivery_rule_action {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Name {
        CacheExpiration,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeliveryRuleCacheExpirationAction {
    #[serde(flatten)]
    pub delivery_rule_action: DeliveryRuleAction,
    pub parameters: CacheExpirationActionParameters,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CacheExpirationActionParameters {
    #[serde(rename = "@odata.type")]
    pub odata_type: cache_expiration_action_parameters::Odata_type,
    #[serde(rename = "cacheBehavior")]
    pub cache_behavior: cache_expiration_action_parameters::CacheBehavior,
    #[serde(rename = "cacheType")]
    pub cache_type: cache_expiration_action_parameters::CacheType,
    #[serde(rename = "cacheDuration", skip_serializing_if = "Option::is_none")]
    pub cache_duration: Option<String>,
}
mod cache_expiration_action_parameters {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Odata_type {
        #[serde(rename = "Microsoft.Azure.Cdn.Models.DeliveryRuleCacheExpirationActionParameters")]
        Microsoft_Azure_Cdn_Models_DeliveryRuleCacheExpirationActionParameters,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum CacheBehavior {
        BypassCache,
        Override,
        SetIfMissing,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum CacheType {
        All,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeepCreatedOrigin {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<DeepCreatedOriginProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeepCreatedOriginProperties {
    #[serde(rename = "hostName")]
    pub host_name: String,
    #[serde(rename = "httpPort", skip_serializing_if = "Option::is_none")]
    pub http_port: Option<i64>,
    #[serde(rename = "httpsPort", skip_serializing_if = "Option::is_none")]
    pub https_port: Option<i64>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GeoFilter {
    #[serde(rename = "relativePath")]
    pub relative_path: String,
    pub action: geo_filter::Action,
    #[serde(rename = "countryCodes")]
    pub country_codes: Vec<String>,
}
mod geo_filter {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Action {
        Block,
        Allow,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PurgeParameters {
    #[serde(rename = "contentPaths")]
    pub content_paths: Vec<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LoadParameters {
    #[serde(rename = "contentPaths")]
    pub content_paths: Vec<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Origin {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<OriginProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OriginProperties {
    #[serde(rename = "hostName")]
    pub host_name: String,
    #[serde(rename = "httpPort", skip_serializing_if = "Option::is_none")]
    pub http_port: Option<i64>,
    #[serde(rename = "httpsPort", skip_serializing_if = "Option::is_none")]
    pub https_port: Option<i64>,
    #[serde(rename = "resourceState", skip_serializing)]
    pub resource_state: Option<origin_properties::ResourceState>,
    #[serde(rename = "provisioningState", skip_serializing)]
    pub provisioning_state: Option<String>,
}
mod origin_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ResourceState {
        Creating,
        Active,
        Deleting,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OriginUpdateParameters {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<OriginPropertiesParameters>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OriginPropertiesParameters {
    #[serde(rename = "hostName", skip_serializing_if = "Option::is_none")]
    pub host_name: Option<String>,
    #[serde(rename = "httpPort", skip_serializing_if = "Option::is_none")]
    pub http_port: Option<i64>,
    #[serde(rename = "httpsPort", skip_serializing_if = "Option::is_none")]
    pub https_port: Option<i64>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OriginListResult {
    #[serde(skip_serializing)]
    pub value: Vec<Origin>,
    #[serde(rename = "nextLink", skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CustomDomain {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<CustomDomainProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CustomDomainProperties {
    #[serde(rename = "hostName")]
    pub host_name: String,
    #[serde(rename = "resourceState", skip_serializing)]
    pub resource_state: Option<custom_domain_properties::ResourceState>,
    #[serde(rename = "customHttpsProvisioningState", skip_serializing)]
    pub custom_https_provisioning_state: Option<custom_domain_properties::CustomHttpsProvisioningState>,
    #[serde(rename = "customHttpsProvisioningSubstate", skip_serializing)]
    pub custom_https_provisioning_substate: Option<custom_domain_properties::CustomHttpsProvisioningSubstate>,
    #[serde(rename = "validationData", skip_serializing_if = "Option::is_none")]
    pub validation_data: Option<String>,
    #[serde(rename = "provisioningState", skip_serializing)]
    pub provisioning_state: Option<String>,
}
mod custom_domain_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ResourceState {
        Creating,
        Active,
        Deleting,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum CustomHttpsProvisioningState {
        Enabling,
        Enabled,
        Disabling,
        Disabled,
        Failed,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum CustomHttpsProvisioningSubstate {
        SubmittingDomainControlValidationRequest,
        PendingDomainControlValidationREquestApproval,
        DomainControlValidationRequestApproved,
        DomainControlValidationRequestRejected,
        DomainControlValidationRequestTimedOut,
        IssuingCertificate,
        DeployingCertificate,
        CertificateDeployed,
        DeletingCertificate,
        CertificateDeleted,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CustomDomainParameters {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<CustomDomainPropertiesParameters>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CustomDomainPropertiesParameters {
    #[serde(rename = "hostName")]
    pub host_name: String,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CustomDomainHttpsParameters {
    #[serde(rename = "certificateSource")]
    pub certificate_source: custom_domain_https_parameters::CertificateSource,
    #[serde(rename = "protocolType")]
    pub protocol_type: custom_domain_https_parameters::ProtocolType,
}
mod custom_domain_https_parameters {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum CertificateSource {
        AzureKeyVault,
        Cdn,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ProtocolType {
        ServerNameIndication,
        #[serde(rename = "IPBased")]
        IpBased,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CdnManagedHttpsParameters {
    #[serde(flatten)]
    pub custom_domain_https_parameters: CustomDomainHttpsParameters,
    #[serde(rename = "certificateSourceParameters")]
    pub certificate_source_parameters: CdnCertificateSourceParameters,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CdnCertificateSourceParameters {
    #[serde(rename = "@odata.type")]
    pub odata_type: cdn_certificate_source_parameters::Odata_type,
    #[serde(rename = "certificateType")]
    pub certificate_type: cdn_certificate_source_parameters::CertificateType,
}
mod cdn_certificate_source_parameters {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Odata_type {
        #[serde(rename = "#Microsoft.Azure.Cdn.Models.CdnCertificateSourceParameters")]
        Microsoft_Azure_Cdn_Models_CdnCertificateSourceParameters,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum CertificateType {
        Shared,
        Dedicated,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserManagedHttpsParameters {
    #[serde(flatten)]
    pub custom_domain_https_parameters: CustomDomainHttpsParameters,
    #[serde(rename = "certificateSourceParameters")]
    pub certificate_source_parameters: KeyVaultCertificateSourceParameters,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct KeyVaultCertificateSourceParameters {
    #[serde(rename = "@odata.type")]
    pub odata_type: key_vault_certificate_source_parameters::Odata_type,
    #[serde(rename = "subscriptionId")]
    pub subscription_id: String,
    #[serde(rename = "resourceGroupName")]
    pub resource_group_name: String,
    #[serde(rename = "vaultName")]
    pub vault_name: String,
    #[serde(rename = "secretName")]
    pub secret_name: String,
    #[serde(rename = "secretVersion")]
    pub secret_version: String,
    #[serde(rename = "updateRule")]
    pub update_rule: key_vault_certificate_source_parameters::UpdateRule,
    #[serde(rename = "deleteRule")]
    pub delete_rule: key_vault_certificate_source_parameters::DeleteRule,
}
mod key_vault_certificate_source_parameters {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Odata_type {
        #[serde(rename = "#Microsoft.Azure.Cdn.Models.KeyVaultCertificateSourceParameters")]
        Microsoft_Azure_Cdn_Models_KeyVaultCertificateSourceParameters,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum UpdateRule {
        NoAction,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum DeleteRule {
        NoAction,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CustomDomainListResult {
    #[serde(skip_serializing)]
    pub value: Vec<CustomDomain>,
    #[serde(rename = "nextLink", skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ValidateCustomDomainInput {
    #[serde(rename = "hostName")]
    pub host_name: String,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ValidateCustomDomainOutput {
    #[serde(rename = "customDomainValidated", skip_serializing)]
    pub custom_domain_validated: Option<bool>,
    #[serde(skip_serializing)]
    pub reason: Option<String>,
    #[serde(skip_serializing)]
    pub message: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CheckNameAvailabilityInput {
    pub name: String,
    #[serde(rename = "type")]
    pub type_: ResourceType,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CheckNameAvailabilityOutput {
    #[serde(rename = "nameAvailable", skip_serializing)]
    pub name_available: Option<bool>,
    #[serde(skip_serializing)]
    pub reason: Option<String>,
    #[serde(skip_serializing)]
    pub message: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ValidateProbeInput {
    #[serde(rename = "probeURL")]
    pub probe_url: String,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ValidateProbeOutput {
    #[serde(rename = "isValid", skip_serializing)]
    pub is_valid: Option<bool>,
    #[serde(rename = "errorCode", skip_serializing)]
    pub error_code: Option<String>,
    #[serde(skip_serializing)]
    pub message: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResourceUsageListResult {
    #[serde(skip_serializing)]
    pub value: Vec<ResourceUsage>,
    #[serde(rename = "nextLink", skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResourceUsage {
    #[serde(rename = "resourceType", skip_serializing)]
    pub resource_type: Option<String>,
    #[serde(skip_serializing)]
    pub unit: Option<String>,
    #[serde(rename = "currentValue", skip_serializing)]
    pub current_value: Option<i64>,
    #[serde(skip_serializing)]
    pub limit: Option<i64>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ResourceType {
    #[serde(rename = "Microsoft.Cdn/Profiles/Endpoints")]
    Microsoft_CdnProfilesEndpoints,
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
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OperationsListResult {
    #[serde(skip_serializing)]
    pub value: Vec<Operation>,
    #[serde(rename = "nextLink", skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EdgenodeResult {
    #[serde(skip_serializing)]
    pub value: Vec<EdgeNode>,
    #[serde(rename = "nextLink", skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EdgeNode {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<EdgeNodeProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EdgeNodeProperties {
    #[serde(rename = "ipAddressGroups")]
    pub ip_address_groups: Vec<IpAddressGroup>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IpAddressGroup {
    #[serde(rename = "deliveryRegion", skip_serializing_if = "Option::is_none")]
    pub delivery_region: Option<String>,
    #[serde(rename = "ipv4Addresses", skip_serializing_if = "Vec::is_empty")]
    pub ipv4_addresses: Vec<CidrIpAddress>,
    #[serde(rename = "ipv6Addresses", skip_serializing_if = "Vec::is_empty")]
    pub ipv6_addresses: Vec<CidrIpAddress>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CidrIpAddress {
    #[serde(rename = "baseIpAddress", skip_serializing_if = "Option::is_none")]
    pub base_ip_address: Option<String>,
    #[serde(rename = "prefixLength", skip_serializing_if = "Option::is_none")]
    pub prefix_length: Option<i64>,
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
pub struct TrackedResource {
    #[serde(flatten)]
    pub resource: Resource,
    pub location: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProxyResource {
    #[serde(flatten)]
    pub resource: Resource,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum QueryStringCachingBehavior {
    IgnoreQueryString,
    BypassCaching,
    UseQueryString,
    NotSet,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Sku {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<sku::Name>,
}
mod sku {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Name {
        #[serde(rename = "Standard_Verizon")]
        StandardVerizon,
        #[serde(rename = "Premium_Verizon")]
        PremiumVerizon,
        #[serde(rename = "Custom_Verizon")]
        CustomVerizon,
        #[serde(rename = "Standard_Akamai")]
        StandardAkamai,
        #[serde(rename = "Standard_ChinaCdn")]
        StandardChinaCdn,
        #[serde(rename = "Premium_ChinaCdn")]
        PremiumChinaCdn,
        #[serde(rename = "Standard_Microsoft")]
        StandardMicrosoft,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum OptimizationType {
    GeneralWebDelivery,
    GeneralMediaStreaming,
    VideoOnDemandMediaStreaming,
    LargeFileDownload,
    DynamicSiteAcceleration,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ErrorResponse {
    #[serde(skip_serializing)]
    pub code: Option<String>,
    #[serde(skip_serializing)]
    pub message: Option<String>,
}
