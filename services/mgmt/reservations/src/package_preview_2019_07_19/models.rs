#![doc = "generated by AutoRust 0.1.0"]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CurrentQuotaLimit {
    #[serde(rename = "quotaInformation", skip_serializing_if = "Option::is_none")]
    pub quota_information: Option<CurrentQuotaLimitBase>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<QuotaRequestStatusDetails>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CurrentQuotaLimitBase {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<QuotaProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct QuotaProperties {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    #[serde(rename = "currentValue", skip_serializing)]
    pub current_value: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<ResourceName>,
    #[serde(rename = "resourceType", skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<ResourceTypesName>,
    #[serde(rename = "quotaPeriod", skip_serializing)]
    pub quota_period: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResourceName {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(rename = "localizedValue", skip_serializing)]
    pub localized_value: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct QuotaLimits {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<CurrentQuotaLimitBase>,
    #[serde(rename = "nextLink", skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct QuotaLimitsResponse {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<CurrentQuotaLimit>,
    #[serde(rename = "nextLink", skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateGenericQuotaRequestParameters {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<CurrentQuotaLimitBase>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SubRequest {
    #[serde(skip_serializing)]
    pub limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<ResourceName>,
    #[serde(rename = "resourceType", skip_serializing)]
    pub resource_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
    #[serde(rename = "provisioningState", skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<QuotaRequestState>,
    #[serde(skip_serializing)]
    pub message: Option<String>,
    #[serde(rename = "subRequestId", skip_serializing)]
    pub sub_request_id: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct QuotaRequestOneResourceSubmitResponse {
    #[serde(skip_serializing)]
    pub id: Option<String>,
    #[serde(skip_serializing)]
    pub name: Option<String>,
    #[serde(rename = "type", skip_serializing)]
    pub type_: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<QuotaRequestOneResourceProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct QuotaRequestSubmitResponse {
    #[serde(skip_serializing)]
    pub id: Option<String>,
    #[serde(skip_serializing)]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<QuotaRequestProperties>,
    #[serde(rename = "type", skip_serializing)]
    pub type_: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct QuotaRequestSubmitResponse201 {
    #[serde(skip_serializing)]
    pub id: Option<String>,
    #[serde(skip_serializing)]
    pub name: Option<String>,
    #[serde(rename = "type", skip_serializing)]
    pub type_: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<QuotaRequestStatusDetails>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct QuotaRequestStatusDetails {
    #[serde(rename = "provisioningState", skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<QuotaRequestState>,
    #[serde(skip_serializing)]
    pub message: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct QuotaRequestDetails {
    #[serde(skip_serializing)]
    pub id: Option<String>,
    #[serde(skip_serializing)]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<QuotaRequestProperties>,
    #[serde(rename = "type", skip_serializing)]
    pub type_: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct QuotaRequestDetailsList {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<QuotaRequestDetails>,
    #[serde(rename = "nextLink", skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct QuotaRequestProperties {
    #[serde(rename = "provisioningState", skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<QuotaRequestState>,
    #[serde(skip_serializing)]
    pub message: Option<String>,
    #[serde(rename = "requestSubmitTime", skip_serializing)]
    pub request_submit_time: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<SubRequest>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct QuotaRequestOneResourceProperties {
    #[serde(rename = "provisioningState", skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<QuotaRequestState>,
    #[serde(skip_serializing)]
    pub message: Option<String>,
    #[serde(rename = "requestSubmitTime", skip_serializing)]
    pub request_submit_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<CurrentQuotaLimitBase>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum QuotaRequestState {
    Accepted,
    Invalid,
    Succeeded,
    Failed,
    InProgress,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ResourceTypesName {
    #[serde(rename = "standard")]
    Standard,
    #[serde(rename = "dedicated")]
    Dedicated,
    #[serde(rename = "lowPriority")]
    LowPriority,
    #[serde(rename = "shared")]
    Shared,
    #[serde(rename = "serviceSpecific")]
    ServiceSpecific,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AutoQuotaIncreaseDetail {
    #[serde(skip_serializing)]
    pub id: Option<String>,
    #[serde(skip_serializing)]
    pub name: Option<String>,
    #[serde(rename = "type", skip_serializing)]
    pub type_: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<AutoQuotaIncreaseSettings>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AutoQuotaIncreaseSettings {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings: Option<AqiSettings>,
    #[serde(rename = "onFailure", skip_serializing_if = "Option::is_none")]
    pub on_failure: Option<Actions>,
    #[serde(rename = "onSuccess", skip_serializing_if = "Option::is_none")]
    pub on_success: Option<Actions>,
    #[serde(rename = "supportTicketAction", skip_serializing_if = "Option::is_none")]
    pub support_ticket_action: Option<SupportRequestAction>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AqiSettings {
    #[serde(rename = "autoQuotaIncreaseState", skip_serializing_if = "Option::is_none")]
    pub auto_quota_increase_state: Option<AqiState>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum SupportContactTypes {
    #[serde(rename = "email")]
    Email,
    #[serde(rename = "phone")]
    Phone,
    #[serde(rename = "chat")]
    Chat,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SupportRequestAction {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub severity: Option<SeverityTypes>,
    #[serde(rename = "firstName", skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    #[serde(rename = "lastName", skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    #[serde(rename = "phoneNumber", skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<String>,
    #[serde(rename = "primaryEmailAddress", skip_serializing_if = "Option::is_none")]
    pub primary_email_address: Option<String>,
    #[serde(rename = "supportLanguage", skip_serializing_if = "Option::is_none")]
    pub support_language: Option<String>,
    #[serde(rename = "preferredContactMethod", skip_serializing_if = "Option::is_none")]
    pub preferred_contact_method: Option<ContactMethod>,
    #[serde(rename = "alternateEmailAddresses", skip_serializing_if = "Vec::is_empty")]
    pub alternate_email_addresses: Vec<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum SeverityTypes {
    Critical,
    Moderate,
    Minimal,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ContactMethod {
    Email,
    Phone,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum AqiState {
    #[serde(rename = "enabled")]
    Enabled,
    #[serde(rename = "disabled")]
    Disabled,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PhoneAction {
    #[serde(rename = "phoneNumber", skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<String>,
    #[serde(rename = "preferredChannel", skip_serializing_if = "Option::is_none")]
    pub preferred_channel: Option<ContactMethod>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EmailAction {
    #[serde(rename = "emailAddress", skip_serializing_if = "Option::is_none")]
    pub email_address: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EmailActions {
    #[serde(rename = "emailAddresses", skip_serializing_if = "Vec::is_empty")]
    pub email_addresses: Vec<EmailAction>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Actions {
    #[serde(rename = "emailActions", skip_serializing_if = "Option::is_none")]
    pub email_actions: Option<EmailActions>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ExceptionResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<ServiceError>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServiceError {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(skip_serializing)]
    pub details: Vec<ServiceErrorDetail>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServiceErrorDetail {
    #[serde(skip_serializing)]
    pub code: Option<String>,
    #[serde(skip_serializing)]
    pub message: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ReservationStatusCode {
    None,
    Pending,
    Active,
    PurchaseError,
    PaymentInstrumentError,
    Split,
    Merged,
    Expired,
    Succeeded,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ErrorResponseCode {
    NotSpecified,
    InternalServerError,
    ServerTimeout,
    AuthorizationFailed,
    BadRequest,
    ClientCertificateThumbprintNotSet,
    InvalidRequestContent,
    OperationFailed,
    HttpMethodNotSupported,
    InvalidRequestUri,
    MissingTenantId,
    InvalidTenantId,
    InvalidReservationOrderId,
    InvalidReservationId,
    ReservationIdNotInReservationOrder,
    ReservationOrderNotFound,
    InvalidSubscriptionId,
    InvalidAccessToken,
    InvalidLocationId,
    UnauthenticatedRequestsThrottled,
    InvalidHealthCheckType,
    Forbidden,
    BillingScopeIdCannotBeChanged,
    AppliedScopesNotAssociatedWithCommerceAccount,
    PatchValuesSameAsExisting,
    RoleAssignmentCreationFailed,
    ReservationOrderCreationFailed,
    ReservationOrderNotEnabled,
    CapacityUpdateScopesFailed,
    UnsupportedReservationTerm,
    ReservationOrderIdAlreadyExists,
    RiskCheckFailed,
    CreateQuoteFailed,
    ActivateQuoteFailed,
    NonsupportedAccountId,
    PaymentInstrumentNotFound,
    MissingAppliedScopesForSingle,
    NoValidReservationsToReRate,
    #[serde(rename = "ReRateOnlyAllowedForEA")]
    ReRateOnlyAllowedForEa,
    OperationCannotBePerformedInCurrentState,
    InvalidSingleAppliedScopesCount,
    InvalidFulfillmentRequestParameters,
    NotSupportedCountry,
    InvalidRefundQuantity,
    PurchaseError,
    BillingCustomerInputError,
    BillingPaymentInstrumentSoftError,
    BillingPaymentInstrumentHardError,
    BillingTransientError,
    BillingError,
    FulfillmentConfigurationError,
    FulfillmentOutOfStockError,
    FulfillmentTransientError,
    FulfillmentError,
    CalculatePriceFailed,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SkuName {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Catalog {
    #[serde(rename = "resourceType", skip_serializing)]
    pub resource_type: Option<String>,
    #[serde(skip_serializing)]
    pub name: Option<String>,
    #[serde(rename = "billingPlans", skip_serializing_if = "Option::is_none")]
    pub billing_plans: Option<serde_json::Value>,
    #[serde(skip_serializing)]
    pub terms: Vec<ReservationTerm>,
    #[serde(skip_serializing)]
    pub locations: Vec<String>,
    #[serde(rename = "skuProperties", skip_serializing)]
    pub sku_properties: Vec<SkuProperty>,
    #[serde(skip_serializing)]
    pub restrictions: Vec<SkuRestriction>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SkuProperty {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SkuRestriction {
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub values: Vec<String>,
    #[serde(rename = "reasonCode", skip_serializing_if = "Option::is_none")]
    pub reason_code: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReservationOrderResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub etag: Option<i64>,
    #[serde(skip_serializing)]
    pub id: Option<String>,
    #[serde(skip_serializing)]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<ReservationOrderProperties>,
    #[serde(rename = "type", skip_serializing)]
    pub type_: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ReservationBillingPlan {
    Upfront,
    Monthly,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ReservationTerm {
    #[serde(rename = "P1Y")]
    P1y,
    #[serde(rename = "P3Y")]
    P3y,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum PaymentStatus {
    Succeeded,
    Failed,
    Scheduled,
    Cancelled,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PaymentDetail {
    #[serde(rename = "dueDate", skip_serializing_if = "Option::is_none")]
    pub due_date: Option<String>,
    #[serde(rename = "paymentDate", skip_serializing_if = "Option::is_none")]
    pub payment_date: Option<String>,
    #[serde(rename = "pricingCurrencyTotal", skip_serializing_if = "Option::is_none")]
    pub pricing_currency_total: Option<Price>,
    #[serde(rename = "billingCurrencyTotal", skip_serializing_if = "Option::is_none")]
    pub billing_currency_total: Option<Price>,
    #[serde(rename = "billingAccount", skip_serializing_if = "Option::is_none")]
    pub billing_account: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<PaymentStatus>,
    #[serde(rename = "extendedStatusInfo", skip_serializing_if = "Option::is_none")]
    pub extended_status_info: Option<ExtendedStatusInfo>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReservationOrderBillingPlanInformation {
    #[serde(rename = "pricingCurrencyTotal", skip_serializing_if = "Option::is_none")]
    pub pricing_currency_total: Option<Price>,
    #[serde(rename = "startDate", skip_serializing_if = "Option::is_none")]
    pub start_date: Option<String>,
    #[serde(rename = "nextPaymentDueDate", skip_serializing_if = "Option::is_none")]
    pub next_payment_due_date: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub transactions: Vec<PaymentDetail>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReservationOrderProperties {
    #[serde(rename = "displayName", skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(rename = "requestDateTime", skip_serializing_if = "Option::is_none")]
    pub request_date_time: Option<String>,
    #[serde(rename = "createdDateTime", skip_serializing_if = "Option::is_none")]
    pub created_date_time: Option<String>,
    #[serde(rename = "expiryDate", skip_serializing_if = "Option::is_none")]
    pub expiry_date: Option<String>,
    #[serde(rename = "originalQuantity", skip_serializing_if = "Option::is_none")]
    pub original_quantity: Option<ReservationQuantity>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub term: Option<ReservationTerm>,
    #[serde(rename = "provisioningState", skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
    #[serde(rename = "billingPlan", skip_serializing_if = "Option::is_none")]
    pub billing_plan: Option<ReservationBillingPlan>,
    #[serde(rename = "planInformation", skip_serializing_if = "Option::is_none")]
    pub plan_information: Option<ReservationOrderBillingPlanInformation>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub reservations: Vec<ReservationResponse>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReservationResponse {
    #[serde(skip_serializing)]
    pub location: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub etag: Option<i64>,
    #[serde(skip_serializing)]
    pub id: Option<String>,
    #[serde(skip_serializing)]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sku: Option<SkuName>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<ReservationProperties>,
    #[serde(rename = "type", skip_serializing)]
    pub type_: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RenewPropertiesResponse {
    #[serde(rename = "purchaseProperties", skip_serializing_if = "Option::is_none")]
    pub purchase_properties: Option<PurchaseRequest>,
    #[serde(rename = "pricingCurrencyTotal", skip_serializing_if = "Option::is_none")]
    pub pricing_currency_total: Option<renew_properties_response::PricingCurrencyTotal>,
    #[serde(rename = "billingCurrencyTotal", skip_serializing_if = "Option::is_none")]
    pub billing_currency_total: Option<renew_properties_response::BillingCurrencyTotal>,
}
mod renew_properties_response {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct PricingCurrencyTotal {
        #[serde(rename = "currencyCode", skip_serializing_if = "Option::is_none")]
        pub currency_code: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub amount: Option<f64>,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct BillingCurrencyTotal {
        #[serde(rename = "currencyCode", skip_serializing_if = "Option::is_none")]
        pub currency_code: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub amount: Option<f64>,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CalculatePriceResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<CalculatePriceResponseProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CalculatePriceResponseProperties {
    #[serde(rename = "billingCurrencyTotal", skip_serializing_if = "Option::is_none")]
    pub billing_currency_total: Option<calculate_price_response_properties::BillingCurrencyTotal>,
    #[serde(rename = "isBillingPartnerManaged", skip_serializing_if = "Option::is_none")]
    pub is_billing_partner_managed: Option<bool>,
    #[serde(rename = "reservationOrderId", skip_serializing_if = "Option::is_none")]
    pub reservation_order_id: Option<String>,
    #[serde(rename = "skuTitle", skip_serializing_if = "Option::is_none")]
    pub sku_title: Option<String>,
    #[serde(rename = "skuDescription", skip_serializing_if = "Option::is_none")]
    pub sku_description: Option<String>,
    #[serde(rename = "pricingCurrencyTotal", skip_serializing_if = "Option::is_none")]
    pub pricing_currency_total: Option<calculate_price_response_properties::PricingCurrencyTotal>,
    #[serde(rename = "paymentSchedule", skip_serializing_if = "Vec::is_empty")]
    pub payment_schedule: Vec<PaymentDetail>,
}
mod calculate_price_response_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct BillingCurrencyTotal {
        #[serde(rename = "currencyCode", skip_serializing_if = "Option::is_none")]
        pub currency_code: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub amount: Option<f64>,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct PricingCurrencyTotal {
        #[serde(rename = "currencyCode", skip_serializing_if = "Option::is_none")]
        pub currency_code: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub amount: Option<f64>,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReservationProperties {
    #[serde(rename = "reservedResourceType", skip_serializing_if = "Option::is_none")]
    pub reserved_resource_type: Option<ReservedResourceType>,
    #[serde(rename = "instanceFlexibility", skip_serializing_if = "Option::is_none")]
    pub instance_flexibility: Option<InstanceFlexibility>,
    #[serde(rename = "displayName", skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(rename = "appliedScopes", skip_serializing_if = "Option::is_none")]
    pub applied_scopes: Option<AppliedScopes>,
    #[serde(rename = "appliedScopeType", skip_serializing_if = "Option::is_none")]
    pub applied_scope_type: Option<AppliedScopeType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<ReservationQuantity>,
    #[serde(rename = "provisioningState", skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
    #[serde(rename = "effectiveDateTime", skip_serializing_if = "Option::is_none")]
    pub effective_date_time: Option<String>,
    #[serde(rename = "lastUpdatedDateTime", skip_serializing)]
    pub last_updated_date_time: Option<String>,
    #[serde(rename = "expiryDate", skip_serializing_if = "Option::is_none")]
    pub expiry_date: Option<String>,
    #[serde(rename = "skuDescription", skip_serializing_if = "Option::is_none")]
    pub sku_description: Option<String>,
    #[serde(rename = "extendedStatusInfo", skip_serializing_if = "Option::is_none")]
    pub extended_status_info: Option<ExtendedStatusInfo>,
    #[serde(rename = "billingPlan", skip_serializing_if = "Option::is_none")]
    pub billing_plan: Option<ReservationBillingPlan>,
    #[serde(rename = "splitProperties", skip_serializing_if = "Option::is_none")]
    pub split_properties: Option<ReservationSplitProperties>,
    #[serde(rename = "mergeProperties", skip_serializing_if = "Option::is_none")]
    pub merge_properties: Option<ReservationMergeProperties>,
    #[serde(rename = "billingScopeId", skip_serializing_if = "Option::is_none")]
    pub billing_scope_id: Option<BillingScopeId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub renew: Option<Renew>,
    #[serde(rename = "renewSource", skip_serializing_if = "Option::is_none")]
    pub renew_source: Option<String>,
    #[serde(rename = "renewDestination", skip_serializing_if = "Option::is_none")]
    pub renew_destination: Option<String>,
    #[serde(rename = "renewProperties", skip_serializing_if = "Option::is_none")]
    pub renew_properties: Option<RenewPropertiesResponse>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub term: Option<ReservationTerm>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReservationSplitProperties {
    #[serde(rename = "splitDestinations", skip_serializing_if = "Vec::is_empty")]
    pub split_destinations: Vec<String>,
    #[serde(rename = "splitSource", skip_serializing_if = "Option::is_none")]
    pub split_source: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReservationMergeProperties {
    #[serde(rename = "mergeDestination", skip_serializing_if = "Option::is_none")]
    pub merge_destination: Option<String>,
    #[serde(rename = "mergeSources", skip_serializing_if = "Vec::is_empty")]
    pub merge_sources: Vec<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PurchaseRequestProperties {
    #[serde(rename = "reservedResourceType", skip_serializing_if = "Option::is_none")]
    pub reserved_resource_type: Option<ReservedResourceType>,
    #[serde(rename = "billingScopeId", skip_serializing_if = "Option::is_none")]
    pub billing_scope_id: Option<BillingScopeId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub term: Option<ReservationTerm>,
    #[serde(rename = "billingPlan", skip_serializing_if = "Option::is_none")]
    pub billing_plan: Option<ReservationBillingPlan>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<ReservationQuantity>,
    #[serde(rename = "displayName", skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(rename = "appliedScopeType", skip_serializing_if = "Option::is_none")]
    pub applied_scope_type: Option<AppliedScopeType>,
    #[serde(rename = "appliedScopes", skip_serializing_if = "Option::is_none")]
    pub applied_scopes: Option<AppliedScopes>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub renew: Option<Renew>,
    #[serde(rename = "reservedResourceProperties", skip_serializing_if = "Option::is_none")]
    pub reserved_resource_properties: Option<purchase_request_properties::ReservedResourceProperties>,
}
mod purchase_request_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct ReservedResourceProperties {
        #[serde(rename = "instanceFlexibility", skip_serializing_if = "Option::is_none")]
        pub instance_flexibility: Option<InstanceFlexibility>,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PatchProperties {
    #[serde(rename = "appliedScopeType", skip_serializing_if = "Option::is_none")]
    pub applied_scope_type: Option<AppliedScopeType>,
    #[serde(rename = "appliedScopes", skip_serializing_if = "Option::is_none")]
    pub applied_scopes: Option<AppliedScopes>,
    #[serde(rename = "instanceFlexibility", skip_serializing_if = "Option::is_none")]
    pub instance_flexibility: Option<InstanceFlexibility>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub renew: Option<Renew>,
    #[serde(rename = "renewProperties", skip_serializing_if = "Option::is_none")]
    pub renew_properties: Option<patch_properties::RenewProperties>,
}
mod patch_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct RenewProperties {
        #[serde(rename = "purchaseProperties", skip_serializing_if = "Option::is_none")]
        pub purchase_properties: Option<PurchaseRequest>,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SplitProperties {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub quantities: Vec<i64>,
    #[serde(rename = "reservationId", skip_serializing_if = "Option::is_none")]
    pub reservation_id: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MergeProperties {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub sources: Vec<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MergeRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<MergeProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PurchaseRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sku: Option<SkuName>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<PurchaseRequestProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Patch {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<PatchProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SplitRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<SplitProperties>,
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
pub struct ExtendedStatusInfo {
    #[serde(rename = "statusCode", skip_serializing_if = "Option::is_none")]
    pub status_code: Option<ReservationStatusCode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReservationOrderList {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ReservationOrderResponse>,
    #[serde(rename = "nextLink", skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReservationList {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ReservationResponse>,
    #[serde(rename = "nextLink", skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AppliedReservations {
    #[serde(skip_serializing)]
    pub id: Option<String>,
    #[serde(skip_serializing)]
    pub name: Option<String>,
    #[serde(rename = "type", skip_serializing)]
    pub type_: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<AppliedReservationsProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AppliedReservationsProperties {
    #[serde(rename = "reservationOrderIds", skip_serializing_if = "Option::is_none")]
    pub reservation_order_ids: Option<AppliedReservationList>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AppliedReservationList {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<String>,
    #[serde(rename = "nextLink", skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum InstanceFlexibility {
    On,
    Off,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum AppliedScopeType {
    Single,
    Shared,
}
pub type AppliedScopes = Vec<String>;
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BillingScopeId {}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Renew {}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReservationQuantity {}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Properties {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<SubscriptionScopeProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SubscriptionScopeProperties {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub scopes: Vec<ScopeProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ScopeProperties {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valid: Option<bool>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ReservedResourceType {
    VirtualMachines,
    SqlDatabases,
    SuseLinux,
    CosmosDb,
    RedHat,
    SqlDataWarehouse,
    VMwareCloudSimple,
    RedHatOsa,
    Databricks,
    AppService,
    ManagedDisk,
    BlockBlob,
    RedisCache,
    AzureDataExplorer,
    MySql,
    MariaDb,
    PostgreSql,
    DedicatedHost,
    SapHana,
    SqlAzureHybridBenefit,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Price {
    #[serde(rename = "currencyCode", skip_serializing_if = "Option::is_none")]
    pub currency_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<f64>,
}
