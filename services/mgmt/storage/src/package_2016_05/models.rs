#![doc = "generated by AutoRust 0.1.0"]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StorageAccountCheckNameAvailabilityParameters {
    pub name: String,
    #[serde(rename = "type")]
    pub type_: storage_account_check_name_availability_parameters::Type,
}
mod storage_account_check_name_availability_parameters {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        #[serde(rename = "Microsoft.Storage/storageAccounts")]
        Microsoft_StorageStorageAccounts,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CheckNameAvailabilityResult {
    #[serde(rename = "nameAvailable", skip_serializing)]
    pub name_available: Option<bool>,
    #[serde(skip_serializing)]
    pub reason: Option<check_name_availability_result::Reason>,
    #[serde(skip_serializing)]
    pub message: Option<String>,
}
mod check_name_availability_result {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Reason {
        AccountNameInvalid,
        AlreadyExists,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Sku {
    pub name: sku::Name,
    #[serde(skip_serializing)]
    pub tier: Option<sku::Tier>,
}
mod sku {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Name {
        #[serde(rename = "Standard_LRS")]
        StandardLrs,
        #[serde(rename = "Standard_GRS")]
        StandardGrs,
        #[serde(rename = "Standard_RAGRS")]
        StandardRagrs,
        #[serde(rename = "Standard_ZRS")]
        StandardZrs,
        #[serde(rename = "Premium_LRS")]
        PremiumLrs,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Tier {
        Standard,
        Premium,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CustomDomain {
    pub name: String,
    #[serde(rename = "useSubDomainName", skip_serializing_if = "Option::is_none")]
    pub use_sub_domain_name: Option<bool>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EncryptionService {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(rename = "lastEnabledTime", skip_serializing)]
    pub last_enabled_time: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EncryptionServices {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blob: Option<EncryptionService>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Encryption {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub services: Option<EncryptionServices>,
    #[serde(rename = "keySource")]
    pub key_source: encryption::KeySource,
}
mod encryption {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum KeySource {
        #[serde(rename = "Microsoft.Storage")]
        Microsoft_Storage,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StorageAccountPropertiesCreateParameters {
    #[serde(rename = "customDomain", skip_serializing_if = "Option::is_none")]
    pub custom_domain: Option<CustomDomain>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption: Option<Encryption>,
    #[serde(rename = "accessTier", skip_serializing_if = "Option::is_none")]
    pub access_tier: Option<storage_account_properties_create_parameters::AccessTier>,
}
mod storage_account_properties_create_parameters {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum AccessTier {
        Hot,
        Cool,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StorageAccountCreateParameters {
    pub sku: Sku,
    pub kind: storage_account_create_parameters::Kind,
    pub location: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<StorageAccountPropertiesCreateParameters>,
}
mod storage_account_create_parameters {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Kind {
        Storage,
        BlobStorage,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Endpoints {
    #[serde(skip_serializing)]
    pub blob: Option<String>,
    #[serde(skip_serializing)]
    pub queue: Option<String>,
    #[serde(skip_serializing)]
    pub table: Option<String>,
    #[serde(skip_serializing)]
    pub file: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StorageAccountProperties {
    #[serde(rename = "provisioningState", skip_serializing)]
    pub provisioning_state: Option<storage_account_properties::ProvisioningState>,
    #[serde(rename = "primaryEndpoints", skip_serializing_if = "Option::is_none")]
    pub primary_endpoints: Option<Endpoints>,
    #[serde(rename = "primaryLocation", skip_serializing)]
    pub primary_location: Option<String>,
    #[serde(rename = "statusOfPrimary", skip_serializing)]
    pub status_of_primary: Option<storage_account_properties::StatusOfPrimary>,
    #[serde(rename = "lastGeoFailoverTime", skip_serializing)]
    pub last_geo_failover_time: Option<String>,
    #[serde(rename = "secondaryLocation", skip_serializing)]
    pub secondary_location: Option<String>,
    #[serde(rename = "statusOfSecondary", skip_serializing)]
    pub status_of_secondary: Option<storage_account_properties::StatusOfSecondary>,
    #[serde(rename = "creationTime", skip_serializing)]
    pub creation_time: Option<String>,
    #[serde(rename = "customDomain", skip_serializing_if = "Option::is_none")]
    pub custom_domain: Option<CustomDomain>,
    #[serde(rename = "secondaryEndpoints", skip_serializing_if = "Option::is_none")]
    pub secondary_endpoints: Option<Endpoints>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption: Option<Encryption>,
    #[serde(rename = "accessTier", skip_serializing)]
    pub access_tier: Option<storage_account_properties::AccessTier>,
}
mod storage_account_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ProvisioningState {
        Creating,
        #[serde(rename = "ResolvingDNS")]
        ResolvingDns,
        Succeeded,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum StatusOfPrimary {
        #[serde(rename = "available")]
        Available,
        #[serde(rename = "unavailable")]
        Unavailable,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum StatusOfSecondary {
        #[serde(rename = "available")]
        Available,
        #[serde(rename = "unavailable")]
        Unavailable,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum AccessTier {
        Hot,
        Cool,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StorageAccount {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sku: Option<Sku>,
    #[serde(skip_serializing)]
    pub kind: Option<storage_account::Kind>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<StorageAccountProperties>,
}
mod storage_account {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Kind {
        Storage,
        BlobStorage,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StorageAccountKey {
    #[serde(rename = "keyName", skip_serializing)]
    pub key_name: Option<String>,
    #[serde(skip_serializing)]
    pub value: Option<String>,
    #[serde(skip_serializing)]
    pub permissions: Option<storage_account_key::Permissions>,
}
mod storage_account_key {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Permissions {
        Read,
        Full,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StorageAccountListResult {
    #[serde(skip_serializing)]
    pub value: Vec<StorageAccount>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StorageAccountListKeysResult {
    #[serde(skip_serializing)]
    pub keys: Vec<StorageAccountKey>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StorageAccountRegenerateKeyParameters {
    #[serde(rename = "keyName")]
    pub key_name: String,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StorageAccountPropertiesUpdateParameters {
    #[serde(rename = "customDomain", skip_serializing_if = "Option::is_none")]
    pub custom_domain: Option<CustomDomain>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption: Option<Encryption>,
    #[serde(rename = "accessTier", skip_serializing_if = "Option::is_none")]
    pub access_tier: Option<storage_account_properties_update_parameters::AccessTier>,
}
mod storage_account_properties_update_parameters {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum AccessTier {
        Hot,
        Cool,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StorageAccountUpdateParameters {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sku: Option<Sku>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<StorageAccountPropertiesUpdateParameters>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UsageName {
    #[serde(skip_serializing)]
    pub value: Option<String>,
    #[serde(rename = "localizedValue", skip_serializing)]
    pub localized_value: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Usage {
    #[serde(skip_serializing)]
    pub unit: Option<usage::Unit>,
    #[serde(rename = "currentValue", skip_serializing)]
    pub current_value: Option<i32>,
    #[serde(skip_serializing)]
    pub limit: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<UsageName>,
}
mod usage {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Unit {
        Count,
        Bytes,
        Seconds,
        Percent,
        CountsPerSecond,
        BytesPerSecond,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UsageListResult {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Usage>,
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
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AccountSasParameters {
    #[serde(rename = "signedServices")]
    pub signed_services: account_sas_parameters::SignedServices,
    #[serde(rename = "signedResourceTypes")]
    pub signed_resource_types: account_sas_parameters::SignedResourceTypes,
    #[serde(rename = "signedPermission")]
    pub signed_permission: account_sas_parameters::SignedPermission,
    #[serde(rename = "signedIp", skip_serializing_if = "Option::is_none")]
    pub signed_ip: Option<String>,
    #[serde(rename = "signedProtocol", skip_serializing_if = "Option::is_none")]
    pub signed_protocol: Option<account_sas_parameters::SignedProtocol>,
    #[serde(rename = "signedStart", skip_serializing_if = "Option::is_none")]
    pub signed_start: Option<String>,
    #[serde(rename = "signedExpiry")]
    pub signed_expiry: String,
    #[serde(rename = "keyToSign", skip_serializing_if = "Option::is_none")]
    pub key_to_sign: Option<String>,
}
mod account_sas_parameters {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum SignedServices {
        #[serde(rename = "b")]
        B,
        #[serde(rename = "q")]
        Q,
        #[serde(rename = "t")]
        T,
        #[serde(rename = "f")]
        F,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum SignedResourceTypes {
        #[serde(rename = "s")]
        S,
        #[serde(rename = "c")]
        C,
        #[serde(rename = "o")]
        O,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum SignedPermission {
        #[serde(rename = "r")]
        R,
        #[serde(rename = "d")]
        D,
        #[serde(rename = "w")]
        W,
        #[serde(rename = "l")]
        L,
        #[serde(rename = "a")]
        A,
        #[serde(rename = "c")]
        C,
        #[serde(rename = "u")]
        U,
        #[serde(rename = "p")]
        P,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum SignedProtocol {
        #[serde(rename = "https,http")]
        HttpsHttp,
        #[serde(rename = "https")]
        Https,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ListAccountSasResponse {
    #[serde(rename = "accountSasToken", skip_serializing)]
    pub account_sas_token: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServiceSasParameters {
    #[serde(rename = "canonicalizedResource")]
    pub canonicalized_resource: String,
    #[serde(rename = "signedResource")]
    pub signed_resource: service_sas_parameters::SignedResource,
    #[serde(rename = "signedPermission", skip_serializing_if = "Option::is_none")]
    pub signed_permission: Option<service_sas_parameters::SignedPermission>,
    #[serde(rename = "signedIp", skip_serializing_if = "Option::is_none")]
    pub signed_ip: Option<String>,
    #[serde(rename = "signedProtocol", skip_serializing_if = "Option::is_none")]
    pub signed_protocol: Option<service_sas_parameters::SignedProtocol>,
    #[serde(rename = "signedStart", skip_serializing_if = "Option::is_none")]
    pub signed_start: Option<String>,
    #[serde(rename = "signedExpiry", skip_serializing_if = "Option::is_none")]
    pub signed_expiry: Option<String>,
    #[serde(rename = "signedIdentifier", skip_serializing_if = "Option::is_none")]
    pub signed_identifier: Option<String>,
    #[serde(rename = "startPk", skip_serializing_if = "Option::is_none")]
    pub start_pk: Option<String>,
    #[serde(rename = "endPk", skip_serializing_if = "Option::is_none")]
    pub end_pk: Option<String>,
    #[serde(rename = "startRk", skip_serializing_if = "Option::is_none")]
    pub start_rk: Option<String>,
    #[serde(rename = "endRk", skip_serializing_if = "Option::is_none")]
    pub end_rk: Option<String>,
    #[serde(rename = "keyToSign", skip_serializing_if = "Option::is_none")]
    pub key_to_sign: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rscc: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rscd: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rsce: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rscl: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rsct: Option<String>,
}
mod service_sas_parameters {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum SignedResource {
        #[serde(rename = "b")]
        B,
        #[serde(rename = "c")]
        C,
        #[serde(rename = "f")]
        F,
        #[serde(rename = "s")]
        S,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum SignedPermission {
        #[serde(rename = "r")]
        R,
        #[serde(rename = "d")]
        D,
        #[serde(rename = "w")]
        W,
        #[serde(rename = "l")]
        L,
        #[serde(rename = "a")]
        A,
        #[serde(rename = "c")]
        C,
        #[serde(rename = "u")]
        U,
        #[serde(rename = "p")]
        P,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum SignedProtocol {
        #[serde(rename = "https,http")]
        HttpsHttp,
        #[serde(rename = "https")]
        Https,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ListServiceSasResponse {
    #[serde(rename = "serviceSasToken", skip_serializing)]
    pub service_sas_token: Option<String>,
}
