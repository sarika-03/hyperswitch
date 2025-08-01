use std::collections::{HashMap, HashSet};

use common_enums::{AuthenticationConnectors, UIWidgetFormLayout, VaultSdk};
use common_types::primitive_wrappers;
use common_utils::{encryption::Encryption, pii};
use diesel::{AsChangeset, Identifiable, Insertable, Queryable, Selectable};
use masking::Secret;
use time::Duration;

#[cfg(feature = "v1")]
use crate::schema::business_profile;
#[cfg(feature = "v2")]
use crate::schema_v2::business_profile;

/// Note: The order of fields in the struct is important.
/// This should be in the same order as the fields in the schema.rs file, otherwise the code will
/// not compile
/// If two adjacent columns have the same type, then the compiler will not throw any error, but the
/// fields read / written will be interchanged
#[cfg(feature = "v1")]
#[derive(Clone, Debug, Identifiable, Queryable, Selectable, router_derive::DebugAsDisplay)]
#[diesel(table_name = business_profile, primary_key(profile_id), check_for_backend(diesel::pg::Pg))]
pub struct Profile {
    pub profile_id: common_utils::id_type::ProfileId,
    pub merchant_id: common_utils::id_type::MerchantId,
    pub profile_name: String,
    pub created_at: time::PrimitiveDateTime,
    pub modified_at: time::PrimitiveDateTime,
    pub return_url: Option<String>,
    pub enable_payment_response_hash: bool,
    pub payment_response_hash_key: Option<String>,
    pub redirect_to_merchant_with_http_post: bool,
    pub webhook_details: Option<WebhookDetails>,
    pub metadata: Option<pii::SecretSerdeValue>,
    pub routing_algorithm: Option<serde_json::Value>,
    pub intent_fulfillment_time: Option<i64>,
    pub frm_routing_algorithm: Option<serde_json::Value>,
    pub payout_routing_algorithm: Option<serde_json::Value>,
    pub is_recon_enabled: bool,
    #[diesel(deserialize_as = super::OptionalDieselArray<String>)]
    pub applepay_verified_domains: Option<Vec<String>>,
    pub payment_link_config: Option<BusinessPaymentLinkConfig>,
    pub session_expiry: Option<i64>,
    pub authentication_connector_details: Option<AuthenticationConnectorDetails>,
    pub payout_link_config: Option<BusinessPayoutLinkConfig>,
    pub is_extended_card_info_enabled: Option<bool>,
    pub extended_card_info_config: Option<pii::SecretSerdeValue>,
    pub is_connector_agnostic_mit_enabled: Option<bool>,
    pub use_billing_as_payment_method_billing: Option<bool>,
    pub collect_shipping_details_from_wallet_connector: Option<bool>,
    pub collect_billing_details_from_wallet_connector: Option<bool>,
    pub outgoing_webhook_custom_http_headers: Option<Encryption>,
    pub always_collect_billing_details_from_wallet_connector: Option<bool>,
    pub always_collect_shipping_details_from_wallet_connector: Option<bool>,
    pub tax_connector_id: Option<common_utils::id_type::MerchantConnectorAccountId>,
    pub is_tax_connector_enabled: Option<bool>,
    pub version: common_enums::ApiVersion,
    pub dynamic_routing_algorithm: Option<serde_json::Value>,
    pub is_network_tokenization_enabled: bool,
    pub is_auto_retries_enabled: Option<bool>,
    pub max_auto_retries_enabled: Option<i16>,
    pub always_request_extended_authorization:
        Option<primitive_wrappers::AlwaysRequestExtendedAuthorization>,
    pub is_click_to_pay_enabled: bool,
    pub authentication_product_ids:
        Option<common_types::payments::AuthenticationConnectorAccountMap>,
    pub card_testing_guard_config: Option<CardTestingGuardConfig>,
    pub card_testing_secret_key: Option<Encryption>,
    pub is_clear_pan_retries_enabled: bool,
    pub force_3ds_challenge: Option<bool>,
    pub is_debit_routing_enabled: bool,
    pub merchant_business_country: Option<common_enums::CountryAlpha2>,
    pub id: Option<common_utils::id_type::ProfileId>,
    pub is_iframe_redirection_enabled: Option<bool>,
    pub is_pre_network_tokenization_enabled: Option<bool>,
    pub three_ds_decision_rule_algorithm: Option<serde_json::Value>,
    pub acquirer_config_map: Option<common_types::domain::AcquirerConfigMap>,
    pub merchant_category_code: Option<common_enums::MerchantCategoryCode>,
    pub merchant_country_code: Option<common_types::payments::MerchantCountryCode>,
}

#[cfg(feature = "v1")]
#[derive(Clone, Debug, Insertable, router_derive::DebugAsDisplay)]
#[diesel(table_name = business_profile, primary_key(profile_id))]
pub struct ProfileNew {
    pub profile_id: common_utils::id_type::ProfileId,
    pub merchant_id: common_utils::id_type::MerchantId,
    pub profile_name: String,
    pub created_at: time::PrimitiveDateTime,
    pub modified_at: time::PrimitiveDateTime,
    pub return_url: Option<String>,
    pub enable_payment_response_hash: bool,
    pub payment_response_hash_key: Option<String>,
    pub redirect_to_merchant_with_http_post: bool,
    pub webhook_details: Option<WebhookDetails>,
    pub metadata: Option<pii::SecretSerdeValue>,
    pub routing_algorithm: Option<serde_json::Value>,
    pub intent_fulfillment_time: Option<i64>,
    pub frm_routing_algorithm: Option<serde_json::Value>,
    pub payout_routing_algorithm: Option<serde_json::Value>,
    pub is_recon_enabled: bool,
    #[diesel(deserialize_as = super::OptionalDieselArray<String>)]
    pub applepay_verified_domains: Option<Vec<String>>,
    pub payment_link_config: Option<BusinessPaymentLinkConfig>,
    pub session_expiry: Option<i64>,
    pub authentication_connector_details: Option<AuthenticationConnectorDetails>,
    pub payout_link_config: Option<BusinessPayoutLinkConfig>,
    pub is_extended_card_info_enabled: Option<bool>,
    pub extended_card_info_config: Option<pii::SecretSerdeValue>,
    pub is_connector_agnostic_mit_enabled: Option<bool>,
    pub use_billing_as_payment_method_billing: Option<bool>,
    pub collect_shipping_details_from_wallet_connector: Option<bool>,
    pub collect_billing_details_from_wallet_connector: Option<bool>,
    pub outgoing_webhook_custom_http_headers: Option<Encryption>,
    pub always_collect_billing_details_from_wallet_connector: Option<bool>,
    pub always_collect_shipping_details_from_wallet_connector: Option<bool>,
    pub tax_connector_id: Option<common_utils::id_type::MerchantConnectorAccountId>,
    pub is_tax_connector_enabled: Option<bool>,
    pub version: common_enums::ApiVersion,
    pub is_network_tokenization_enabled: bool,
    pub is_auto_retries_enabled: Option<bool>,
    pub max_auto_retries_enabled: Option<i16>,
    pub is_click_to_pay_enabled: bool,
    pub authentication_product_ids:
        Option<common_types::payments::AuthenticationConnectorAccountMap>,
    pub card_testing_guard_config: Option<CardTestingGuardConfig>,
    pub card_testing_secret_key: Option<Encryption>,
    pub is_clear_pan_retries_enabled: bool,
    pub force_3ds_challenge: Option<bool>,
    pub is_debit_routing_enabled: bool,
    pub merchant_business_country: Option<common_enums::CountryAlpha2>,
    pub id: Option<common_utils::id_type::ProfileId>,
    pub is_iframe_redirection_enabled: Option<bool>,
    pub is_pre_network_tokenization_enabled: Option<bool>,
    pub merchant_category_code: Option<common_enums::MerchantCategoryCode>,
    pub merchant_country_code: Option<common_types::payments::MerchantCountryCode>,
}

#[cfg(feature = "v1")]
#[derive(Clone, Debug, AsChangeset, router_derive::DebugAsDisplay)]
#[diesel(table_name = business_profile)]
pub struct ProfileUpdateInternal {
    pub profile_name: Option<String>,
    pub modified_at: time::PrimitiveDateTime,
    pub return_url: Option<String>,
    pub enable_payment_response_hash: Option<bool>,
    pub payment_response_hash_key: Option<String>,
    pub redirect_to_merchant_with_http_post: Option<bool>,
    pub webhook_details: Option<WebhookDetails>,
    pub metadata: Option<pii::SecretSerdeValue>,
    pub routing_algorithm: Option<serde_json::Value>,
    pub intent_fulfillment_time: Option<i64>,
    pub frm_routing_algorithm: Option<serde_json::Value>,
    pub payout_routing_algorithm: Option<serde_json::Value>,
    pub is_recon_enabled: Option<bool>,
    #[diesel(deserialize_as = super::OptionalDieselArray<String>)]
    pub applepay_verified_domains: Option<Vec<String>>,
    pub payment_link_config: Option<BusinessPaymentLinkConfig>,
    pub session_expiry: Option<i64>,
    pub authentication_connector_details: Option<AuthenticationConnectorDetails>,
    pub payout_link_config: Option<BusinessPayoutLinkConfig>,
    pub is_extended_card_info_enabled: Option<bool>,
    pub extended_card_info_config: Option<pii::SecretSerdeValue>,
    pub is_connector_agnostic_mit_enabled: Option<bool>,
    pub use_billing_as_payment_method_billing: Option<bool>,
    pub collect_shipping_details_from_wallet_connector: Option<bool>,
    pub collect_billing_details_from_wallet_connector: Option<bool>,
    pub outgoing_webhook_custom_http_headers: Option<Encryption>,
    pub always_collect_billing_details_from_wallet_connector: Option<bool>,
    pub always_collect_shipping_details_from_wallet_connector: Option<bool>,
    pub tax_connector_id: Option<common_utils::id_type::MerchantConnectorAccountId>,
    pub is_tax_connector_enabled: Option<bool>,
    pub dynamic_routing_algorithm: Option<serde_json::Value>,
    pub is_network_tokenization_enabled: Option<bool>,
    pub is_auto_retries_enabled: Option<bool>,
    pub max_auto_retries_enabled: Option<i16>,
    pub always_request_extended_authorization:
        Option<primitive_wrappers::AlwaysRequestExtendedAuthorization>,
    pub is_click_to_pay_enabled: Option<bool>,
    pub authentication_product_ids:
        Option<common_types::payments::AuthenticationConnectorAccountMap>,
    pub card_testing_guard_config: Option<CardTestingGuardConfig>,
    pub card_testing_secret_key: Option<Encryption>,
    pub is_clear_pan_retries_enabled: Option<bool>,
    pub force_3ds_challenge: Option<bool>,
    pub is_debit_routing_enabled: Option<bool>,
    pub merchant_business_country: Option<common_enums::CountryAlpha2>,
    pub is_iframe_redirection_enabled: Option<bool>,
    pub is_pre_network_tokenization_enabled: Option<bool>,
    pub three_ds_decision_rule_algorithm: Option<serde_json::Value>,
    pub acquirer_config_map: Option<common_types::domain::AcquirerConfigMap>,
    pub merchant_category_code: Option<common_enums::MerchantCategoryCode>,
    pub merchant_country_code: Option<common_types::payments::MerchantCountryCode>,
}

#[cfg(feature = "v1")]
impl ProfileUpdateInternal {
    pub fn apply_changeset(self, source: Profile) -> Profile {
        let Self {
            profile_name,
            modified_at,
            return_url,
            enable_payment_response_hash,
            payment_response_hash_key,
            redirect_to_merchant_with_http_post,
            webhook_details,
            metadata,
            routing_algorithm,
            intent_fulfillment_time,
            frm_routing_algorithm,
            payout_routing_algorithm,
            is_recon_enabled,
            applepay_verified_domains,
            payment_link_config,
            session_expiry,
            authentication_connector_details,
            payout_link_config,
            is_extended_card_info_enabled,
            extended_card_info_config,
            is_connector_agnostic_mit_enabled,
            use_billing_as_payment_method_billing,
            collect_shipping_details_from_wallet_connector,
            collect_billing_details_from_wallet_connector,
            outgoing_webhook_custom_http_headers,
            always_collect_billing_details_from_wallet_connector,
            always_collect_shipping_details_from_wallet_connector,
            tax_connector_id,
            is_tax_connector_enabled,
            dynamic_routing_algorithm,
            is_network_tokenization_enabled,
            is_auto_retries_enabled,
            max_auto_retries_enabled,
            always_request_extended_authorization,
            is_click_to_pay_enabled,
            authentication_product_ids,
            card_testing_guard_config,
            card_testing_secret_key,
            is_clear_pan_retries_enabled,
            force_3ds_challenge,
            is_debit_routing_enabled,
            merchant_business_country,
            is_iframe_redirection_enabled,
            is_pre_network_tokenization_enabled,
            three_ds_decision_rule_algorithm,
            acquirer_config_map,
            merchant_category_code,
            merchant_country_code,
        } = self;
        Profile {
            profile_id: source.profile_id,
            merchant_id: source.merchant_id,
            profile_name: profile_name.unwrap_or(source.profile_name),
            created_at: source.created_at,
            modified_at,
            return_url: return_url.or(source.return_url),
            enable_payment_response_hash: enable_payment_response_hash
                .unwrap_or(source.enable_payment_response_hash),
            payment_response_hash_key: payment_response_hash_key
                .or(source.payment_response_hash_key),
            redirect_to_merchant_with_http_post: redirect_to_merchant_with_http_post
                .unwrap_or(source.redirect_to_merchant_with_http_post),
            webhook_details: webhook_details.or(source.webhook_details),
            metadata: metadata.or(source.metadata),
            routing_algorithm: routing_algorithm.or(source.routing_algorithm),
            intent_fulfillment_time: intent_fulfillment_time.or(source.intent_fulfillment_time),
            frm_routing_algorithm: frm_routing_algorithm.or(source.frm_routing_algorithm),
            payout_routing_algorithm: payout_routing_algorithm.or(source.payout_routing_algorithm),
            is_recon_enabled: is_recon_enabled.unwrap_or(source.is_recon_enabled),
            applepay_verified_domains: applepay_verified_domains
                .or(source.applepay_verified_domains),
            payment_link_config: payment_link_config.or(source.payment_link_config),
            session_expiry: session_expiry.or(source.session_expiry),
            authentication_connector_details: authentication_connector_details
                .or(source.authentication_connector_details),
            payout_link_config: payout_link_config.or(source.payout_link_config),
            is_extended_card_info_enabled: is_extended_card_info_enabled
                .or(source.is_extended_card_info_enabled),
            is_connector_agnostic_mit_enabled: is_connector_agnostic_mit_enabled
                .or(source.is_connector_agnostic_mit_enabled),
            extended_card_info_config: extended_card_info_config
                .or(source.extended_card_info_config),
            use_billing_as_payment_method_billing: use_billing_as_payment_method_billing
                .or(source.use_billing_as_payment_method_billing),
            collect_shipping_details_from_wallet_connector:
                collect_shipping_details_from_wallet_connector
                    .or(source.collect_shipping_details_from_wallet_connector),
            collect_billing_details_from_wallet_connector:
                collect_billing_details_from_wallet_connector
                    .or(source.collect_billing_details_from_wallet_connector),
            outgoing_webhook_custom_http_headers: outgoing_webhook_custom_http_headers
                .or(source.outgoing_webhook_custom_http_headers),
            always_collect_billing_details_from_wallet_connector:
                always_collect_billing_details_from_wallet_connector
                    .or(source.always_collect_billing_details_from_wallet_connector),
            always_collect_shipping_details_from_wallet_connector:
                always_collect_shipping_details_from_wallet_connector
                    .or(source.always_collect_shipping_details_from_wallet_connector),
            tax_connector_id: tax_connector_id.or(source.tax_connector_id),
            is_tax_connector_enabled: is_tax_connector_enabled.or(source.is_tax_connector_enabled),
            version: source.version,
            dynamic_routing_algorithm: dynamic_routing_algorithm
                .or(source.dynamic_routing_algorithm),
            is_network_tokenization_enabled: is_network_tokenization_enabled
                .unwrap_or(source.is_network_tokenization_enabled),
            is_auto_retries_enabled: is_auto_retries_enabled.or(source.is_auto_retries_enabled),
            max_auto_retries_enabled: max_auto_retries_enabled.or(source.max_auto_retries_enabled),
            always_request_extended_authorization: always_request_extended_authorization
                .or(source.always_request_extended_authorization),
            is_click_to_pay_enabled: is_click_to_pay_enabled
                .unwrap_or(source.is_click_to_pay_enabled),
            authentication_product_ids: authentication_product_ids
                .or(source.authentication_product_ids),
            card_testing_guard_config: card_testing_guard_config
                .or(source.card_testing_guard_config),
            card_testing_secret_key,
            is_clear_pan_retries_enabled: is_clear_pan_retries_enabled
                .unwrap_or(source.is_clear_pan_retries_enabled),
            force_3ds_challenge,
            id: source.id,
            is_debit_routing_enabled: is_debit_routing_enabled
                .unwrap_or(source.is_debit_routing_enabled),
            merchant_business_country: merchant_business_country
                .or(source.merchant_business_country),
            is_iframe_redirection_enabled: is_iframe_redirection_enabled
                .or(source.is_iframe_redirection_enabled),
            is_pre_network_tokenization_enabled: is_pre_network_tokenization_enabled
                .or(source.is_pre_network_tokenization_enabled),
            three_ds_decision_rule_algorithm: three_ds_decision_rule_algorithm
                .or(source.three_ds_decision_rule_algorithm),
            acquirer_config_map: acquirer_config_map.or(source.acquirer_config_map),
            merchant_category_code: merchant_category_code.or(source.merchant_category_code),
            merchant_country_code: merchant_country_code.or(source.merchant_country_code),
        }
    }
}

/// Note: The order of fields in the struct is important.
/// This should be in the same order as the fields in the schema.rs file, otherwise the code will
/// not compile
/// If two adjacent columns have the same type, then the compiler will not throw any error, but the
/// fields read / written will be interchanged
#[cfg(feature = "v2")]
#[derive(Clone, Debug, Identifiable, Queryable, Selectable, router_derive::DebugAsDisplay)]
#[diesel(table_name = business_profile, primary_key(id), check_for_backend(diesel::pg::Pg))]
pub struct Profile {
    pub merchant_id: common_utils::id_type::MerchantId,
    pub profile_name: String,
    pub created_at: time::PrimitiveDateTime,
    pub modified_at: time::PrimitiveDateTime,
    pub return_url: Option<common_utils::types::Url>,
    pub enable_payment_response_hash: bool,
    pub payment_response_hash_key: Option<String>,
    pub redirect_to_merchant_with_http_post: bool,
    pub webhook_details: Option<WebhookDetails>,
    pub metadata: Option<pii::SecretSerdeValue>,
    pub is_recon_enabled: bool,
    #[diesel(deserialize_as = super::OptionalDieselArray<String>)]
    pub applepay_verified_domains: Option<Vec<String>>,
    pub payment_link_config: Option<BusinessPaymentLinkConfig>,
    pub session_expiry: Option<i64>,
    pub authentication_connector_details: Option<AuthenticationConnectorDetails>,
    pub payout_link_config: Option<BusinessPayoutLinkConfig>,
    pub is_extended_card_info_enabled: Option<bool>,
    pub extended_card_info_config: Option<pii::SecretSerdeValue>,
    pub is_connector_agnostic_mit_enabled: Option<bool>,
    pub use_billing_as_payment_method_billing: Option<bool>,
    pub collect_shipping_details_from_wallet_connector: Option<bool>,
    pub collect_billing_details_from_wallet_connector: Option<bool>,
    pub outgoing_webhook_custom_http_headers: Option<Encryption>,
    pub always_collect_billing_details_from_wallet_connector: Option<bool>,
    pub always_collect_shipping_details_from_wallet_connector: Option<bool>,
    pub tax_connector_id: Option<common_utils::id_type::MerchantConnectorAccountId>,
    pub is_tax_connector_enabled: Option<bool>,
    pub version: common_enums::ApiVersion,
    pub dynamic_routing_algorithm: Option<serde_json::Value>,
    pub is_network_tokenization_enabled: bool,
    pub is_auto_retries_enabled: Option<bool>,
    pub max_auto_retries_enabled: Option<i16>,
    pub always_request_extended_authorization:
        Option<primitive_wrappers::AlwaysRequestExtendedAuthorization>,
    pub is_click_to_pay_enabled: bool,
    pub authentication_product_ids:
        Option<common_types::payments::AuthenticationConnectorAccountMap>,
    pub card_testing_guard_config: Option<CardTestingGuardConfig>,
    pub card_testing_secret_key: Option<Encryption>,
    pub is_clear_pan_retries_enabled: bool,
    pub force_3ds_challenge: Option<bool>,
    pub is_debit_routing_enabled: bool,
    pub merchant_business_country: Option<common_enums::CountryAlpha2>,
    pub id: common_utils::id_type::ProfileId,
    pub is_iframe_redirection_enabled: Option<bool>,
    pub three_ds_decision_rule_algorithm: Option<serde_json::Value>,
    pub acquirer_config_map: Option<common_types::domain::AcquirerConfigMap>,
    pub merchant_category_code: Option<common_enums::MerchantCategoryCode>,
    pub merchant_country_code: Option<common_types::payments::MerchantCountryCode>,
    pub routing_algorithm_id: Option<common_utils::id_type::RoutingId>,
    pub order_fulfillment_time: Option<i64>,
    pub order_fulfillment_time_origin: Option<common_enums::OrderFulfillmentTimeOrigin>,
    pub frm_routing_algorithm_id: Option<String>,
    pub payout_routing_algorithm_id: Option<common_utils::id_type::RoutingId>,
    pub default_fallback_routing: Option<pii::SecretSerdeValue>,
    pub three_ds_decision_manager_config: Option<common_types::payments::DecisionManagerRecord>,
    pub should_collect_cvv_during_payment:
        Option<primitive_wrappers::ShouldCollectCvvDuringPayment>,
    pub is_external_vault_enabled: Option<bool>,
    pub external_vault_connector_details: Option<ExternalVaultConnectorDetails>,
    pub revenue_recovery_retry_algorithm_type: Option<common_enums::RevenueRecoveryAlgorithmType>,
    pub revenue_recovery_retry_algorithm_data: Option<RevenueRecoveryAlgorithmData>,
}

impl Profile {
    #[cfg(feature = "v1")]
    pub fn get_id(&self) -> &common_utils::id_type::ProfileId {
        &self.profile_id
    }

    #[cfg(feature = "v2")]
    pub fn get_id(&self) -> &common_utils::id_type::ProfileId {
        &self.id
    }
}

#[cfg(feature = "v2")]
#[derive(Clone, Debug, Insertable, router_derive::DebugAsDisplay)]
#[diesel(table_name = business_profile, primary_key(profile_id))]
pub struct ProfileNew {
    pub merchant_id: common_utils::id_type::MerchantId,
    pub profile_name: String,
    pub created_at: time::PrimitiveDateTime,
    pub modified_at: time::PrimitiveDateTime,
    pub return_url: Option<common_utils::types::Url>,
    pub enable_payment_response_hash: bool,
    pub payment_response_hash_key: Option<String>,
    pub redirect_to_merchant_with_http_post: bool,
    pub webhook_details: Option<WebhookDetails>,
    pub metadata: Option<pii::SecretSerdeValue>,
    pub is_recon_enabled: bool,
    #[diesel(deserialize_as = super::OptionalDieselArray<String>)]
    pub applepay_verified_domains: Option<Vec<String>>,
    pub payment_link_config: Option<BusinessPaymentLinkConfig>,
    pub session_expiry: Option<i64>,
    pub authentication_connector_details: Option<AuthenticationConnectorDetails>,
    pub payout_link_config: Option<BusinessPayoutLinkConfig>,
    pub is_extended_card_info_enabled: Option<bool>,
    pub extended_card_info_config: Option<pii::SecretSerdeValue>,
    pub is_connector_agnostic_mit_enabled: Option<bool>,
    pub use_billing_as_payment_method_billing: Option<bool>,
    pub collect_shipping_details_from_wallet_connector: Option<bool>,
    pub collect_billing_details_from_wallet_connector: Option<bool>,
    pub outgoing_webhook_custom_http_headers: Option<Encryption>,
    pub always_collect_billing_details_from_wallet_connector: Option<bool>,
    pub always_collect_shipping_details_from_wallet_connector: Option<bool>,
    pub tax_connector_id: Option<common_utils::id_type::MerchantConnectorAccountId>,
    pub is_tax_connector_enabled: Option<bool>,
    pub version: common_enums::ApiVersion,
    pub is_network_tokenization_enabled: bool,
    pub is_auto_retries_enabled: Option<bool>,
    pub max_auto_retries_enabled: Option<i16>,
    pub is_click_to_pay_enabled: bool,
    pub authentication_product_ids:
        Option<common_types::payments::AuthenticationConnectorAccountMap>,
    pub card_testing_guard_config: Option<CardTestingGuardConfig>,
    pub card_testing_secret_key: Option<Encryption>,
    pub is_clear_pan_retries_enabled: Option<bool>,
    pub is_debit_routing_enabled: bool,
    pub merchant_business_country: Option<common_enums::CountryAlpha2>,
    pub merchant_category_code: Option<common_enums::MerchantCategoryCode>,
    pub merchant_country_code: Option<common_types::payments::MerchantCountryCode>,
    pub routing_algorithm_id: Option<common_utils::id_type::RoutingId>,
    pub order_fulfillment_time: Option<i64>,
    pub order_fulfillment_time_origin: Option<common_enums::OrderFulfillmentTimeOrigin>,
    pub frm_routing_algorithm_id: Option<String>,
    pub payout_routing_algorithm_id: Option<common_utils::id_type::RoutingId>,
    pub default_fallback_routing: Option<pii::SecretSerdeValue>,
    pub three_ds_decision_manager_config: Option<common_types::payments::DecisionManagerRecord>,
    pub should_collect_cvv_during_payment:
        Option<primitive_wrappers::ShouldCollectCvvDuringPayment>,
    pub id: common_utils::id_type::ProfileId,
    pub revenue_recovery_retry_algorithm_type: Option<common_enums::RevenueRecoveryAlgorithmType>,
    pub revenue_recovery_retry_algorithm_data: Option<RevenueRecoveryAlgorithmData>,
    pub is_iframe_redirection_enabled: Option<bool>,
    pub is_external_vault_enabled: Option<bool>,
    pub external_vault_connector_details: Option<ExternalVaultConnectorDetails>,
}

#[cfg(feature = "v2")]
#[derive(Clone, Debug, AsChangeset, router_derive::DebugAsDisplay)]
#[diesel(table_name = business_profile)]
pub struct ProfileUpdateInternal {
    pub profile_name: Option<String>,
    pub modified_at: time::PrimitiveDateTime,
    pub return_url: Option<common_utils::types::Url>,
    pub enable_payment_response_hash: Option<bool>,
    pub payment_response_hash_key: Option<String>,
    pub redirect_to_merchant_with_http_post: Option<bool>,
    pub webhook_details: Option<WebhookDetails>,
    pub metadata: Option<pii::SecretSerdeValue>,
    pub is_recon_enabled: Option<bool>,
    #[diesel(deserialize_as = super::OptionalDieselArray<String>)]
    pub applepay_verified_domains: Option<Vec<String>>,
    pub payment_link_config: Option<BusinessPaymentLinkConfig>,
    pub session_expiry: Option<i64>,
    pub authentication_connector_details: Option<AuthenticationConnectorDetails>,
    pub payout_link_config: Option<BusinessPayoutLinkConfig>,
    pub is_extended_card_info_enabled: Option<bool>,
    pub extended_card_info_config: Option<pii::SecretSerdeValue>,
    pub is_connector_agnostic_mit_enabled: Option<bool>,
    pub use_billing_as_payment_method_billing: Option<bool>,
    pub collect_shipping_details_from_wallet_connector: Option<bool>,
    pub collect_billing_details_from_wallet_connector: Option<bool>,
    pub outgoing_webhook_custom_http_headers: Option<Encryption>,
    pub always_collect_billing_details_from_wallet_connector: Option<bool>,
    pub always_collect_shipping_details_from_wallet_connector: Option<bool>,
    pub tax_connector_id: Option<common_utils::id_type::MerchantConnectorAccountId>,
    pub is_tax_connector_enabled: Option<bool>,
    pub is_network_tokenization_enabled: Option<bool>,
    pub is_auto_retries_enabled: Option<bool>,
    pub max_auto_retries_enabled: Option<i16>,
    pub is_click_to_pay_enabled: Option<bool>,
    pub authentication_product_ids:
        Option<common_types::payments::AuthenticationConnectorAccountMap>,
    pub card_testing_guard_config: Option<CardTestingGuardConfig>,
    pub card_testing_secret_key: Option<Encryption>,
    pub is_clear_pan_retries_enabled: Option<bool>,
    pub is_debit_routing_enabled: Option<bool>,
    pub merchant_business_country: Option<common_enums::CountryAlpha2>,
    pub merchant_category_code: Option<common_enums::MerchantCategoryCode>,
    pub merchant_country_code: Option<common_types::payments::MerchantCountryCode>,
    pub routing_algorithm_id: Option<common_utils::id_type::RoutingId>,
    pub order_fulfillment_time: Option<i64>,
    pub order_fulfillment_time_origin: Option<common_enums::OrderFulfillmentTimeOrigin>,
    pub frm_routing_algorithm_id: Option<String>,
    pub payout_routing_algorithm_id: Option<common_utils::id_type::RoutingId>,
    pub default_fallback_routing: Option<pii::SecretSerdeValue>,
    pub three_ds_decision_manager_config: Option<common_types::payments::DecisionManagerRecord>,
    pub should_collect_cvv_during_payment:
        Option<primitive_wrappers::ShouldCollectCvvDuringPayment>,
    pub revenue_recovery_retry_algorithm_type: Option<common_enums::RevenueRecoveryAlgorithmType>,
    pub revenue_recovery_retry_algorithm_data: Option<RevenueRecoveryAlgorithmData>,
    pub is_iframe_redirection_enabled: Option<bool>,
    pub is_external_vault_enabled: Option<bool>,
    pub external_vault_connector_details: Option<ExternalVaultConnectorDetails>,
}

#[cfg(feature = "v2")]
impl ProfileUpdateInternal {
    pub fn apply_changeset(self, source: Profile) -> Profile {
        let Self {
            profile_name,
            modified_at,
            return_url,
            enable_payment_response_hash,
            payment_response_hash_key,
            redirect_to_merchant_with_http_post,
            webhook_details,
            metadata,
            is_recon_enabled,
            applepay_verified_domains,
            payment_link_config,
            session_expiry,
            authentication_connector_details,
            payout_link_config,
            is_extended_card_info_enabled,
            extended_card_info_config,
            is_connector_agnostic_mit_enabled,
            use_billing_as_payment_method_billing,
            collect_shipping_details_from_wallet_connector,
            collect_billing_details_from_wallet_connector,
            outgoing_webhook_custom_http_headers,
            always_collect_billing_details_from_wallet_connector,
            always_collect_shipping_details_from_wallet_connector,
            tax_connector_id,
            is_tax_connector_enabled,
            routing_algorithm_id,
            order_fulfillment_time,
            order_fulfillment_time_origin,
            frm_routing_algorithm_id,
            payout_routing_algorithm_id,
            default_fallback_routing,
            should_collect_cvv_during_payment,
            is_network_tokenization_enabled,
            is_auto_retries_enabled,
            max_auto_retries_enabled,
            is_click_to_pay_enabled,
            authentication_product_ids,
            three_ds_decision_manager_config,
            card_testing_guard_config,
            card_testing_secret_key,
            is_clear_pan_retries_enabled,
            is_debit_routing_enabled,
            merchant_business_country,
            revenue_recovery_retry_algorithm_type,
            revenue_recovery_retry_algorithm_data,
            is_iframe_redirection_enabled,
            is_external_vault_enabled,
            external_vault_connector_details,
            merchant_category_code,
            merchant_country_code,
        } = self;
        Profile {
            id: source.id,
            merchant_id: source.merchant_id,
            profile_name: profile_name.unwrap_or(source.profile_name),
            created_at: source.created_at,
            modified_at,
            return_url: return_url.or(source.return_url),
            enable_payment_response_hash: enable_payment_response_hash
                .unwrap_or(source.enable_payment_response_hash),
            payment_response_hash_key: payment_response_hash_key
                .or(source.payment_response_hash_key),
            redirect_to_merchant_with_http_post: redirect_to_merchant_with_http_post
                .unwrap_or(source.redirect_to_merchant_with_http_post),
            webhook_details: webhook_details.or(source.webhook_details),
            metadata: metadata.or(source.metadata),
            is_recon_enabled: is_recon_enabled.unwrap_or(source.is_recon_enabled),
            applepay_verified_domains: applepay_verified_domains
                .or(source.applepay_verified_domains),
            payment_link_config: payment_link_config.or(source.payment_link_config),
            session_expiry: session_expiry.or(source.session_expiry),
            authentication_connector_details: authentication_connector_details
                .or(source.authentication_connector_details),
            payout_link_config: payout_link_config.or(source.payout_link_config),
            is_extended_card_info_enabled: is_extended_card_info_enabled
                .or(source.is_extended_card_info_enabled),
            is_connector_agnostic_mit_enabled: is_connector_agnostic_mit_enabled
                .or(source.is_connector_agnostic_mit_enabled),
            extended_card_info_config: extended_card_info_config
                .or(source.extended_card_info_config),
            use_billing_as_payment_method_billing: use_billing_as_payment_method_billing
                .or(source.use_billing_as_payment_method_billing),
            collect_shipping_details_from_wallet_connector:
                collect_shipping_details_from_wallet_connector
                    .or(source.collect_shipping_details_from_wallet_connector),
            collect_billing_details_from_wallet_connector:
                collect_billing_details_from_wallet_connector
                    .or(source.collect_billing_details_from_wallet_connector),
            outgoing_webhook_custom_http_headers: outgoing_webhook_custom_http_headers
                .or(source.outgoing_webhook_custom_http_headers),
            always_collect_billing_details_from_wallet_connector:
                always_collect_billing_details_from_wallet_connector
                    .or(always_collect_billing_details_from_wallet_connector),
            always_collect_shipping_details_from_wallet_connector:
                always_collect_shipping_details_from_wallet_connector
                    .or(always_collect_shipping_details_from_wallet_connector),
            tax_connector_id: tax_connector_id.or(source.tax_connector_id),
            is_tax_connector_enabled: is_tax_connector_enabled.or(source.is_tax_connector_enabled),
            routing_algorithm_id: routing_algorithm_id.or(source.routing_algorithm_id),
            order_fulfillment_time: order_fulfillment_time.or(source.order_fulfillment_time),
            order_fulfillment_time_origin: order_fulfillment_time_origin
                .or(source.order_fulfillment_time_origin),
            frm_routing_algorithm_id: frm_routing_algorithm_id.or(source.frm_routing_algorithm_id),
            payout_routing_algorithm_id: payout_routing_algorithm_id
                .or(source.payout_routing_algorithm_id),
            default_fallback_routing: default_fallback_routing.or(source.default_fallback_routing),
            should_collect_cvv_during_payment: should_collect_cvv_during_payment
                .or(source.should_collect_cvv_during_payment),
            version: source.version,
            dynamic_routing_algorithm: None,
            is_network_tokenization_enabled: is_network_tokenization_enabled
                .unwrap_or(source.is_network_tokenization_enabled),
            is_auto_retries_enabled: is_auto_retries_enabled.or(source.is_auto_retries_enabled),
            max_auto_retries_enabled: max_auto_retries_enabled.or(source.max_auto_retries_enabled),
            always_request_extended_authorization: None,
            is_click_to_pay_enabled: is_click_to_pay_enabled
                .unwrap_or(source.is_click_to_pay_enabled),
            authentication_product_ids: authentication_product_ids
                .or(source.authentication_product_ids),
            three_ds_decision_manager_config: three_ds_decision_manager_config
                .or(source.three_ds_decision_manager_config),
            card_testing_guard_config: card_testing_guard_config
                .or(source.card_testing_guard_config),
            card_testing_secret_key: card_testing_secret_key.or(source.card_testing_secret_key),
            is_clear_pan_retries_enabled: is_clear_pan_retries_enabled
                .unwrap_or(source.is_clear_pan_retries_enabled),
            force_3ds_challenge: None,
            is_debit_routing_enabled: is_debit_routing_enabled
                .unwrap_or(source.is_debit_routing_enabled),
            merchant_business_country: merchant_business_country
                .or(source.merchant_business_country),
            revenue_recovery_retry_algorithm_type: revenue_recovery_retry_algorithm_type
                .or(source.revenue_recovery_retry_algorithm_type),
            revenue_recovery_retry_algorithm_data: revenue_recovery_retry_algorithm_data
                .or(source.revenue_recovery_retry_algorithm_data),
            is_iframe_redirection_enabled: is_iframe_redirection_enabled
                .or(source.is_iframe_redirection_enabled),
            is_external_vault_enabled: is_external_vault_enabled
                .or(source.is_external_vault_enabled),
            external_vault_connector_details: external_vault_connector_details
                .or(source.external_vault_connector_details),
            three_ds_decision_rule_algorithm: None,
            acquirer_config_map: None,
            merchant_category_code: merchant_category_code.or(source.merchant_category_code),
            merchant_country_code: merchant_country_code.or(source.merchant_country_code),
        }
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, diesel::AsExpression)]
#[diesel(sql_type = diesel::sql_types::Jsonb)]
pub struct AuthenticationConnectorDetails {
    pub authentication_connectors: Vec<AuthenticationConnectors>,
    pub three_ds_requestor_url: String,
    pub three_ds_requestor_app_url: Option<String>,
}

common_utils::impl_to_sql_from_sql_json!(AuthenticationConnectorDetails);

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, diesel::AsExpression)]
#[diesel(sql_type = diesel::sql_types::Jsonb)]
pub struct ExternalVaultConnectorDetails {
    pub vault_connector_id: common_utils::id_type::MerchantConnectorAccountId,
    pub vault_sdk: Option<VaultSdk>,
}

common_utils::impl_to_sql_from_sql_json!(ExternalVaultConnectorDetails);

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, diesel::AsExpression)]
#[diesel(sql_type = diesel::sql_types::Jsonb)]
pub struct CardTestingGuardConfig {
    pub is_card_ip_blocking_enabled: bool,
    pub card_ip_blocking_threshold: i32,
    pub is_guest_user_card_blocking_enabled: bool,
    pub guest_user_card_blocking_threshold: i32,
    pub is_customer_id_blocking_enabled: bool,
    pub customer_id_blocking_threshold: i32,
    pub card_testing_guard_expiry: i32,
}

common_utils::impl_to_sql_from_sql_json!(CardTestingGuardConfig);

impl Default for CardTestingGuardConfig {
    fn default() -> Self {
        Self {
            is_card_ip_blocking_enabled: common_utils::consts::DEFAULT_CARD_IP_BLOCKING_STATUS,
            card_ip_blocking_threshold: common_utils::consts::DEFAULT_CARD_IP_BLOCKING_THRESHOLD,
            is_guest_user_card_blocking_enabled:
                common_utils::consts::DEFAULT_GUEST_USER_CARD_BLOCKING_STATUS,
            guest_user_card_blocking_threshold:
                common_utils::consts::DEFAULT_GUEST_USER_CARD_BLOCKING_THRESHOLD,
            is_customer_id_blocking_enabled:
                common_utils::consts::DEFAULT_CUSTOMER_ID_BLOCKING_STATUS,
            customer_id_blocking_threshold:
                common_utils::consts::DEFAULT_CUSTOMER_ID_BLOCKING_THRESHOLD,
            card_testing_guard_expiry:
                common_utils::consts::DEFAULT_CARD_TESTING_GUARD_EXPIRY_IN_SECS,
        }
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, diesel::AsExpression)]
#[diesel(sql_type = diesel::sql_types::Json)]
pub struct WebhookDetails {
    pub webhook_version: Option<String>,
    pub webhook_username: Option<String>,
    pub webhook_password: Option<Secret<String>>,
    pub webhook_url: Option<Secret<String>>,
    pub payment_created_enabled: Option<bool>,
    pub payment_succeeded_enabled: Option<bool>,
    pub payment_failed_enabled: Option<bool>,
    pub payment_statuses_enabled: Option<Vec<common_enums::IntentStatus>>,
    pub refund_statuses_enabled: Option<Vec<common_enums::RefundStatus>>,
    pub payout_statuses_enabled: Option<Vec<common_enums::PayoutStatus>>,
}

common_utils::impl_to_sql_from_sql_json!(WebhookDetails);

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, diesel::AsExpression)]
#[diesel(sql_type = diesel::sql_types::Jsonb)]
pub struct BusinessPaymentLinkConfig {
    pub domain_name: Option<String>,
    #[serde(flatten)]
    pub default_config: Option<PaymentLinkConfigRequest>,
    pub business_specific_configs: Option<HashMap<String, PaymentLinkConfigRequest>>,
    pub allowed_domains: Option<HashSet<String>>,
    pub branding_visibility: Option<bool>,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct PaymentLinkConfigRequest {
    pub theme: Option<String>,
    pub logo: Option<String>,
    pub seller_name: Option<String>,
    pub sdk_layout: Option<String>,
    pub display_sdk_only: Option<bool>,
    pub enabled_saved_payment_method: Option<bool>,
    pub hide_card_nickname_field: Option<bool>,
    pub show_card_form_by_default: Option<bool>,
    pub background_image: Option<PaymentLinkBackgroundImageConfig>,
    pub details_layout: Option<common_enums::PaymentLinkDetailsLayout>,
    pub payment_button_text: Option<String>,
    pub custom_message_for_card_terms: Option<String>,
    pub payment_button_colour: Option<String>,
    pub skip_status_screen: Option<bool>,
    pub payment_button_text_colour: Option<String>,
    pub background_colour: Option<String>,
    pub sdk_ui_rules: Option<HashMap<String, HashMap<String, String>>>,
    pub payment_link_ui_rules: Option<HashMap<String, HashMap<String, String>>>,
    pub enable_button_only_on_form_ready: Option<bool>,
    pub payment_form_header_text: Option<String>,
    pub payment_form_label_type: Option<common_enums::PaymentLinkSdkLabelType>,
    pub show_card_terms: Option<common_enums::PaymentLinkShowSdkTerms>,
    pub is_setup_mandate_flow: Option<bool>,
    pub color_icon_card_cvc_error: Option<String>,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, PartialEq)]
pub struct PaymentLinkBackgroundImageConfig {
    pub url: common_utils::types::Url,
    pub position: Option<common_enums::ElementPosition>,
    pub size: Option<common_enums::ElementSize>,
}

common_utils::impl_to_sql_from_sql_json!(BusinessPaymentLinkConfig);

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, diesel::AsExpression)]
#[diesel(sql_type = diesel::sql_types::Jsonb)]
pub struct BusinessPayoutLinkConfig {
    #[serde(flatten)]
    pub config: BusinessGenericLinkConfig,
    pub form_layout: Option<UIWidgetFormLayout>,
    pub payout_test_mode: Option<bool>,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct BusinessGenericLinkConfig {
    pub domain_name: Option<String>,
    pub allowed_domains: HashSet<String>,
    #[serde(flatten)]
    pub ui_config: common_utils::link_utils::GenericLinkUiConfig,
}

common_utils::impl_to_sql_from_sql_json!(BusinessPayoutLinkConfig);

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, diesel::AsExpression)]
#[diesel(sql_type = diesel::sql_types::Jsonb)]
pub struct RevenueRecoveryAlgorithmData {
    pub monitoring_configured_timestamp: time::PrimitiveDateTime,
}

impl RevenueRecoveryAlgorithmData {
    pub fn has_exceeded_monitoring_threshold(&self, monitoring_threshold_in_seconds: i64) -> bool {
        let total_threshold_time = self.monitoring_configured_timestamp
            + Duration::seconds(monitoring_threshold_in_seconds);
        common_utils::date_time::now() >= total_threshold_time
    }
}

common_utils::impl_to_sql_from_sql_json!(RevenueRecoveryAlgorithmData);
