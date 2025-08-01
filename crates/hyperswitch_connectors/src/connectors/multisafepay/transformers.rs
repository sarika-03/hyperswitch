use common_enums::{enums, AttemptStatus, BankNames};
use common_utils::{
    errors::ParsingError,
    pii::{Email, IpAddress},
    request::Method,
    types::{FloatMajorUnit, MinorUnit},
};
use hyperswitch_domain_models::{
    payment_method_data::{BankRedirectData, PayLaterData, PaymentMethodData, WalletData},
    router_data::{ConnectorAuthType, ErrorResponse, RouterData},
    router_flow_types::refunds::{Execute, RSync},
    router_request_types::ResponseId,
    router_response_types::{
        MandateReference, PaymentsResponseData, RedirectForm, RefundsResponseData,
    },
    types::{self},
};
use hyperswitch_interfaces::{
    consts::{NO_ERROR_CODE, NO_ERROR_MESSAGE},
    errors,
};
use masking::{ExposeInterface, Secret};
use serde::{Deserialize, Serialize};
use url::Url;

use crate::{
    types::{RefundsResponseRouterData, ResponseRouterData},
    utils::{
        self, AddressDetailsData, CardData as _, PaymentsAuthorizeRequestData, RouterData as _,
    },
};

#[derive(Debug, Serialize)]
pub struct MultisafepayRouterData<T> {
    amount: MinorUnit,
    router_data: T,
}

impl<T> From<(MinorUnit, T)> for MultisafepayRouterData<T> {
    fn from((amount, item): (MinorUnit, T)) -> Self {
        Self {
            amount,
            router_data: item,
        }
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Type {
    Direct,
    Redirect,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum Gateway {
    Amex,
    CreditCard,
    Discover,
    Maestro,
    MasterCard,
    Visa,
    Klarna,
    Googlepay,
    Paypal,
    Ideal,
    Giropay,
    Trustly,
    Alipay,
    #[serde(rename = "WECHAT")]
    WeChatPay,
    Eps,
    MbWay,
    #[serde(rename = "DIRECTBANK")]
    Sofort,
}

#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub struct Coupons {
    pub allow: Option<Vec<Secret<String>>>,
}

#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub struct Mistercash {
    pub mobile_pay_button_position: Option<String>,
    pub disable_mobile_pay_button: Option<String>,
    pub qr_only: Option<String>,
    pub qr_size: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "UPPERCASE")]
pub struct Gateways {
    pub mistercash: Option<Mistercash>,
}

#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub struct Settings {
    pub coupons: Option<Coupons>,
    pub gateways: Option<Gateways>,
}

#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub struct PaymentOptions {
    pub notification_url: Option<String>,
    pub notification_method: Option<String>,
    pub redirect_url: String,
    pub cancel_url: String,
    pub close_window: Option<bool>,
    pub settings: Option<Settings>,
    pub template_id: Option<String>,
    pub allowed_countries: Option<Vec<String>>,
}

#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub struct Browser {
    pub javascript_enabled: Option<bool>,
    pub java_enabled: Option<bool>,
    pub cookies_enabled: Option<bool>,
    pub language: Option<String>,
    pub screen_color_depth: Option<i32>,
    pub screen_height: Option<i32>,
    pub screen_width: Option<i32>,
    pub time_zone: Option<i32>,
    pub user_agent: Option<String>,
    pub platform: Option<String>,
}

#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub struct Customer {
    pub browser: Option<Browser>,
    pub locale: Option<String>,
    pub ip_address: Option<Secret<String, IpAddress>>,
    pub forward_ip: Option<Secret<String, IpAddress>>,
    pub first_name: Option<Secret<String>>,
    pub last_name: Option<Secret<String>>,
    pub gender: Option<Secret<String>>,
    pub birthday: Option<Secret<String>>,
    pub address1: Option<Secret<String>>,
    pub address2: Option<Secret<String>>,
    pub house_number: Option<Secret<String>>,
    pub zip_code: Option<Secret<String>>,
    pub city: Option<String>,
    pub state: Option<String>,
    pub country: Option<String>,
    pub phone: Option<Secret<String>>,
    pub email: Option<Email>,
    pub user_agent: Option<String>,
    pub referrer: Option<String>,
    pub reference: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct CardInfo {
    pub card_number: Option<cards::CardNumber>,
    pub card_holder_name: Option<Secret<String>>,
    pub card_expiry_date: Option<Secret<i32>>,
    pub card_cvc: Option<Secret<String>>,
    pub flexible_3d: Option<bool>,
    pub moto: Option<bool>,
    pub term_url: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct GpayInfo {
    pub payment_token: Option<Secret<String>>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct PayLaterInfo {
    pub email: Option<Email>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(untagged)]
pub enum GatewayInfo {
    Card(CardInfo),
    Wallet(WalletInfo),
    PayLater(PayLaterInfo),
    BankRedirect(BankRedirectInfo),
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(untagged)]
pub enum WalletInfo {
    GooglePay(GpayInfo),
    Alipay(AlipayInfo),
    WeChatPay(WeChatPayInfo),
    MbWay(MbWayInfo),
}

#[derive(Debug, Clone, Serialize, Eq, PartialEq)]
pub struct MbWayInfo {}

#[derive(Debug, Clone, Serialize, Eq, PartialEq)]
pub struct WeChatPayInfo {}
#[derive(Debug, Clone, Serialize, Eq, PartialEq)]
pub struct AlipayInfo {}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(untagged)]
pub enum BankRedirectInfo {
    Ideal(IdealInfo),
    Trustly(TrustlyInfo),
    Eps(EpsInfo),
    Sofort(SofortInfo),
}

#[derive(Debug, Clone, Serialize, Eq, PartialEq)]
pub struct SofortInfo {}

#[derive(Debug, Clone, Serialize, Eq, PartialEq)]
pub struct EpsInfo {}
#[derive(Debug, Clone, Serialize, Eq, PartialEq)]
pub struct TrustlyInfo {}
#[derive(Debug, Clone, Serialize, Eq, PartialEq)]
pub struct IdealInfo {
    pub issuer_id: MultisafepayBankNames,
}

#[derive(Debug, Clone, Serialize, Eq, PartialEq)]
pub enum MultisafepayBankNames {
    #[serde(rename = "0031")]
    AbnAmro,
    #[serde(rename = "0761")]
    AsnBank,
    #[serde(rename = "4371")]
    Bunq,
    #[serde(rename = "0721")]
    Ing,
    #[serde(rename = "0801")]
    Knab,
    #[serde(rename = "9926")]
    N26,
    #[serde(rename = "9927")]
    NationaleNederlanden,
    #[serde(rename = "0021")]
    Rabobank,
    #[serde(rename = "0771")]
    Regiobank,
    #[serde(rename = "1099")]
    Revolut,
    #[serde(rename = "0751")]
    SnsBank,
    #[serde(rename = "0511")]
    TriodosBank,
    #[serde(rename = "0161")]
    VanLanschot,
    #[serde(rename = "0806")]
    Yoursafe,
    #[serde(rename = "1235")]
    Handelsbanken,
}

impl TryFrom<&BankNames> for MultisafepayBankNames {
    type Error = error_stack::Report<errors::ConnectorError>;
    fn try_from(bank: &BankNames) -> Result<Self, Self::Error> {
        match bank {
            BankNames::AbnAmro => Ok(Self::AbnAmro),
            BankNames::AsnBank => Ok(Self::AsnBank),
            BankNames::Bunq => Ok(Self::Bunq),
            BankNames::Ing => Ok(Self::Ing),
            BankNames::Knab => Ok(Self::Knab),
            BankNames::N26 => Ok(Self::N26),
            BankNames::NationaleNederlanden => Ok(Self::NationaleNederlanden),
            BankNames::Rabobank => Ok(Self::Rabobank),
            BankNames::Regiobank => Ok(Self::Regiobank),
            BankNames::Revolut => Ok(Self::Revolut),
            BankNames::SnsBank => Ok(Self::SnsBank),
            BankNames::TriodosBank => Ok(Self::TriodosBank),
            BankNames::VanLanschot => Ok(Self::VanLanschot),
            BankNames::Yoursafe => Ok(Self::Yoursafe),
            BankNames::Handelsbanken => Ok(Self::Handelsbanken),
            BankNames::AmericanExpress
            | BankNames::AffinBank
            | BankNames::AgroBank
            | BankNames::AllianceBank
            | BankNames::AmBank
            | BankNames::BankOfAmerica
            | BankNames::BankOfChina
            | BankNames::BankIslam
            | BankNames::BankMuamalat
            | BankNames::BankRakyat
            | BankNames::BankSimpananNasional
            | BankNames::Barclays
            | BankNames::BlikPSP
            | BankNames::CapitalOne
            | BankNames::Chase
            | BankNames::Citi
            | BankNames::CimbBank
            | BankNames::Discover
            | BankNames::NavyFederalCreditUnion
            | BankNames::PentagonFederalCreditUnion
            | BankNames::SynchronyBank
            | BankNames::WellsFargo
            | BankNames::HongLeongBank
            | BankNames::HsbcBank
            | BankNames::KuwaitFinanceHouse
            | BankNames::Moneyou
            | BankNames::ArzteUndApothekerBank
            | BankNames::AustrianAnadiBankAg
            | BankNames::BankAustria
            | BankNames::Bank99Ag
            | BankNames::BankhausCarlSpangler
            | BankNames::BankhausSchelhammerUndSchatteraAg
            | BankNames::BankMillennium
            | BankNames::BankPEKAOSA
            | BankNames::BawagPskAg
            | BankNames::BksBankAg
            | BankNames::BrullKallmusBankAg
            | BankNames::BtvVierLanderBank
            | BankNames::CapitalBankGraweGruppeAg
            | BankNames::CeskaSporitelna
            | BankNames::Dolomitenbank
            | BankNames::EasybankAg
            | BankNames::EPlatbyVUB
            | BankNames::ErsteBankUndSparkassen
            | BankNames::FrieslandBank
            | BankNames::HypoAlpeadriabankInternationalAg
            | BankNames::HypoNoeLbFurNiederosterreichUWien
            | BankNames::HypoOberosterreichSalzburgSteiermark
            | BankNames::HypoTirolBankAg
            | BankNames::HypoVorarlbergBankAg
            | BankNames::HypoBankBurgenlandAktiengesellschaft
            | BankNames::KomercniBanka
            | BankNames::MBank
            | BankNames::MarchfelderBank
            | BankNames::Maybank
            | BankNames::OberbankAg
            | BankNames::OsterreichischeArzteUndApothekerbank
            | BankNames::OcbcBank
            | BankNames::PayWithING
            | BankNames::PlaceZIPKO
            | BankNames::PlatnoscOnlineKartaPlatnicza
            | BankNames::PosojilnicaBankEGen
            | BankNames::PostovaBanka
            | BankNames::PublicBank
            | BankNames::RaiffeisenBankengruppeOsterreich
            | BankNames::RhbBank
            | BankNames::SchelhammerCapitalBankAg
            | BankNames::StandardCharteredBank
            | BankNames::SchoellerbankAg
            | BankNames::SpardaBankWien
            | BankNames::SporoPay
            | BankNames::SantanderPrzelew24
            | BankNames::TatraPay
            | BankNames::Viamo
            | BankNames::VolksbankGruppe
            | BankNames::VolkskreditbankAg
            | BankNames::VrBankBraunau
            | BankNames::UobBank
            | BankNames::PayWithAliorBank
            | BankNames::BankiSpoldzielcze
            | BankNames::PayWithInteligo
            | BankNames::BNPParibasPoland
            | BankNames::BankNowySA
            | BankNames::CreditAgricole
            | BankNames::PayWithBOS
            | BankNames::PayWithCitiHandlowy
            | BankNames::PayWithPlusBank
            | BankNames::ToyotaBank
            | BankNames::VeloBank
            | BankNames::ETransferPocztowy24
            | BankNames::PlusBank
            | BankNames::EtransferPocztowy24
            | BankNames::BankiSpbdzielcze
            | BankNames::BankNowyBfgSa
            | BankNames::GetinBank
            | BankNames::Blik
            | BankNames::NoblePay
            | BankNames::IdeaBank
            | BankNames::EnveloBank
            | BankNames::NestPrzelew
            | BankNames::MbankMtransfer
            | BankNames::Inteligo
            | BankNames::PbacZIpko
            | BankNames::BnpParibas
            | BankNames::BankPekaoSa
            | BankNames::VolkswagenBank
            | BankNames::AliorBank
            | BankNames::Boz
            | BankNames::BangkokBank
            | BankNames::KrungsriBank
            | BankNames::KrungThaiBank
            | BankNames::TheSiamCommercialBank
            | BankNames::KasikornBank
            | BankNames::OpenBankSuccess
            | BankNames::OpenBankFailure
            | BankNames::OpenBankCancelled
            | BankNames::Aib
            | BankNames::BankOfScotland
            | BankNames::DanskeBank
            | BankNames::FirstDirect
            | BankNames::FirstTrust
            | BankNames::Halifax
            | BankNames::Lloyds
            | BankNames::Monzo
            | BankNames::NatWest
            | BankNames::NationwideBank
            | BankNames::RoyalBankOfScotland
            | BankNames::Starling
            | BankNames::TsbBank
            | BankNames::TescoBank
            | BankNames::UlsterBank => Err(Into::into(errors::ConnectorError::NotSupported {
                message: String::from("BankRedirect"),
                connector: "Multisafepay",
            })),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub struct DeliveryObject {
    first_name: Secret<String>,
    last_name: Secret<String>,
    address1: Secret<String>,
    house_number: Secret<String>,
    zip_code: Secret<String>,
    city: String,
    country: api_models::enums::CountryAlpha2,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct DefaultObject {
    shipping_taxed: bool,
    rate: f64,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct TaxObject {
    pub default: DefaultObject,
}

#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct CheckoutOptions {
    pub validate_cart: Option<bool>,
    pub tax_tables: TaxObject,
}

#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct Item {
    pub name: String,
    pub unit_price: FloatMajorUnit,
    pub description: Option<String>,
    pub quantity: i64,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct ShoppingCart {
    pub items: Vec<Item>,
}

#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct MultisafepayPaymentsRequest {
    #[serde(rename = "type")]
    pub payment_type: Type,
    pub gateway: Option<Gateway>,
    pub order_id: String,
    pub currency: String,
    pub amount: MinorUnit,
    pub description: String,
    pub payment_options: Option<PaymentOptions>,
    pub customer: Option<Customer>,
    pub gateway_info: Option<GatewayInfo>,
    pub delivery: Option<DeliveryObject>,
    pub checkout_options: Option<CheckoutOptions>,
    pub shopping_cart: Option<ShoppingCart>,
    pub items: Option<String>,
    pub recurring_model: Option<MandateType>,
    pub recurring_id: Option<Secret<String>>,
    pub capture: Option<String>,
    pub days_active: Option<i32>,
    pub seconds_active: Option<i32>,
    pub var1: Option<String>,
    pub var2: Option<String>,
    pub var3: Option<String>,
}

impl TryFrom<utils::CardIssuer> for Gateway {
    type Error = error_stack::Report<errors::ConnectorError>;
    fn try_from(issuer: utils::CardIssuer) -> Result<Self, Self::Error> {
        match issuer {
            utils::CardIssuer::AmericanExpress => Ok(Self::Amex),
            utils::CardIssuer::Master => Ok(Self::MasterCard),
            utils::CardIssuer::Maestro => Ok(Self::Maestro),
            utils::CardIssuer::Discover => Ok(Self::Discover),
            utils::CardIssuer::Visa => Ok(Self::Visa),
            utils::CardIssuer::DinersClub
            | utils::CardIssuer::JCB
            | utils::CardIssuer::CarteBlanche
            | utils::CardIssuer::CartesBancaires => Err(errors::ConnectorError::NotImplemented(
                utils::get_unimplemented_payment_method_error_message("Multisafe pay"),
            )
            .into()),
        }
    }
}

impl TryFrom<&MultisafepayRouterData<&types::PaymentsAuthorizeRouterData>>
    for MultisafepayPaymentsRequest
{
    type Error = error_stack::Report<errors::ConnectorError>;
    fn try_from(
        item: &MultisafepayRouterData<&types::PaymentsAuthorizeRouterData>,
    ) -> Result<Self, Self::Error> {
        let payment_type = match item.router_data.request.payment_method_data {
            PaymentMethodData::Card(ref _ccard) => Type::Direct,
            PaymentMethodData::MandatePayment => Type::Direct,
            PaymentMethodData::Wallet(ref wallet_data) => match wallet_data {
                WalletData::GooglePay(_) => Type::Direct,
                WalletData::PaypalRedirect(_) => Type::Redirect,
                WalletData::AliPayRedirect(_) => Type::Redirect,
                WalletData::WeChatPayRedirect(_) => Type::Redirect,
                WalletData::MbWayRedirect(_) => Type::Redirect,
                WalletData::AliPayQr(_)
                | WalletData::AliPayHkRedirect(_)
                | WalletData::AmazonPayRedirect(_)
                | WalletData::Paysera(_)
                | WalletData::Skrill(_)
                | WalletData::MomoRedirect(_)
                | WalletData::KakaoPayRedirect(_)
                | WalletData::GoPayRedirect(_)
                | WalletData::GcashRedirect(_)
                | WalletData::ApplePay(_)
                | WalletData::ApplePayRedirect(_)
                | WalletData::ApplePayThirdPartySdk(_)
                | WalletData::DanaRedirect {}
                | WalletData::GooglePayRedirect(_)
                | WalletData::GooglePayThirdPartySdk(_)
                | WalletData::MobilePayRedirect(_)
                | WalletData::PaypalSdk(_)
                | WalletData::Paze(_)
                | WalletData::SamsungPay(_)
                | WalletData::TwintRedirect {}
                | WalletData::VippsRedirect {}
                | WalletData::TouchNGoRedirect(_)
                | WalletData::WeChatPayQr(_)
                | WalletData::CashappQr(_)
                | WalletData::SwishQr(_)
                | WalletData::Mifinity(_)
                | WalletData::RevolutPay(_) => Err(errors::ConnectorError::NotImplemented(
                    utils::get_unimplemented_payment_method_error_message("multisafepay"),
                ))?,
            },
            PaymentMethodData::BankRedirect(ref bank_data) => match bank_data {
                BankRedirectData::Giropay { .. } => Type::Redirect,
                BankRedirectData::Ideal { .. } => Type::Direct,
                BankRedirectData::Trustly { .. } => Type::Redirect,
                BankRedirectData::Eps { .. } => Type::Redirect,
                BankRedirectData::Sofort { .. } => Type::Redirect,
                BankRedirectData::BancontactCard { .. }
                | BankRedirectData::Bizum { .. }
                | BankRedirectData::Blik { .. }
                | BankRedirectData::Eft { .. }
                | BankRedirectData::Interac { .. }
                | BankRedirectData::OnlineBankingCzechRepublic { .. }
                | BankRedirectData::OnlineBankingFinland { .. }
                | BankRedirectData::OnlineBankingPoland { .. }
                | BankRedirectData::OnlineBankingSlovakia { .. }
                | BankRedirectData::OpenBankingUk { .. }
                | BankRedirectData::Przelewy24 { .. }
                | BankRedirectData::OnlineBankingFpx { .. }
                | BankRedirectData::OnlineBankingThailand { .. }
                | BankRedirectData::LocalBankRedirect {} => {
                    Err(errors::ConnectorError::NotImplemented(
                        utils::get_unimplemented_payment_method_error_message("multisafepay"),
                    ))?
                }
            },
            PaymentMethodData::PayLater(ref _paylater) => Type::Redirect,
            _ => Type::Redirect,
        };

        let gateway = match item.router_data.request.payment_method_data {
            PaymentMethodData::Card(ref ccard) => {
                Some(Gateway::try_from(ccard.get_card_issuer()?)?)
            }
            PaymentMethodData::Wallet(ref wallet_data) => Some(match wallet_data {
                WalletData::GooglePay(_) => Gateway::Googlepay,
                WalletData::PaypalRedirect(_) => Gateway::Paypal,
                WalletData::AliPayRedirect(_) => Gateway::Alipay,
                WalletData::WeChatPayRedirect(_) => Gateway::WeChatPay,
                WalletData::MbWayRedirect(_) => Gateway::MbWay,
                WalletData::AliPayQr(_)
                | WalletData::AliPayHkRedirect(_)
                | WalletData::AmazonPayRedirect(_)
                | WalletData::Paysera(_)
                | WalletData::Skrill(_)
                | WalletData::MomoRedirect(_)
                | WalletData::KakaoPayRedirect(_)
                | WalletData::GoPayRedirect(_)
                | WalletData::GcashRedirect(_)
                | WalletData::ApplePay(_)
                | WalletData::ApplePayRedirect(_)
                | WalletData::ApplePayThirdPartySdk(_)
                | WalletData::DanaRedirect {}
                | WalletData::GooglePayRedirect(_)
                | WalletData::GooglePayThirdPartySdk(_)
                | WalletData::MobilePayRedirect(_)
                | WalletData::PaypalSdk(_)
                | WalletData::Paze(_)
                | WalletData::SamsungPay(_)
                | WalletData::TwintRedirect {}
                | WalletData::VippsRedirect {}
                | WalletData::TouchNGoRedirect(_)
                | WalletData::WeChatPayQr(_)
                | WalletData::CashappQr(_)
                | WalletData::SwishQr(_)
                | WalletData::Mifinity(_)
                | WalletData::RevolutPay(_) => Err(errors::ConnectorError::NotImplemented(
                    utils::get_unimplemented_payment_method_error_message("multisafepay"),
                ))?,
            }),
            PaymentMethodData::BankRedirect(ref bank_data) => Some(match bank_data {
                BankRedirectData::Giropay { .. } => Gateway::Giropay,
                BankRedirectData::Ideal { .. } => Gateway::Ideal,
                BankRedirectData::Trustly { .. } => Gateway::Trustly,
                BankRedirectData::Eps { .. } => Gateway::Eps,
                BankRedirectData::Sofort { .. } => Gateway::Sofort,
                BankRedirectData::BancontactCard { .. }
                | BankRedirectData::Bizum { .. }
                | BankRedirectData::Blik { .. }
                | BankRedirectData::Eft { .. }
                | BankRedirectData::Interac { .. }
                | BankRedirectData::OnlineBankingCzechRepublic { .. }
                | BankRedirectData::OnlineBankingFinland { .. }
                | BankRedirectData::OnlineBankingPoland { .. }
                | BankRedirectData::OnlineBankingSlovakia { .. }
                | BankRedirectData::OpenBankingUk { .. }
                | BankRedirectData::Przelewy24 { .. }
                | BankRedirectData::OnlineBankingFpx { .. }
                | BankRedirectData::OnlineBankingThailand { .. }
                | BankRedirectData::LocalBankRedirect {} => {
                    Err(errors::ConnectorError::NotImplemented(
                        utils::get_unimplemented_payment_method_error_message("multisafepay"),
                    ))?
                }
            }),
            PaymentMethodData::PayLater(PayLaterData::KlarnaRedirect {}) => Some(Gateway::Klarna),
            PaymentMethodData::MandatePayment => None,
            PaymentMethodData::CardRedirect(_)
            | PaymentMethodData::PayLater(_)
            | PaymentMethodData::BankDebit(_)
            | PaymentMethodData::BankTransfer(_)
            | PaymentMethodData::Crypto(_)
            | PaymentMethodData::Reward
            | PaymentMethodData::RealTimePayment(_)
            | PaymentMethodData::MobilePayment(_)
            | PaymentMethodData::Upi(_)
            | PaymentMethodData::Voucher(_)
            | PaymentMethodData::GiftCard(_)
            | PaymentMethodData::OpenBanking(_)
            | PaymentMethodData::CardToken(_)
            | PaymentMethodData::NetworkToken(_)
            | PaymentMethodData::CardDetailsForNetworkTransactionId(_) => {
                Err(errors::ConnectorError::NotImplemented(
                    utils::get_unimplemented_payment_method_error_message("multisafepay"),
                ))?
            }
        };
        let description = item.router_data.get_description()?;
        let payment_options = PaymentOptions {
            notification_url: None,
            redirect_url: item.router_data.request.get_router_return_url()?,
            cancel_url: item.router_data.request.get_router_return_url()?,
            close_window: None,
            notification_method: None,
            settings: None,
            template_id: None,
            allowed_countries: None,
        };

        let customer = Customer {
            browser: None,
            locale: None,
            ip_address: None,
            forward_ip: None,
            first_name: None,
            last_name: None,
            gender: None,
            birthday: None,
            address1: None,
            address2: None,
            house_number: None,
            zip_code: None,
            city: None,
            state: None,
            country: None,
            phone: None,
            email: item.router_data.request.email.clone(),
            user_agent: None,
            referrer: None,
            reference: Some(item.router_data.connector_request_reference_id.clone()),
        };

        let billing_address = item
            .router_data
            .get_billing()?
            .address
            .as_ref()
            .ok_or_else(utils::missing_field_err("billing.address"))?;
        let first_name = billing_address.get_first_name()?;
        let delivery = DeliveryObject {
            first_name: first_name.clone(),
            last_name: billing_address
                .get_last_name()
                .unwrap_or(first_name)
                .clone(),
            address1: billing_address.get_line1()?.to_owned(),
            house_number: billing_address.get_line2()?.to_owned(),
            zip_code: billing_address.get_zip()?.to_owned(),
            city: billing_address.get_city()?.to_owned(),
            country: billing_address.get_country()?.to_owned(),
        };

        let gateway_info = match item.router_data.request.payment_method_data {
            PaymentMethodData::Card(ref ccard) => Some(GatewayInfo::Card(CardInfo {
                card_number: Some(ccard.card_number.clone()),
                card_expiry_date: Some(Secret::new(
                    (format!(
                        "{}{}",
                        ccard.get_card_expiry_year_2_digit()?.expose(),
                        ccard.card_exp_month.clone().expose()
                    ))
                    .parse::<i32>()
                    .unwrap_or_default(),
                )),
                card_cvc: Some(ccard.card_cvc.clone()),
                card_holder_name: None,
                flexible_3d: None,
                moto: None,
                term_url: None,
            })),
            PaymentMethodData::Wallet(ref wallet_data) => match wallet_data {
                WalletData::GooglePay(ref google_pay) => {
                    Some(GatewayInfo::Wallet(WalletInfo::GooglePay({
                        GpayInfo {
                            payment_token: Some(Secret::new(
                                google_pay.tokenization_data.token.clone(),
                            )),
                        }
                    })))
                }
                WalletData::AliPayRedirect(_) => {
                    Some(GatewayInfo::Wallet(WalletInfo::Alipay(AlipayInfo {})))
                }
                WalletData::PaypalRedirect(_) => None,
                WalletData::WeChatPayRedirect(_) => {
                    Some(GatewayInfo::Wallet(WalletInfo::WeChatPay(WeChatPayInfo {})))
                }
                WalletData::MbWayRedirect(_) => {
                    Some(GatewayInfo::Wallet(WalletInfo::MbWay(MbWayInfo {})))
                }
                WalletData::AliPayQr(_)
                | WalletData::AliPayHkRedirect(_)
                | WalletData::AmazonPayRedirect(_)
                | WalletData::Paysera(_)
                | WalletData::Skrill(_)
                | WalletData::MomoRedirect(_)
                | WalletData::KakaoPayRedirect(_)
                | WalletData::GoPayRedirect(_)
                | WalletData::GcashRedirect(_)
                | WalletData::ApplePay(_)
                | WalletData::ApplePayRedirect(_)
                | WalletData::ApplePayThirdPartySdk(_)
                | WalletData::DanaRedirect {}
                | WalletData::GooglePayRedirect(_)
                | WalletData::GooglePayThirdPartySdk(_)
                | WalletData::MobilePayRedirect(_)
                | WalletData::PaypalSdk(_)
                | WalletData::Paze(_)
                | WalletData::SamsungPay(_)
                | WalletData::TwintRedirect {}
                | WalletData::VippsRedirect {}
                | WalletData::TouchNGoRedirect(_)
                | WalletData::WeChatPayQr(_)
                | WalletData::CashappQr(_)
                | WalletData::SwishQr(_)
                | WalletData::Mifinity(_)
                | WalletData::RevolutPay(_) => Err(errors::ConnectorError::NotImplemented(
                    utils::get_unimplemented_payment_method_error_message("multisafepay"),
                ))?,
            },
            PaymentMethodData::PayLater(ref paylater) => {
                Some(GatewayInfo::PayLater(PayLaterInfo {
                    email: Some(match paylater {
                        PayLaterData::KlarnaRedirect {} => item.router_data.get_billing_email()?,
                        PayLaterData::KlarnaSdk { token: _ }
                        | PayLaterData::AffirmRedirect {}
                        | PayLaterData::AfterpayClearpayRedirect {}
                        | PayLaterData::PayBrightRedirect {}
                        | PayLaterData::WalleyRedirect {}
                        | PayLaterData::AlmaRedirect {}
                        | PayLaterData::AtomeRedirect {}
                        | PayLaterData::BreadpayRedirect {} => {
                            Err(errors::ConnectorError::NotImplemented(
                                utils::get_unimplemented_payment_method_error_message(
                                    "multisafepay",
                                ),
                            ))?
                        }
                    }),
                }))
            }
            PaymentMethodData::BankRedirect(ref bank_redirect_data) => match bank_redirect_data {
                BankRedirectData::Ideal { bank_name, .. } => Some(GatewayInfo::BankRedirect(
                    BankRedirectInfo::Ideal(IdealInfo {
                        issuer_id: MultisafepayBankNames::try_from(&bank_name.ok_or(
                            errors::ConnectorError::MissingRequiredField {
                                field_name: "ideal.bank_name",
                            },
                        )?)?,
                    }),
                )),
                BankRedirectData::Trustly { .. } => Some(GatewayInfo::BankRedirect(
                    BankRedirectInfo::Trustly(TrustlyInfo {}),
                )),
                BankRedirectData::Eps { .. } => {
                    Some(GatewayInfo::BankRedirect(BankRedirectInfo::Eps(EpsInfo {})))
                }
                BankRedirectData::Sofort { .. } => Some(GatewayInfo::BankRedirect(
                    BankRedirectInfo::Sofort(SofortInfo {}),
                )),
                BankRedirectData::BancontactCard { .. }
                | BankRedirectData::Bizum { .. }
                | BankRedirectData::Blik { .. }
                | BankRedirectData::Eft { .. }
                | BankRedirectData::Giropay { .. }
                | BankRedirectData::Interac { .. }
                | BankRedirectData::OnlineBankingCzechRepublic { .. }
                | BankRedirectData::OnlineBankingFinland { .. }
                | BankRedirectData::OnlineBankingPoland { .. }
                | BankRedirectData::OnlineBankingSlovakia { .. }
                | BankRedirectData::OpenBankingUk { .. }
                | BankRedirectData::Przelewy24 { .. }
                | BankRedirectData::OnlineBankingFpx { .. }
                | BankRedirectData::OnlineBankingThailand { .. }
                | BankRedirectData::LocalBankRedirect {} => None,
            },
            PaymentMethodData::MandatePayment => None,
            PaymentMethodData::CardRedirect(_)
            | PaymentMethodData::BankDebit(_)
            | PaymentMethodData::BankTransfer(_)
            | PaymentMethodData::Crypto(_)
            | PaymentMethodData::Reward
            | PaymentMethodData::RealTimePayment(_)
            | PaymentMethodData::MobilePayment(_)
            | PaymentMethodData::Upi(_)
            | PaymentMethodData::Voucher(_)
            | PaymentMethodData::GiftCard(_)
            | PaymentMethodData::CardToken(_)
            | PaymentMethodData::OpenBanking(_)
            | PaymentMethodData::NetworkToken(_)
            | PaymentMethodData::CardDetailsForNetworkTransactionId(_) => {
                Err(errors::ConnectorError::NotImplemented(
                    utils::get_unimplemented_payment_method_error_message("multisafepay"),
                ))?
            }
        };

        Ok(Self {
            payment_type,
            gateway,
            order_id: item.router_data.connector_request_reference_id.to_string(),
            currency: item.router_data.request.currency.to_string(),
            amount: item.amount,
            description,
            payment_options: Some(payment_options),
            customer: Some(customer),
            delivery: Some(delivery),
            gateway_info,
            checkout_options: None,
            shopping_cart: None,
            capture: None,
            items: None,
            recurring_model: if item.router_data.request.is_mandate_payment() {
                Some(MandateType::Unscheduled)
            } else {
                None
            },
            recurring_id: item
                .router_data
                .request
                .mandate_id
                .clone()
                .and_then(|mandate_ids| match mandate_ids.mandate_reference_id {
                    Some(api_models::payments::MandateReferenceId::ConnectorMandateId(
                        connector_mandate_ids,
                    )) => connector_mandate_ids
                        .get_connector_mandate_id()
                        .map(Secret::new),
                    _ => None,
                }),
            days_active: Some(30),
            seconds_active: Some(259200),
            var1: None,
            var2: None,
            var3: None,
        })
    }
}

// Auth Struct
pub struct MultisafepayAuthType {
    pub(super) api_key: Secret<String>,
}

impl TryFrom<&ConnectorAuthType> for MultisafepayAuthType {
    type Error = error_stack::Report<errors::ConnectorError>;
    fn try_from(auth_type: &ConnectorAuthType) -> Result<Self, Self::Error> {
        if let ConnectorAuthType::HeaderKey { api_key } = auth_type {
            Ok(Self {
                api_key: api_key.to_owned(),
            })
        } else {
            Err(errors::ConnectorError::FailedToObtainAuthType.into())
        }
    }
}
// PaymentsResponse
#[derive(Debug, Clone, Default, Eq, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum MultisafepayPaymentStatus {
    Completed,
    Declined,
    #[default]
    Initialized,
    Void,
    Uncleared,
}

#[derive(Debug, Clone, Eq, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum MandateType {
    Unscheduled,
}

impl From<MultisafepayPaymentStatus> for AttemptStatus {
    fn from(item: MultisafepayPaymentStatus) -> Self {
        match item {
            MultisafepayPaymentStatus::Completed => Self::Charged,
            MultisafepayPaymentStatus::Declined => Self::Failure,
            MultisafepayPaymentStatus::Initialized => Self::AuthenticationPending,
            MultisafepayPaymentStatus::Uncleared => Self::Pending,
            MultisafepayPaymentStatus::Void => Self::Voided,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub struct Data {
    #[serde(rename = "type")]
    pub payment_type: Option<String>,
    pub order_id: String,
    pub currency: Option<String>,
    pub amount: Option<MinorUnit>,
    pub description: Option<String>,
    pub capture: Option<String>,
    pub payment_url: Option<Url>,
    pub status: Option<MultisafepayPaymentStatus>,
    pub reason: Option<String>,
    pub reason_code: Option<String>,
    pub payment_details: Option<MultisafepayPaymentDetails>,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub struct MultisafepayPaymentDetails {
    pub account_holder_name: Option<Secret<String>>,
    pub account_id: Option<Secret<String>>,
    pub card_expiry_date: Option<Secret<String>>,
    pub external_transaction_id: Option<serde_json::Value>,
    pub last4: Option<Secret<String>>,
    pub recurring_flow: Option<String>,
    pub recurring_id: Option<Secret<String>>,
    pub recurring_model: Option<String>,
    #[serde(rename = "type")]
    pub payment_type: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MultisafepayPaymentsResponse {
    pub success: bool,
    pub data: Data,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(untagged)]
pub enum MultisafepayAuthResponse {
    ErrorResponse(MultisafepayErrorResponse),
    PaymentResponse(Box<MultisafepayPaymentsResponse>),
}

impl<F, T> TryFrom<ResponseRouterData<F, MultisafepayAuthResponse, T, PaymentsResponseData>>
    for RouterData<F, T, PaymentsResponseData>
{
    type Error = error_stack::Report<ParsingError>;
    fn try_from(
        item: ResponseRouterData<F, MultisafepayAuthResponse, T, PaymentsResponseData>,
    ) -> Result<Self, Self::Error> {
        match item.response {
            MultisafepayAuthResponse::PaymentResponse(payment_response) => {
                let redirection_data = payment_response
                    .data
                    .payment_url
                    .clone()
                    .map(|url| RedirectForm::from((url, Method::Get)));

                let default_status = if payment_response.success {
                    MultisafepayPaymentStatus::Initialized
                } else {
                    MultisafepayPaymentStatus::Declined
                };

                let status =
                    AttemptStatus::from(payment_response.data.status.unwrap_or(default_status));

                Ok(Self {
                    status,
                    response: if utils::is_payment_failure(status) {
                        Err(populate_error_reason(
                            payment_response.data.reason_code,
                            payment_response.data.reason.clone(),
                            payment_response.data.reason,
                            item.http_code,
                            Some(status),
                            Some(payment_response.data.order_id),
                        ))
                    } else {
                        Ok(PaymentsResponseData::TransactionResponse {
                            resource_id: ResponseId::ConnectorTransactionId(
                                payment_response.data.order_id.clone(),
                            ),
                            redirection_data: Box::new(redirection_data),
                            mandate_reference: Box::new(
                                payment_response
                                    .data
                                    .payment_details
                                    .and_then(|payment_details| payment_details.recurring_id)
                                    .map(|id| MandateReference {
                                        connector_mandate_id: Some(id.expose()),
                                        payment_method_id: None,
                                        mandate_metadata: None,
                                        connector_mandate_request_reference_id: None,
                                    }),
                            ),
                            connector_metadata: None,
                            network_txn_id: None,
                            connector_response_reference_id: Some(
                                payment_response.data.order_id.clone(),
                            ),
                            incremental_authorization_allowed: None,
                            charges: None,
                        })
                    },
                    ..item.data
                })
            }
            MultisafepayAuthResponse::ErrorResponse(error_response) => {
                let attempt_status = Option::<AttemptStatus>::from(error_response.clone());
                Ok(Self {
                    response: Err(populate_error_reason(
                        Some(error_response.error_code.to_string()),
                        Some(error_response.error_info.clone()),
                        Some(error_response.error_info),
                        item.http_code,
                        attempt_status,
                        None,
                    )),
                    ..item.data
                })
            }
        }
    }
}
pub fn populate_error_reason(
    code: Option<String>,
    message: Option<String>,
    reason: Option<String>,
    http_code: u16,
    attempt_status: Option<AttemptStatus>,
    connector_transaction_id: Option<String>,
) -> ErrorResponse {
    ErrorResponse {
        code: code.unwrap_or(NO_ERROR_CODE.to_string()),
        message: message.clone().unwrap_or(NO_ERROR_MESSAGE.to_string()),
        reason,
        status_code: http_code,
        attempt_status,
        connector_transaction_id,
        network_advice_code: None,
        network_decline_code: None,
        network_error_message: None,
    }
}
// REFUND :
// Type definition for RefundRequest
#[derive(Debug, Serialize)]
pub struct MultisafepayRefundRequest {
    pub currency: enums::Currency,
    pub amount: MinorUnit,
    pub description: Option<String>,
    pub refund_order_id: Option<String>,
    pub checkout_data: Option<ShoppingCart>,
}

impl<F> TryFrom<&MultisafepayRouterData<&types::RefundsRouterData<F>>>
    for MultisafepayRefundRequest
{
    type Error = error_stack::Report<errors::ConnectorError>;
    fn try_from(
        item: &MultisafepayRouterData<&types::RefundsRouterData<F>>,
    ) -> Result<Self, Self::Error> {
        Ok(Self {
            currency: item.router_data.request.currency,
            amount: item.amount,
            description: item.router_data.description.clone(),
            refund_order_id: Some(item.router_data.request.refund_id.clone()),
            checkout_data: None,
        })
    }
}

// Type definition for Refund Response

#[allow(dead_code)]
#[derive(Debug, Serialize, Default, Deserialize, Clone)]
pub enum RefundStatus {
    Succeeded,
    Failed,
    #[default]
    Processing,
}

impl From<RefundStatus> for enums::RefundStatus {
    fn from(item: RefundStatus) -> Self {
        match item {
            RefundStatus::Succeeded => Self::Success,
            RefundStatus::Failed => Self::Failure,
            RefundStatus::Processing => Self::Pending,
        }
    }
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct RefundData {
    pub transaction_id: i64,
    pub refund_id: i64,
    pub order_id: Option<String>,
    pub error_code: Option<i32>,
    pub error_info: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RefundResponse {
    pub success: bool,
    pub data: RefundData,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MultisafepayRefundResponse {
    ErrorResponse(MultisafepayErrorResponse),
    RefundResponse(RefundResponse),
}

impl TryFrom<RefundsResponseRouterData<Execute, MultisafepayRefundResponse>>
    for types::RefundsRouterData<Execute>
{
    type Error = error_stack::Report<ParsingError>;
    fn try_from(
        item: RefundsResponseRouterData<Execute, MultisafepayRefundResponse>,
    ) -> Result<Self, Self::Error> {
        match item.response {
            MultisafepayRefundResponse::RefundResponse(refund_data) => {
                let refund_status = if refund_data.success {
                    RefundStatus::Succeeded
                } else {
                    RefundStatus::Failed
                };

                Ok(Self {
                    response: Ok(RefundsResponseData {
                        connector_refund_id: refund_data.data.refund_id.to_string(),
                        refund_status: enums::RefundStatus::from(refund_status),
                    }),
                    ..item.data
                })
            }
            MultisafepayRefundResponse::ErrorResponse(error_response) => {
                let attempt_status = Option::<AttemptStatus>::from(error_response.clone());
                Ok(Self {
                    response: Err(ErrorResponse {
                        code: error_response.error_code.to_string(),
                        message: error_response.error_info.clone(),
                        reason: Some(error_response.error_info),
                        status_code: item.http_code,
                        attempt_status,
                        connector_transaction_id: None,
                        network_advice_code: None,
                        network_decline_code: None,
                        network_error_message: None,
                    }),
                    ..item.data
                })
            }
        }
    }
}

impl TryFrom<RefundsResponseRouterData<RSync, MultisafepayRefundResponse>>
    for types::RefundsRouterData<RSync>
{
    type Error = error_stack::Report<ParsingError>;
    fn try_from(
        item: RefundsResponseRouterData<RSync, MultisafepayRefundResponse>,
    ) -> Result<Self, Self::Error> {
        match item.response {
            MultisafepayRefundResponse::RefundResponse(refund_data) => {
                let refund_status = if refund_data.success {
                    RefundStatus::Succeeded
                } else {
                    RefundStatus::Failed
                };

                Ok(Self {
                    response: Ok(RefundsResponseData {
                        connector_refund_id: refund_data.data.refund_id.to_string(),
                        refund_status: enums::RefundStatus::from(refund_status),
                    }),
                    ..item.data
                })
            }
            MultisafepayRefundResponse::ErrorResponse(error_response) => Ok(Self {
                response: Err(populate_error_reason(
                    Some(error_response.error_code.to_string()),
                    Some(error_response.error_info.clone()),
                    Some(error_response.error_info),
                    item.http_code,
                    None,
                    None,
                )),
                ..item.data
            }),
        }
    }
}

#[derive(Default, Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct MultisafepayErrorResponse {
    pub error_code: i32,
    pub error_info: String,
}

impl From<MultisafepayErrorResponse> for Option<AttemptStatus> {
    fn from(error_data: MultisafepayErrorResponse) -> Self {
        match error_data.error_code {
            10001 // InvalidAmount
            | 1002 // InvalidCurrency
            | 1003  // InvalidAccountID
            | 1004 // InvalidSiteID
            | 1005 // InvalidSecurityCode
            | 1006 // InvalidTransactionID
            | 1007 // InvalidIPAddress
            | 1008 // InvalidDescription
            | 1010 // InvalidVariable
            | 1011 // InvalidCustomerAccountID
            | 1012 // InvalidCustomerSecurityCode
            | 1013 // InvalidSignature
            | 1015 //UnknownAccountID
            | 1016 // MissingData
            | 1018 // InvalidCountryCode
            | 1025 // MultisafepayErrorCodes::IncorrectCustomerIPAddress
            | 1026 // MultisafepayErrorCodes::MultipleCurrenciesInCart
            | 1027 // MultisafepayErrorCodes::CartCurrencyDifferentToOrderCurrency
            | 1028 // IncorrectCustomTaxRate
            | 1029 // IncorrectItemTaxRate
            | 1030 // IncorrectItemCurrency
            | 1031 // IncorrectItemPrice
            | 1035 // InvalidSignatureRefund
            | 1036 // InvalidIdealIssuerID
            | 5001 // CartDataNotValidated
            | 1032 // InvalidAPIKey
            => {
                Some(AttemptStatus::AuthenticationFailed)
            }

            1034 // CannotRefundTransaction
            | 1022 // CannotInitiateTransaction
            | 1024 //TransactionDeclined
            => Some(AttemptStatus::Failure),
            1017 // InsufficientFunds
            => Some(AttemptStatus::AuthorizationFailed),
            _ => None,
        }
    }
}
