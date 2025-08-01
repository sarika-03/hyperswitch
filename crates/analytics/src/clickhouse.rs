use std::sync::Arc;

use actix_web::http::StatusCode;
use common_utils::errors::ParsingError;
use error_stack::{report, Report, ResultExt};
use router_env::logger;
use time::PrimitiveDateTime;

use super::{
    active_payments::metrics::ActivePaymentsMetricRow,
    auth_events::metrics::AuthEventMetricRow,
    frm::{filters::FrmFilterRow, metrics::FrmMetricRow},
    health_check::HealthCheck,
    payment_intents::{filters::PaymentIntentFilterRow, metrics::PaymentIntentMetricRow},
    payments::{
        distribution::PaymentDistributionRow, filters::PaymentFilterRow, metrics::PaymentMetricRow,
    },
    query::{Aggregate, ToSql, Window},
    refunds::{
        distribution::RefundDistributionRow, filters::RefundFilterRow, metrics::RefundMetricRow,
    },
    sdk_events::{filters::SdkEventFilter, metrics::SdkEventMetricRow},
    types::{AnalyticsCollection, AnalyticsDataSource, LoadRow, QueryExecutionError},
};
use crate::{
    api_event::{
        events::ApiLogsResult,
        filters::ApiEventFilter,
        metrics::{latency::LatencyAvg, ApiEventMetricRow},
    },
    auth_events::filters::AuthEventFilterRow,
    connector_events::events::ConnectorEventsResult,
    disputes::{filters::DisputeFilterRow, metrics::DisputeMetricRow},
    outgoing_webhook_event::events::OutgoingWebhookLogsResult,
    routing_events::events::RoutingEventsResult,
    sdk_events::events::SdkEventsResult,
    types::TableEngine,
};

pub type ClickhouseResult<T> = error_stack::Result<T, ClickhouseError>;

#[derive(Clone, Debug)]
pub struct ClickhouseClient {
    pub config: Arc<ClickhouseConfig>,
    pub database: String,
}

#[derive(Clone, Debug, serde::Deserialize)]
pub struct ClickhouseConfig {
    username: String,
    password: Option<String>,
    host: String,
}

impl Default for ClickhouseConfig {
    fn default() -> Self {
        Self {
            username: "default".to_string(),
            password: None,
            host: "http://localhost:8123".to_string(),
        }
    }
}

impl ClickhouseClient {
    async fn execute_query(&self, query: &str) -> ClickhouseResult<Vec<serde_json::Value>> {
        logger::debug!("Executing query: {query}");
        let client = reqwest::Client::new();
        let params = CkhQuery {
            date_time_output_format: String::from("iso"),
            output_format_json_quote_64bit_integers: 0,
            database: self.database.clone(),
        };
        let response = client
            .post(&self.config.host)
            .query(&params)
            .basic_auth(self.config.username.clone(), self.config.password.clone())
            .body(format!("{query}\nFORMAT JSON"))
            .send()
            .await
            .change_context(ClickhouseError::ConnectionError)?;

        logger::debug!(clickhouse_response=?response, query=?query, "Clickhouse response");
        if response.status() != StatusCode::OK {
            response.text().await.map_or_else(
                |er| {
                    Err(ClickhouseError::ResponseError)
                        .attach_printable_lazy(|| format!("Error: {er:?}"))
                },
                |t| Err(report!(ClickhouseError::ResponseNotOK(t))),
            )
        } else {
            Ok(response
                .json::<CkhOutput<serde_json::Value>>()
                .await
                .change_context(ClickhouseError::ResponseError)?
                .data)
        }
    }
}

#[async_trait::async_trait]
impl HealthCheck for ClickhouseClient {
    async fn deep_health_check(
        &self,
    ) -> common_utils::errors::CustomResult<(), QueryExecutionError> {
        self.execute_query("SELECT 1")
            .await
            .map(|_| ())
            .change_context(QueryExecutionError::DatabaseError)
    }
}

#[async_trait::async_trait]
impl AnalyticsDataSource for ClickhouseClient {
    type Row = serde_json::Value;

    async fn load_results<T>(
        &self,
        query: &str,
    ) -> common_utils::errors::CustomResult<Vec<T>, QueryExecutionError>
    where
        Self: LoadRow<T>,
    {
        self.execute_query(query)
            .await
            .change_context(QueryExecutionError::DatabaseError)?
            .into_iter()
            .map(Self::load_row)
            .collect::<Result<Vec<_>, _>>()
            .change_context(QueryExecutionError::RowExtractionFailure)
    }

    fn get_table_engine(table: AnalyticsCollection) -> TableEngine {
        match table {
            AnalyticsCollection::Payment
            | AnalyticsCollection::PaymentSessionized
            | AnalyticsCollection::Refund
            | AnalyticsCollection::RefundSessionized
            | AnalyticsCollection::FraudCheck
            | AnalyticsCollection::PaymentIntent
            | AnalyticsCollection::PaymentIntentSessionized
            | AnalyticsCollection::Authentications
            | AnalyticsCollection::Dispute => {
                TableEngine::CollapsingMergeTree { sign: "sign_flag" }
            }
            AnalyticsCollection::DisputeSessionized => {
                TableEngine::CollapsingMergeTree { sign: "sign_flag" }
            }
            AnalyticsCollection::SdkEvents
            | AnalyticsCollection::SdkEventsAnalytics
            | AnalyticsCollection::ApiEvents
            | AnalyticsCollection::ConnectorEvents
            | AnalyticsCollection::RoutingEvents
            | AnalyticsCollection::ApiEventsAnalytics
            | AnalyticsCollection::OutgoingWebhookEvent
            | AnalyticsCollection::ActivePaymentsAnalytics => TableEngine::BasicTree,
        }
    }
}

impl<T, E> LoadRow<T> for ClickhouseClient
where
    Self::Row: TryInto<T, Error = Report<E>>,
{
    fn load_row(row: Self::Row) -> common_utils::errors::CustomResult<T, QueryExecutionError> {
        row.try_into()
            .map_err(|error| error.change_context(QueryExecutionError::RowExtractionFailure))
    }
}

impl super::payments::filters::PaymentFilterAnalytics for ClickhouseClient {}
impl super::payments::metrics::PaymentMetricAnalytics for ClickhouseClient {}
impl super::payments::distribution::PaymentDistributionAnalytics for ClickhouseClient {}
impl super::payment_intents::filters::PaymentIntentFilterAnalytics for ClickhouseClient {}
impl super::payment_intents::metrics::PaymentIntentMetricAnalytics for ClickhouseClient {}
impl super::refunds::metrics::RefundMetricAnalytics for ClickhouseClient {}
impl super::refunds::filters::RefundFilterAnalytics for ClickhouseClient {}
impl super::refunds::distribution::RefundDistributionAnalytics for ClickhouseClient {}
impl super::frm::metrics::FrmMetricAnalytics for ClickhouseClient {}
impl super::frm::filters::FrmFilterAnalytics for ClickhouseClient {}
impl super::sdk_events::filters::SdkEventFilterAnalytics for ClickhouseClient {}
impl super::sdk_events::metrics::SdkEventMetricAnalytics for ClickhouseClient {}
impl super::sdk_events::events::SdkEventsFilterAnalytics for ClickhouseClient {}
impl super::active_payments::metrics::ActivePaymentsMetricAnalytics for ClickhouseClient {}
impl super::auth_events::metrics::AuthEventMetricAnalytics for ClickhouseClient {}
impl super::auth_events::filters::AuthEventFilterAnalytics for ClickhouseClient {}
impl super::api_event::events::ApiLogsFilterAnalytics for ClickhouseClient {}
impl super::api_event::filters::ApiEventFilterAnalytics for ClickhouseClient {}
impl super::api_event::metrics::ApiEventMetricAnalytics for ClickhouseClient {}
impl super::connector_events::events::ConnectorEventLogAnalytics for ClickhouseClient {}
impl super::routing_events::events::RoutingEventLogAnalytics for ClickhouseClient {}
impl super::outgoing_webhook_event::events::OutgoingWebhookLogsFilterAnalytics
    for ClickhouseClient
{
}
impl super::disputes::filters::DisputeFilterAnalytics for ClickhouseClient {}
impl super::disputes::metrics::DisputeMetricAnalytics for ClickhouseClient {}

#[derive(Debug, serde::Serialize)]
struct CkhQuery {
    date_time_output_format: String,
    output_format_json_quote_64bit_integers: u8,
    database: String,
}

#[derive(Debug, serde::Deserialize)]
struct CkhOutput<T> {
    data: Vec<T>,
}

impl TryInto<ApiLogsResult> for serde_json::Value {
    type Error = Report<ParsingError>;

    fn try_into(self) -> Result<ApiLogsResult, Self::Error> {
        serde_json::from_value(self).change_context(ParsingError::StructParseFailure(
            "Failed to parse ApiLogsResult in clickhouse results",
        ))
    }
}

impl TryInto<SdkEventsResult> for serde_json::Value {
    type Error = Report<ParsingError>;

    fn try_into(self) -> Result<SdkEventsResult, Self::Error> {
        serde_json::from_value(self).change_context(ParsingError::StructParseFailure(
            "Failed to parse SdkEventsResult in clickhouse results",
        ))
    }
}

impl TryInto<ConnectorEventsResult> for serde_json::Value {
    type Error = Report<ParsingError>;

    fn try_into(self) -> Result<ConnectorEventsResult, Self::Error> {
        serde_json::from_value(self).change_context(ParsingError::StructParseFailure(
            "Failed to parse ConnectorEventsResult in clickhouse results",
        ))
    }
}

impl TryInto<RoutingEventsResult> for serde_json::Value {
    type Error = Report<ParsingError>;

    fn try_into(self) -> Result<RoutingEventsResult, Self::Error> {
        serde_json::from_value(self).change_context(ParsingError::StructParseFailure(
            "Failed to parse RoutingEventsResult in clickhouse results",
        ))
    }
}

impl TryInto<PaymentMetricRow> for serde_json::Value {
    type Error = Report<ParsingError>;

    fn try_into(self) -> Result<PaymentMetricRow, Self::Error> {
        serde_json::from_value(self).change_context(ParsingError::StructParseFailure(
            "Failed to parse PaymentMetricRow in clickhouse results",
        ))
    }
}

impl TryInto<PaymentDistributionRow> for serde_json::Value {
    type Error = Report<ParsingError>;

    fn try_into(self) -> Result<PaymentDistributionRow, Self::Error> {
        serde_json::from_value(self).change_context(ParsingError::StructParseFailure(
            "Failed to parse PaymentDistributionRow in clickhouse results",
        ))
    }
}

impl TryInto<PaymentFilterRow> for serde_json::Value {
    type Error = Report<ParsingError>;

    fn try_into(self) -> Result<PaymentFilterRow, Self::Error> {
        serde_json::from_value(self).change_context(ParsingError::StructParseFailure(
            "Failed to parse FilterRow in clickhouse results",
        ))
    }
}

impl TryInto<PaymentIntentMetricRow> for serde_json::Value {
    type Error = Report<ParsingError>;

    fn try_into(self) -> Result<PaymentIntentMetricRow, Self::Error> {
        serde_json::from_value(self).change_context(ParsingError::StructParseFailure(
            "Failed to parse PaymentIntentMetricRow in clickhouse results",
        ))
    }
}

impl TryInto<PaymentIntentFilterRow> for serde_json::Value {
    type Error = Report<ParsingError>;

    fn try_into(self) -> Result<PaymentIntentFilterRow, Self::Error> {
        serde_json::from_value(self).change_context(ParsingError::StructParseFailure(
            "Failed to parse PaymentIntentFilterRow in clickhouse results",
        ))
    }
}

impl TryInto<RefundMetricRow> for serde_json::Value {
    type Error = Report<ParsingError>;

    fn try_into(self) -> Result<RefundMetricRow, Self::Error> {
        serde_json::from_value(self).change_context(ParsingError::StructParseFailure(
            "Failed to parse RefundMetricRow in clickhouse results",
        ))
    }
}

impl TryInto<RefundFilterRow> for serde_json::Value {
    type Error = Report<ParsingError>;

    fn try_into(self) -> Result<RefundFilterRow, Self::Error> {
        serde_json::from_value(self).change_context(ParsingError::StructParseFailure(
            "Failed to parse RefundFilterRow in clickhouse results",
        ))
    }
}

impl TryInto<RefundDistributionRow> for serde_json::Value {
    type Error = Report<ParsingError>;

    fn try_into(self) -> Result<RefundDistributionRow, Self::Error> {
        serde_json::from_value(self).change_context(ParsingError::StructParseFailure(
            "Failed to parse RefundDistributionRow in clickhouse results",
        ))
    }
}

impl TryInto<FrmMetricRow> for serde_json::Value {
    type Error = Report<ParsingError>;

    fn try_into(self) -> Result<FrmMetricRow, Self::Error> {
        serde_json::from_value(self).change_context(ParsingError::StructParseFailure(
            "Failed to parse FrmMetricRow in clickhouse results",
        ))
    }
}

impl TryInto<FrmFilterRow> for serde_json::Value {
    type Error = Report<ParsingError>;

    fn try_into(self) -> Result<FrmFilterRow, Self::Error> {
        serde_json::from_value(self).change_context(ParsingError::StructParseFailure(
            "Failed to parse FrmFilterRow in clickhouse results",
        ))
    }
}
impl TryInto<DisputeMetricRow> for serde_json::Value {
    type Error = Report<ParsingError>;

    fn try_into(self) -> Result<DisputeMetricRow, Self::Error> {
        serde_json::from_value(self).change_context(ParsingError::StructParseFailure(
            "Failed to parse DisputeMetricRow in clickhouse results",
        ))
    }
}

impl TryInto<DisputeFilterRow> for serde_json::Value {
    type Error = Report<ParsingError>;

    fn try_into(self) -> Result<DisputeFilterRow, Self::Error> {
        serde_json::from_value(self).change_context(ParsingError::StructParseFailure(
            "Failed to parse DisputeFilterRow in clickhouse results",
        ))
    }
}

impl TryInto<ApiEventMetricRow> for serde_json::Value {
    type Error = Report<ParsingError>;

    fn try_into(self) -> Result<ApiEventMetricRow, Self::Error> {
        serde_json::from_value(self).change_context(ParsingError::StructParseFailure(
            "Failed to parse ApiEventMetricRow in clickhouse results",
        ))
    }
}

impl TryInto<LatencyAvg> for serde_json::Value {
    type Error = Report<ParsingError>;

    fn try_into(self) -> Result<LatencyAvg, Self::Error> {
        serde_json::from_value(self).change_context(ParsingError::StructParseFailure(
            "Failed to parse LatencyAvg in clickhouse results",
        ))
    }
}

impl TryInto<SdkEventMetricRow> for serde_json::Value {
    type Error = Report<ParsingError>;

    fn try_into(self) -> Result<SdkEventMetricRow, Self::Error> {
        serde_json::from_value(self).change_context(ParsingError::StructParseFailure(
            "Failed to parse SdkEventMetricRow in clickhouse results",
        ))
    }
}

impl TryInto<SdkEventFilter> for serde_json::Value {
    type Error = Report<ParsingError>;

    fn try_into(self) -> Result<SdkEventFilter, Self::Error> {
        serde_json::from_value(self).change_context(ParsingError::StructParseFailure(
            "Failed to parse SdkEventFilter in clickhouse results",
        ))
    }
}

impl TryInto<AuthEventMetricRow> for serde_json::Value {
    type Error = Report<ParsingError>;

    fn try_into(self) -> Result<AuthEventMetricRow, Self::Error> {
        serde_json::from_value(self).change_context(ParsingError::StructParseFailure(
            "Failed to parse AuthEventMetricRow in clickhouse results",
        ))
    }
}

impl TryInto<AuthEventFilterRow> for serde_json::Value {
    type Error = Report<ParsingError>;

    fn try_into(self) -> Result<AuthEventFilterRow, Self::Error> {
        serde_json::from_value(self).change_context(ParsingError::StructParseFailure(
            "Failed to parse AuthEventFilterRow in clickhouse results",
        ))
    }
}

impl TryInto<ApiEventFilter> for serde_json::Value {
    type Error = Report<ParsingError>;

    fn try_into(self) -> Result<ApiEventFilter, Self::Error> {
        serde_json::from_value(self).change_context(ParsingError::StructParseFailure(
            "Failed to parse ApiEventFilter in clickhouse results",
        ))
    }
}

impl TryInto<OutgoingWebhookLogsResult> for serde_json::Value {
    type Error = Report<ParsingError>;

    fn try_into(self) -> Result<OutgoingWebhookLogsResult, Self::Error> {
        serde_json::from_value(self).change_context(ParsingError::StructParseFailure(
            "Failed to parse OutgoingWebhookLogsResult in clickhouse results",
        ))
    }
}

impl TryInto<ActivePaymentsMetricRow> for serde_json::Value {
    type Error = Report<ParsingError>;

    fn try_into(self) -> Result<ActivePaymentsMetricRow, Self::Error> {
        serde_json::from_value(self).change_context(ParsingError::StructParseFailure(
            "Failed to parse ActivePaymentsMetricRow in clickhouse results",
        ))
    }
}

impl ToSql<ClickhouseClient> for PrimitiveDateTime {
    fn to_sql(&self, _table_engine: &TableEngine) -> error_stack::Result<String, ParsingError> {
        Ok(self.assume_utc().unix_timestamp().to_string())
    }
}

impl ToSql<ClickhouseClient> for AnalyticsCollection {
    fn to_sql(&self, _table_engine: &TableEngine) -> error_stack::Result<String, ParsingError> {
        match self {
            Self::Payment => Ok("payment_attempts".to_string()),
            Self::PaymentSessionized => Ok("sessionizer_payment_attempts".to_string()),
            Self::Refund => Ok("refunds".to_string()),
            Self::RefundSessionized => Ok("sessionizer_refunds".to_string()),
            Self::FraudCheck => Ok("fraud_check".to_string()),
            Self::SdkEvents => Ok("sdk_events_audit".to_string()),
            Self::SdkEventsAnalytics => Ok("sdk_events".to_string()),
            Self::ApiEvents => Ok("api_events_audit".to_string()),
            Self::ApiEventsAnalytics => Ok("api_events".to_string()),
            Self::PaymentIntent => Ok("payment_intents".to_string()),
            Self::PaymentIntentSessionized => Ok("sessionizer_payment_intents".to_string()),
            Self::ConnectorEvents => Ok("connector_events_audit".to_string()),
            Self::OutgoingWebhookEvent => Ok("outgoing_webhook_events_audit".to_string()),
            Self::Dispute => Ok("dispute".to_string()),
            Self::DisputeSessionized => Ok("sessionizer_dispute".to_string()),
            Self::ActivePaymentsAnalytics => Ok("active_payments".to_string()),
            Self::Authentications => Ok("authentications".to_string()),
            Self::RoutingEvents => Ok("routing_events_audit".to_string()),
        }
    }
}

impl<T> ToSql<ClickhouseClient> for Aggregate<T>
where
    T: ToSql<ClickhouseClient>,
{
    fn to_sql(&self, table_engine: &TableEngine) -> error_stack::Result<String, ParsingError> {
        Ok(match self {
            Self::Count { field: _, alias } => {
                let query = match table_engine {
                    TableEngine::CollapsingMergeTree { sign } => format!("sum({sign})"),
                    TableEngine::BasicTree => "count(*)".to_string(),
                };
                format!(
                    "{query}{}",
                    alias.map_or_else(|| "".to_owned(), |alias| format!(" as {alias}"))
                )
            }
            Self::Sum { field, alias } => {
                let query = match table_engine {
                    TableEngine::CollapsingMergeTree { sign } => format!(
                        "sum({sign} * {})",
                        field
                            .to_sql(table_engine)
                            .attach_printable("Failed to sum aggregate")?
                    ),
                    TableEngine::BasicTree => format!(
                        "sum({})",
                        field
                            .to_sql(table_engine)
                            .attach_printable("Failed to sum aggregate")?
                    ),
                };
                format!(
                    "{query}{}",
                    alias.map_or_else(|| "".to_owned(), |alias| format!(" as {alias}"))
                )
            }
            Self::Min { field, alias } => {
                format!(
                    "min({}){}",
                    field
                        .to_sql(table_engine)
                        .attach_printable("Failed to min aggregate")?,
                    alias.map_or_else(|| "".to_owned(), |alias| format!(" as {alias}"))
                )
            }
            Self::Max { field, alias } => {
                format!(
                    "max({}){}",
                    field
                        .to_sql(table_engine)
                        .attach_printable("Failed to max aggregate")?,
                    alias.map_or_else(|| "".to_owned(), |alias| format!(" as {alias}"))
                )
            }
            Self::Percentile {
                field,
                alias,
                percentile,
            } => {
                format!(
                    "quantilesExact(0.{})({})[1]{}",
                    percentile.map_or_else(|| "50".to_owned(), |percentile| percentile.to_string()),
                    field
                        .to_sql(table_engine)
                        .attach_printable("Failed to percentile aggregate")?,
                    alias.map_or_else(|| "".to_owned(), |alias| format!(" as {alias}"))
                )
            }
            Self::DistinctCount { field, alias } => {
                format!(
                    "count(distinct {}){}",
                    field
                        .to_sql(table_engine)
                        .attach_printable("Failed to percentile aggregate")?,
                    alias.map_or_else(|| "".to_owned(), |alias| format!(" as {alias}"))
                )
            }
        })
    }
}

impl<T> ToSql<ClickhouseClient> for Window<T>
where
    T: ToSql<ClickhouseClient>,
{
    fn to_sql(&self, table_engine: &TableEngine) -> error_stack::Result<String, ParsingError> {
        Ok(match self {
            Self::Sum {
                field,
                partition_by,
                order_by,
                alias,
            } => {
                format!(
                    "sum({}) over ({}{}){}",
                    field
                        .to_sql(table_engine)
                        .attach_printable("Failed to sum window")?,
                    partition_by.as_ref().map_or_else(
                        || "".to_owned(),
                        |partition_by| format!("partition by {}", partition_by.to_owned())
                    ),
                    order_by.as_ref().map_or_else(
                        || "".to_owned(),
                        |(order_column, order)| format!(
                            " order by {} {}",
                            order_column.to_owned(),
                            order
                        )
                    ),
                    alias.map_or_else(|| "".to_owned(), |alias| format!(" as {alias}"))
                )
            }
            Self::RowNumber {
                field: _,
                partition_by,
                order_by,
                alias,
            } => {
                format!(
                    "row_number() over ({}{}){}",
                    partition_by.as_ref().map_or_else(
                        || "".to_owned(),
                        |partition_by| format!("partition by {}", partition_by.to_owned())
                    ),
                    order_by.as_ref().map_or_else(
                        || "".to_owned(),
                        |(order_column, order)| format!(
                            " order by {} {}",
                            order_column.to_owned(),
                            order
                        )
                    ),
                    alias.map_or_else(|| "".to_owned(), |alias| format!(" as {alias}"))
                )
            }
        })
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ClickhouseError {
    #[error("Clickhouse connection error")]
    ConnectionError,
    #[error("Clickhouse NON-200 response content: '{0}'")]
    ResponseNotOK(String),
    #[error("Clickhouse response error")]
    ResponseError,
}
