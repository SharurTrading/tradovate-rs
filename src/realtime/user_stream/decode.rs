// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

//! Bounded typed decoding for user bootstrap and `props` payloads.

use serde::{Deserialize, de::DeserializeOwned};
use serde_json::value::RawValue;

use super::{PropertyEvent, PropertyEventType, UserBootstrap, UserEntityBatch, UserStreamEvent};
use crate::api::current::alerts::AdminAlertSignal;
use crate::api::current::users::{
    Account, AccountRiskStatus, AnnualReview, CashBalance, Command, CommandReport, Contract,
    ContractGroup, ContractMaturity, Currency, Exchange, ExecutionReport, Fill, FillFee, FillPair,
    MarginSnapshot, Order, OrderStrategy, OrderStrategyLink, OrderStrategyType, OrderVersion,
    Position, Product, Property, SpreadDefinition, User, UserAccountAutoLiq, UserPlugin,
    UserPromoCode, UserProperty, UserReadStatus,
};
use crate::realtime::bounded::{self, BoundedMap, BoundedVec, DecodeError};
use crate::realtime::{
    ProviderCode, RealtimeError, RealtimeEventKind, RealtimePayloadError, Response,
};

pub(super) const MAX_BOOTSTRAP_COLLECTIONS: usize = 128;
const MAX_ENTITIES_PER_BATCH: usize = 16_384;
const MAX_PROPERTY_EVENTS: usize = 4_096;

pub(crate) const BOOTSTRAP_COLLECTIONS: &[&str] = &[
    "users",
    "userProperties",
    "properties",
    "accounts",
    "accountRiskStatuses",
    "marginSnapshots",
    "userAccountAutoLiqs",
    "cashBalances",
    "currencies",
    "positions",
    "fillPairs",
    "orders",
    "contracts",
    "contractMaturities",
    "products",
    "exchanges",
    "spreadDefinitions",
    "commands",
    "commandReports",
    "executionReports",
    "orderVersions",
    "fills",
    "fillFees",
    "orderStrategies",
    "orderStrategyLinks",
    "userPlugins",
    "annualReviews",
    "userReadStatuses",
    "userPromoCodes",
    "contractGroups",
    "orderStrategyTypes",
];

pub(crate) fn bootstrap(response: &Response) -> Result<UserStreamEvent, RealtimeError> {
    let data = response.data().ok_or_else(|| {
        invalid(
            RealtimeEventKind::Bootstrap,
            RealtimePayloadError::MissingData,
        )
    })?;
    let collections = bounded::from_str::<
        BoundedMap<String, Box<RawValue>, MAX_BOOTSTRAP_COLLECTIONS>,
    >(data.get())
    .map_err(|error| decode_error(RealtimeEventKind::Bootstrap, error))?;
    if !collections.as_map().contains_key("users")
        || !collections.as_map().contains_key("contractGroups")
    {
        return Err(invalid(
            RealtimeEventKind::Bootstrap,
            RealtimePayloadError::MissingData,
        ));
    }
    let entities = collections
        .into_map()
        .into_iter()
        .map(|(name, raw)| decode_bootstrap_collection(&name, &raw))
        .collect::<Result<Vec<_>, _>>()?;
    if entities.is_empty() {
        return Err(invalid(
            RealtimeEventKind::Bootstrap,
            RealtimePayloadError::MissingData,
        ));
    }
    Ok(UserStreamEvent::Bootstrap(UserBootstrap {
        entities: entities.into_boxed_slice(),
    }))
}

pub(crate) fn properties(data: Option<&RawValue>) -> Result<UserStreamEvent, RealtimeError> {
    let data = data.ok_or_else(|| {
        invalid(
            RealtimeEventKind::Properties,
            RealtimePayloadError::MissingData,
        )
    })?;
    let events = bounded::one_or_many::<WireProperty, MAX_PROPERTY_EVENTS>(data.get())
        .map_err(|error| decode_error(RealtimeEventKind::Properties, error))?;
    if events.is_empty() {
        return Err(invalid(
            RealtimeEventKind::Properties,
            RealtimePayloadError::MissingData,
        ));
    }
    let events = events
        .into_iter()
        .map(property)
        .collect::<Result<Vec<_>, _>>()?;
    Ok(UserStreamEvent::Properties(events.into_boxed_slice()))
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct WireProperty {
    entity_type: String,
    event_type: String,
    entity: Box<RawValue>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct OtherEnvironmentAdminAlertSignal {
    admin_alert_signal: AdminAlertSignal,
}

fn property(wire: WireProperty) -> Result<PropertyEvent, RealtimeError> {
    let event_type = match wire.event_type.as_str() {
        "Created" => PropertyEventType::Created,
        "Updated" => PropertyEventType::Updated,
        "Deleted" => PropertyEventType::Deleted,
        _ => PropertyEventType::Unknown(provider_code(wire.event_type)?),
    };
    let entities = decode_entity(&wire.entity_type, &wire.entity)?;
    Ok(PropertyEvent {
        event_type,
        entities,
    })
}

fn decode_bootstrap_collection(
    name: &str,
    raw: &RawValue,
) -> Result<UserEntityBatch, RealtimeError> {
    match name {
        "users" => decode_vec(raw).map(UserEntityBatch::Users),
        "userProperties" => decode_vec(raw).map(UserEntityBatch::UserProperties),
        "properties" => decode_vec(raw).map(UserEntityBatch::Properties),
        "accounts" => decode_vec(raw).map(UserEntityBatch::Accounts),
        "accountRiskStatuses" => decode_vec(raw).map(UserEntityBatch::AccountRiskStatuses),
        "marginSnapshots" => decode_vec(raw).map(UserEntityBatch::MarginSnapshots),
        "userAccountAutoLiqs" => decode_vec(raw).map(UserEntityBatch::UserAccountAutoLiqs),
        "cashBalances" => decode_vec(raw).map(UserEntityBatch::CashBalances),
        "currencies" => decode_vec(raw).map(UserEntityBatch::Currencies),
        "positions" => decode_vec(raw).map(UserEntityBatch::Positions),
        "fillPairs" => decode_vec(raw).map(UserEntityBatch::FillPairs),
        "orders" => decode_vec(raw).map(UserEntityBatch::Orders),
        "contracts" => decode_vec(raw).map(UserEntityBatch::Contracts),
        "contractMaturities" => decode_vec(raw).map(UserEntityBatch::ContractMaturities),
        "products" => decode_vec(raw).map(UserEntityBatch::Products),
        "exchanges" => decode_vec(raw).map(UserEntityBatch::Exchanges),
        "spreadDefinitions" => decode_vec(raw).map(UserEntityBatch::SpreadDefinitions),
        "commands" => decode_vec(raw).map(UserEntityBatch::Commands),
        "commandReports" => decode_vec(raw).map(UserEntityBatch::CommandReports),
        "executionReports" => decode_vec(raw).map(UserEntityBatch::ExecutionReports),
        "orderVersions" => decode_vec(raw).map(UserEntityBatch::OrderVersions),
        "fills" => decode_vec(raw).map(UserEntityBatch::Fills),
        "fillFees" => decode_vec(raw).map(UserEntityBatch::FillFees),
        "orderStrategies" => decode_vec(raw).map(UserEntityBatch::OrderStrategies),
        "orderStrategyLinks" => decode_vec(raw).map(UserEntityBatch::OrderStrategyLinks),
        "userPlugins" => decode_vec(raw).map(UserEntityBatch::UserPlugins),
        "annualReviews" => decode_vec(raw).map(UserEntityBatch::AnnualReviews),
        "userReadStatuses" => decode_vec(raw).map(UserEntityBatch::UserReadStatuses),
        "userPromoCodes" => decode_vec(raw).map(UserEntityBatch::UserPromoCodes),
        "contractGroups" => decode_vec(raw).map(UserEntityBatch::ContractGroups),
        "orderStrategyTypes" => decode_vec(raw).map(UserEntityBatch::OrderStrategyTypes),
        _ => unsupported(name.to_owned(), raw, true),
    }
}

fn decode_entity(name: &str, raw: &RawValue) -> Result<UserEntityBatch, RealtimeError> {
    match name {
        "user" => decode_one_or_many::<User>(raw).map(UserEntityBatch::Users),
        "userProperty" => {
            decode_one_or_many::<UserProperty>(raw).map(UserEntityBatch::UserProperties)
        }
        "property" => decode_one_or_many::<Property>(raw).map(UserEntityBatch::Properties),
        "account" => decode_one_or_many::<Account>(raw).map(UserEntityBatch::Accounts),
        "accountRiskStatus" => {
            decode_one_or_many::<AccountRiskStatus>(raw).map(UserEntityBatch::AccountRiskStatuses)
        }
        "marginSnapshot" => {
            decode_one_or_many::<MarginSnapshot>(raw).map(UserEntityBatch::MarginSnapshots)
        }
        "userAccountAutoLiq" => {
            decode_one_or_many::<UserAccountAutoLiq>(raw).map(UserEntityBatch::UserAccountAutoLiqs)
        }
        "cashBalance" => decode_one_or_many::<CashBalance>(raw).map(UserEntityBatch::CashBalances),
        "currency" => decode_one_or_many::<Currency>(raw).map(UserEntityBatch::Currencies),
        "position" => decode_one_or_many::<Position>(raw).map(UserEntityBatch::Positions),
        "fillPair" => decode_one_or_many::<FillPair>(raw).map(UserEntityBatch::FillPairs),
        "order" => decode_one_or_many::<Order>(raw).map(UserEntityBatch::Orders),
        "contract" => decode_one_or_many::<Contract>(raw).map(UserEntityBatch::Contracts),
        "contractMaturity" => {
            decode_one_or_many::<ContractMaturity>(raw).map(UserEntityBatch::ContractMaturities)
        }
        "product" => decode_one_or_many::<Product>(raw).map(UserEntityBatch::Products),
        "exchange" => decode_one_or_many::<Exchange>(raw).map(UserEntityBatch::Exchanges),
        "spreadDefinition" => {
            decode_one_or_many::<SpreadDefinition>(raw).map(UserEntityBatch::SpreadDefinitions)
        }
        "command" => decode_one_or_many::<Command>(raw).map(UserEntityBatch::Commands),
        "commandReport" => {
            decode_one_or_many::<CommandReport>(raw).map(UserEntityBatch::CommandReports)
        }
        "executionReport" => {
            decode_one_or_many::<ExecutionReport>(raw).map(UserEntityBatch::ExecutionReports)
        }
        "orderVersion" => {
            decode_one_or_many::<OrderVersion>(raw).map(UserEntityBatch::OrderVersions)
        }
        "fill" => decode_one_or_many::<Fill>(raw).map(UserEntityBatch::Fills),
        "fillFee" => decode_one_or_many::<FillFee>(raw).map(UserEntityBatch::FillFees),
        "orderStrategy" => {
            decode_one_or_many::<OrderStrategy>(raw).map(UserEntityBatch::OrderStrategies)
        }
        "orderStrategyLink" => {
            decode_one_or_many::<OrderStrategyLink>(raw).map(UserEntityBatch::OrderStrategyLinks)
        }
        "userPlugin" => decode_one_or_many::<UserPlugin>(raw).map(UserEntityBatch::UserPlugins),
        "annualReview" => {
            decode_one_or_many::<AnnualReview>(raw).map(UserEntityBatch::AnnualReviews)
        }
        "userReadStatus" => {
            decode_one_or_many::<UserReadStatus>(raw).map(UserEntityBatch::UserReadStatuses)
        }
        "userPromoCode" => {
            decode_one_or_many::<UserPromoCode>(raw).map(UserEntityBatch::UserPromoCodes)
        }
        "contractGroup" => {
            decode_one_or_many::<ContractGroup>(raw).map(UserEntityBatch::ContractGroups)
        }
        "orderStrategyType" => {
            decode_one_or_many::<OrderStrategyType>(raw).map(UserEntityBatch::OrderStrategyTypes)
        }
        "OtherEnvAdminAlertSignal" => decode_one_or_many::<OtherEnvironmentAdminAlertSignal>(raw)
            .map(|values| {
                values
                    .into_vec()
                    .into_iter()
                    .map(|value| value.admin_alert_signal)
                    .collect::<Vec<_>>()
                    .into_boxed_slice()
            })
            .map(UserEntityBatch::OtherEnvironmentAdminAlertSignals),
        _ => unsupported(name.to_owned(), raw, false),
    }
}

fn decode_vec<T>(raw: &RawValue) -> Result<Box<[T]>, RealtimeError>
where
    T: DeserializeOwned,
{
    bounded::from_str::<BoundedVec<T, MAX_ENTITIES_PER_BATCH>>(raw.get())
        .map(BoundedVec::into_vec)
        .map(Vec::into_boxed_slice)
        .map_err(|error| decode_error(RealtimeEventKind::Bootstrap, error))
}

fn decode_one_or_many<T>(raw: &RawValue) -> Result<Box<[T]>, RealtimeError>
where
    T: DeserializeOwned,
{
    bounded::one_or_many::<T, MAX_ENTITIES_PER_BATCH>(raw.get())
        .map(Vec::into_boxed_slice)
        .map_err(|error| decode_error(RealtimeEventKind::Properties, error))
}

fn unsupported(
    name: String,
    raw: &RawValue,
    bootstrap: bool,
) -> Result<UserEntityBatch, RealtimeError> {
    let kind = if bootstrap {
        RealtimeEventKind::Bootstrap
    } else {
        RealtimeEventKind::Properties
    };
    let count = bounded::count_one_or_many::<MAX_ENTITIES_PER_BATCH>(raw.get())
        .map_err(|error| decode_error(kind, error))?;
    Ok(UserEntityBatch::Unsupported {
        entity_type: provider_code(name)?,
        item_count: count,
    })
}

fn provider_code(value: String) -> Result<ProviderCode, RealtimeError> {
    ProviderCode::from_wire(value).ok_or_else(|| {
        invalid(
            RealtimeEventKind::Properties,
            RealtimePayloadError::InvalidValue,
        )
    })
}

fn decode_error(kind: RealtimeEventKind, error: DecodeError) -> RealtimeError {
    let reason = match error {
        DecodeError::LimitExceeded => RealtimePayloadError::LimitExceeded,
        DecodeError::Malformed => RealtimePayloadError::Malformed,
    };
    invalid(kind, reason)
}

fn invalid(kind: RealtimeEventKind, reason: RealtimePayloadError) -> RealtimeError {
    RealtimeError::InvalidEvent { kind, reason }
}
