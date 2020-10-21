#![doc = "generated by AutoRust 0.1.0"]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ARecord {
    #[serde(rename = "ipv4Address", skip_serializing_if = "Option::is_none")]
    pub ipv4_address: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AaaaRecord {
    #[serde(rename = "ipv6Address", skip_serializing_if = "Option::is_none")]
    pub ipv6_address: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MxRecord {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preference: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exchange: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NsRecord {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nsdname: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PtrRecord {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ptrdname: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SrvRecord {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weight: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TxtRecord {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CnameRecord {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cname: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SoaRecord {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(rename = "serialNumber", skip_serializing_if = "Option::is_none")]
    pub serial_number: Option<i64>,
    #[serde(rename = "refreshTime", skip_serializing_if = "Option::is_none")]
    pub refresh_time: Option<i64>,
    #[serde(rename = "retryTime", skip_serializing_if = "Option::is_none")]
    pub retry_time: Option<i64>,
    #[serde(rename = "expireTime", skip_serializing_if = "Option::is_none")]
    pub expire_time: Option<i64>,
    #[serde(rename = "minimumTTL", skip_serializing_if = "Option::is_none")]
    pub minimum_ttl: Option<i64>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CaaRecord {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flags: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RecordSetProperties {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
    #[serde(rename = "TTL", skip_serializing_if = "Option::is_none")]
    pub ttl: Option<i64>,
    #[serde(skip_serializing)]
    pub fqdn: Option<String>,
    #[serde(rename = "provisioningState", skip_serializing)]
    pub provisioning_state: Option<String>,
    #[serde(rename = "targetResource", skip_serializing_if = "Option::is_none")]
    pub target_resource: Option<SubResource>,
    #[serde(rename = "ARecords", skip_serializing_if = "Vec::is_empty")]
    pub a_records: Vec<ARecord>,
    #[serde(rename = "AAAARecords", skip_serializing_if = "Vec::is_empty")]
    pub aaaa_records: Vec<AaaaRecord>,
    #[serde(rename = "MXRecords", skip_serializing_if = "Vec::is_empty")]
    pub mx_records: Vec<MxRecord>,
    #[serde(rename = "NSRecords", skip_serializing_if = "Vec::is_empty")]
    pub ns_records: Vec<NsRecord>,
    #[serde(rename = "PTRRecords", skip_serializing_if = "Vec::is_empty")]
    pub ptr_records: Vec<PtrRecord>,
    #[serde(rename = "SRVRecords", skip_serializing_if = "Vec::is_empty")]
    pub srv_records: Vec<SrvRecord>,
    #[serde(rename = "TXTRecords", skip_serializing_if = "Vec::is_empty")]
    pub txt_records: Vec<TxtRecord>,
    #[serde(rename = "CNAMERecord", skip_serializing_if = "Option::is_none")]
    pub cname_record: Option<CnameRecord>,
    #[serde(rename = "SOARecord", skip_serializing_if = "Option::is_none")]
    pub soa_record: Option<SoaRecord>,
    #[serde(rename = "caaRecords", skip_serializing_if = "Vec::is_empty")]
    pub caa_records: Vec<CaaRecord>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RecordSet {
    #[serde(skip_serializing)]
    pub id: Option<String>,
    #[serde(skip_serializing)]
    pub name: Option<String>,
    #[serde(rename = "type", skip_serializing)]
    pub type_: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<RecordSetProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RecordSetUpdateParameters {
    #[serde(rename = "RecordSet", skip_serializing_if = "Option::is_none")]
    pub record_set: Option<RecordSet>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RecordSetListResult {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<RecordSet>,
    #[serde(rename = "nextLink", skip_serializing)]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ZoneProperties {
    #[serde(rename = "maxNumberOfRecordSets", skip_serializing)]
    pub max_number_of_record_sets: Option<i64>,
    #[serde(rename = "numberOfRecordSets", skip_serializing)]
    pub number_of_record_sets: Option<i64>,
    #[serde(rename = "nameServers", skip_serializing)]
    pub name_servers: Vec<String>,
    #[serde(rename = "zoneType", skip_serializing_if = "Option::is_none")]
    pub zone_type: Option<zone_properties::ZoneType>,
    #[serde(rename = "registrationVirtualNetworks", skip_serializing_if = "Vec::is_empty")]
    pub registration_virtual_networks: Vec<SubResource>,
    #[serde(rename = "resolutionVirtualNetworks", skip_serializing_if = "Vec::is_empty")]
    pub resolution_virtual_networks: Vec<SubResource>,
}
mod zone_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ZoneType {
        Public,
        Private,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Zone {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<ZoneProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ZoneUpdate {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ZoneListResult {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Zone>,
    #[serde(rename = "nextLink", skip_serializing)]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DnsResourceReferenceRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<DnsResourceReferenceRequestProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DnsResourceReferenceRequestProperties {
    #[serde(rename = "targetResources", skip_serializing_if = "Vec::is_empty")]
    pub target_resources: Vec<SubResource>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DnsResourceReferenceResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<DnsResourceReferenceResultProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DnsResourceReferenceResultProperties {
    #[serde(rename = "dnsResourceReferences", skip_serializing_if = "Vec::is_empty")]
    pub dns_resource_references: Vec<DnsResourceReference>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DnsResourceReference {
    #[serde(rename = "dnsResources", skip_serializing_if = "Vec::is_empty")]
    pub dns_resources: Vec<SubResource>,
    #[serde(rename = "targetResource", skip_serializing_if = "Option::is_none")]
    pub target_resource: Option<SubResource>,
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
pub struct SubResource {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CloudError {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<CloudErrorBody>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CloudErrorBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub details: Vec<CloudErrorBody>,
}
