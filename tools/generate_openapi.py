#!/usr/bin/env python3
# SPDX-FileCopyrightText: 2026 Kevin Monaghan
# SPDX-License-Identifier: MIT-0

"""Generate the current, typed Tradovate REST surface from the pinned schema."""

from __future__ import annotations

import argparse
import hashlib
import json
import re
import shutil
import subprocess
import sys
import tempfile
from collections import defaultdict
from pathlib import Path
from typing import Any


ROOT = Path(__file__).resolve().parents[1]
SPEC = ROOT / "spec" / "official" / "openapi-2026-08-21.json"
OUTPUT = ROOT / "src" / "api" / "current" / "generated"
COVERAGE_OUTPUT = ROOT / "docs" / "api-coverage-rest.md"
SOURCE_URL = "https://partner.tradovate.com/openapi.json"
EXPECTED_SHA256 = "37caeccf4b0913460a788fcaf4c902497059b8ffe6f6355512e6c08eaacde769"

TAG_MODULE = {
    "Authentication": "authentication",
    "Accounting": "accounting",
    "contractLibrary": "contracts",
    "Orders": "orders",
    "Positions": "positions",
    "Risks": "risks",
    "Users": "users",
    "fees": "fees",
    "funds": "funds",
    "Configuration": "configuration",
    "Alerts": "alerts",
}

MODULE_DOC = {
    "authentication": "Current authentication operations and wire models.",
    "accounting": "Current account, balance, margin, and permission operations.",
    "contracts": "Current contract-library operations and wire models.",
    "orders": "Current order, command, execution, and fill operations.",
    "positions": "Current position and fill-pair operations.",
    "risks": "Current risk-control operations and wire models.",
    "users": "Current user, subscription, and contact operations.",
    "fees": "Current fee and subscription-plan operations.",
    "funds": "Current fund-adjustment operations.",
    "configuration": "Current configuration and entitlement operations.",
    "alerts": "Current alert operations and wire models.",
    "shared": "Wire models shared by current Tradovate capabilities.",
}

SAFE_POST_QUERIES = {
    "/cashBalance/getcashbalancesnapshot",
    "/contract/getproductfeeparams",
    "/contract/rollcontract",
    "/contract/rollcontracts",
    "/order/dryrun",
    "/customerApplication/checkduplicate",
    "/customerApplication/getpartnersubaccountrequeststatus",
    "/user/getaccounttradingpermissions",
    "/user/getsecondmarketdatasubscriptioncost",
    "/user/syncrequest",
}

LIFECYCLE_ENDPOINTS = {
    "/auth/accesstokenrequest",
    "/auth/oauthtoken",
    "/auth/renewaccesstoken",
    "/user/modifycredentials",
    "/user/modifypassword",
    "/user/syncrequest",
}

SPECIAL_ENDPOINTS = {
    "/account/resetdemoaccountstate",
    "/accountRiskStatus/resetautoliqstatus",
    "/accountRiskStatus/setadminautoliqaction",
    "/adminAlertSignal/completealertsignal",
    "/adminAlertSignal/takealertsignalownership",
    "/alert/deletealert",
    "/alert/dismissalert",
    "/alert/createalert",
    "/alert/markreadalertsignal",
    "/alert/modifyalert",
    "/alert/resetalert",
    "/auth/accesstokenrequest",
    "/auth/renewaccesstoken",
    "/auth/oauthtoken",
    "/cashBalance/changedemobalance",
    "/contactInfo/updatecontactinfo",
    "/contactInfo/updatecontactcountry",
    "/contactInfo/updatecontactinfoname",
    "/accountRiskStatus/setaccountnotes",
    "/accountRiskStatus/switchriskcategory",
    "/accountRiskStatus/updatemaxnetliq",
    "/order/cancelorder",
    "/order/dryrun",
    "/order/liquidateposition",
    "/order/liquidatepositions",
    "/order/modifyorder",
    "/order/placeoco",
    "/order/placeorder",
    "/order/placeoso",
    "/orderStrategy/interruptorderstrategy",
    "/orderStrategy/modifyorderstrategy",
    "/orderStrategy/startorderstrategy",
    "/contract/rollcontracts",
    "/customerApplication/createpartnersubaccountrequest",
    "/customerApplication/submitcustomerapplicationdocument",
    "/customerApplication/submitpartnersubaccountdocument",
    "/executionReport/find",
    "/fundTransaction/adjustcash",
    "/marketDataSubscription/list",
    "/marketDataSubscription/create",
    "/marketDataSubscription/update",
    "/pOAContact/create",
    "/pOAContact/update",
    "/tradovateSubscription/create",
    "/tradovateSubscription/list",
    "/user/activatesecondmarketdatasubscriptionrenewal",
    "/user/accepttradingpermission",
    "/user/addmarketdatasubscription",
    "/user/addtradovatesubscription",
    "/user/canceleverything",
    "/user/canceltradovatesubscription",
    "/user/createevaluationaccounts",
    "/user/createevaluationusers",
    "/user/createtradingpermission",
    "/user/expireuserlockout",
    "/user/requesttradingpermission",
    "/user/revoketradingpermission",
    "/user/revoketradingpermissions",
    "/userAccountAutoLiq/create",
    "/userAccountAutoLiq/update",
    "/userAccountAutoLiq/updateuserautoliq",
    "/userAccountAutoLiq/updateuserautoliqs",
    "/userAccountPositionLimit/create",
    "/userAccountPositionLimit/deleteuseraccountpositionlimit",
    "/userAccountPositionLimit/deleteuseraccountriskparameter",
    "/userAccountPositionLimit/update",
    "/userAccountRiskParameter/create",
    "/userAccountRiskParameter/update",
    "/user/addsecondmarketdatasubscription",
    "/user/cancelsecondmarketdatasubscription",
    "/user/cancelsecondmarketdatasubscriptionrenewal",
    "/user/expiremarketdatasubscription",
    "/user/getsecondmarketdatasubscriptioncost",
    "/user/modifycredentials",
    "/user/modifyemailaddress",
    "/user/modifypassword",
    "/user/opendemoaccount",
    "/user/signuporganizationmember",
    "/userPlugin/addentitlementsubscription",
    "/userPlugin/create",
    "/userPlugin/update",
    "/userPlugin/list",
    "/userPlugin/changepluginpermission",
    "/userSessionStats/list",
    "/user/syncrequest",
    "/workspaceTemplate/create",
    "/workspaceTemplate/update",
}

DOCUMENTATION_BLOCKED_REASONS = {
    "/accountRiskStatus/setaccountnotes": (
        "the pinned request schema contains no notes value to set"
    ),
    "/contract/rollcontracts": (
        "the response contracts map has no documented key or value schema"
    ),
    "/executionReport/find": "the pinned operation publishes no parameters or response schema",
    "/marketDataSubscription/list": "the pinned operation publishes no response schema",
    "/orderStrategy/modifyorderstrategy": (
        "the request exposes a raw command string without a documented command grammar"
    ),
    "/tradovateSubscription/list": "the pinned operation publishes no response schema",
    "/user/activatesecondmarketdatasubscriptionrenewal": (
        "the pinned operation publishes neither request nor response content"
    ),
    "/userAccountAutoLiq/create": (
        "the component omits the account or target identity required for a safe create"
    ),
    "/userAccountAutoLiq/update": (
        "the component omits target identity and does not require an update ID"
    ),
    "/user/addsecondmarketdatasubscription": (
        "the pinned operation publishes neither request nor response content"
    ),
    "/user/cancelsecondmarketdatasubscription": (
        "the pinned operation publishes neither request nor response content"
    ),
    "/user/cancelsecondmarketdatasubscriptionrenewal": (
        "the pinned operation publishes neither request nor response content"
    ),
    "/user/expiremarketdatasubscription": (
        "the pinned operation publishes neither request nor response content"
    ),
    "/user/getsecondmarketdatasubscriptioncost": (
        "the pinned operation publishes neither request nor response content or a cost unit"
    ),
    "/userPlugin/list": "the pinned operation publishes no response schema",
    "/userSessionStats/list": "the pinned operation publishes no response schema",
}

DOCUMENTATION_BLOCKED_ENDPOINTS = set(DOCUMENTATION_BLOCKED_REASONS)

# Mutation completion is never inferred from arbitrary field presence. Every
# response schema that can resolve a mutation is reviewed here. A required
# direct entity is accepted after decoding; optional wrappers must expose the
# named endpoint-specific evidence. Schemas with only an `ok`, `success`, or
# `*Reason::Success` indicator need no entry.
MUTATION_ALWAYS_EVIDENCE = {
    "CancelEverythingResponse",
}

MUTATION_ANY_EVIDENCE = {
    "CashBalanceSnapshot": (
        "totalCashValue",
        "totalPnL",
        "initialMargin",
        "maintenanceMargin",
        "netLiq",
        "openPnL",
        "realizedPnL",
        "weekRealizedPnL",
        "currencyCashAvailWithdrawalUSD",
        "netLiqSOD",
        "totalCashValueSOD",
        "cashUSD",
        "cashSODUSD",
        "fullInitialMargin",
        "fullInitialMarginSOD",
        "autoLiqLevel",
    ),
    "CreatePartnerSubAccountRequestResponse": ("requestId",),
    "MarketDataSubscription": ("id",),
    "OpenDemoAccountResponse": ("accountId",),
    "POAContact": ("id",),
    "SubmitPartnerSubAccountDocumentResponse": ("documentId",),
    "TradovateSubscription": ("id",),
    "UserAccountAutoLiq": ("id",),
    "UserAccountPositionLimit": ("id",),
    "UserAccountRiskParameter": ("id",),
    "UserPlugin": ("id",),
    "UserStatusMessage": ("status",),
    "WorkspaceTemplate": ("id",),
}

MUTATION_CUSTOM_EVIDENCE = {
    "AccountRiskStatusResponse": "self.account_risk_status.as_ref().is_some_and(|value| value.id().is_some())",
    "AdminAlertSignalResponse": "self.admin_alert_signal.as_ref().is_some_and(|value| value.id().is_some())",
    "AlertResponse": "self.alert.as_ref().is_some_and(|value| value.id().is_some())",
    "CreateEvaluationAccountsResponse": "!self.results.is_empty() && self.results.iter().all(|value| value.error_text().is_none_or(str::is_empty) && value.account_id().is_some() && value.trading_permission_id().is_some())",
    "CreateEvaluationUsersResponse": "!self.results.is_empty() && self.results.iter().all(|value| value.error_text().is_none_or(str::is_empty) && value.user_id().is_some())",
    "EntitlementSubscriptionResponse": "self.entitlement_subscription.as_ref().is_some_and(|value| value.id().is_some())",
    "MarketDataSubscriptionResponse": "self.market_data_subscription.as_ref().is_some_and(|value| value.id().is_some())",
    "TradingPermissionResponse": "self.trading_permission.as_ref().is_some_and(|value| value.id().is_some())",
    "TradovateSubscriptionResponse": "self.tradovate_subscription.as_ref().is_some_and(|value| value.id().is_some())",
    "UpdateContactInfoResponse": "self.contact_info.as_ref().is_some_and(|value| value.id().is_some())",
    "UpdateUserAutoLiqResponse": "self.user_account_auto_liq.as_ref().is_some_and(|value| value.id().is_some()) || self.permissioned_account_auto_liq.as_ref().is_some_and(|value| value.id().is_some())",
    "UpdateUserAutoLiqsResponse": "!self.user_auto_liqs.is_empty() && self.user_auto_liqs.iter().all(|value| value.user_account_auto_liq().is_some_and(|item| item.id().is_some()) || value.permissioned_account_auto_liq().is_some_and(|item| item.id().is_some()))",
}

MUTATION_ALL_EVIDENCE: dict[str, tuple[str, ...]] = {}

UNSPECIFIED_RESPONSES = {
    "/executionReport/find",
    "/marketDataSubscription/list",
    "/tradovateSubscription/list",
    "/user/activatesecondmarketdatasubscriptionrenewal",
    "/user/addsecondmarketdatasubscription",
    "/user/cancelsecondmarketdatasubscription",
    "/user/cancelsecondmarketdatasubscriptionrenewal",
    "/user/expiremarketdatasubscription",
    "/user/getsecondmarketdatasubscriptioncost",
    "/userPlugin/list",
    "/userSessionStats/list",
}

INCOMPLETE_RESPONSE_ENDPOINTS = {"/contract/rollcontracts"}

# These pinned components declare an object but publish no member/value
# grammar. They remain named so the coverage manifest can identify the gap,
# but decoding must reject any non-empty provider object rather than silently
# discard unknown financial/risk data.
INCOMPLETE_COMPONENT_SCHEMAS = {
    "ExtraPreTradeRiskContracts",
    "ExtraPreTradeRiskProducts",
    "RollContractsResponseContracts",
}

SECRET_NAME_SCHEMAS = {
    "AccessTokenRequest",
    "AccessTokenResponse",
    "EvaluationUser",
    "ModifyCredentials",
    "SignUpOrganizationMember",
}

ROOT_IDS = {
    "AccountId": "crate::AccountId",
    "CommandId": "crate::CommandId",
    "ContractId": "crate::ContractId",
    "ContractMaturityId": "crate::ContractMaturityId",
    "OrderId": "crate::OrderId",
    "PositionId": "crate::PositionId",
    "UserId": "crate::UserId",
}

# Provider field prefixes describe a role, not a distinct identity domain.
# Canonicalizing these suffixes lets an ID decoded from one operation flow
# directly into another operation for the same entity.
CANONICAL_ID_SUFFIXES = (
    ("contractmaturityid", "ContractMaturityId"),
    ("accountid", "AccountId"),
    ("userid", "UserId"),
    ("contractid", "ContractId"),
    ("commandid", "CommandId"),
    ("positionid", "PositionId"),
    ("orderid", "OrderId"),
    ("fillid", "FillId"),
    ("currencyid", "CurrencyId"),
    ("documentid", "DocumentId"),
    ("docid", "DocumentId"),
    ("creditcardid", "CreditCardId"),
    ("contractgroupid", "ContractGroupId"),
)

CANONICAL_ID_FIELDS = {
    ("CreatePartnerSubAccountRequestResponse", "requestId"): "SubAccountRequestId",
    ("PartnerSubAccountRequestStatusResponse", "requestId"): "SubAccountRequestId",
    ("ContractMaturity", "underlyingId"): "ContractMaturityId",
    ("MarketDataSubscriptionExchangeScope", "id"): "ExchangeScopeId",
    ("Order", "linkedId"): "OrderId",
    ("Order", "parentId"): "OrderId",
}

SECRET_FIELDS = {
    "accessToken",
    "access_token",
    "assertion",
    "base64data",
    "client_secret",
    "code",
    "code_verifier",
    "currentPassword",
    "httpAuth",
    "id_token",
    "mdAccessToken",
    "nationalId",
    "password",
    "refresh_token",
    "sec",
    "secret",
    "taxIdentifier",
}

SECRET_RESPONSE_SCHEMAS = {"AccessTokenResponse", "OAuthTokenResponse"}

SECRET_ACCESSOR_FIELDS = {
    ("AccessTokenResponse", "accessToken"),
    ("CreatePartnerSubAccountRequest", "nationalId"),
    ("CreatePartnerSubAccountRequest", "taxIdentifier"),
    ("EvaluationUser", "name"),
    ("OAuthTokenResponse", "access_token"),
    ("POAContact", "nationalId"),
    ("POAContact", "taxIdentifier"),
    ("SignUpOrganizationMember", "name"),
    ("SubmitCustomerApplicationDocument", "base64data"),
    ("SubmitPartnerSubAccountDocument", "base64data"),
}

STRING_NEWTYPE_FIELDS = {
    "accountSpec": "crate::AccountSpec",
    "clOrdId": "crate::ClientOrderId",
    "deviceId": "crate::DeviceId",
    "symbol": "crate::Symbol",
}

RUST_KEYWORDS = {
    "as", "async", "await", "break", "const", "continue", "crate", "dyn",
    "else", "enum", "extern", "false", "fn", "for", "if", "impl", "in",
    "let", "loop", "match", "mod", "move", "mut", "pub", "ref", "return",
    "self", "Self", "static", "struct", "super", "trait", "true", "type",
    "union", "unsafe", "use", "where", "while", "yield", "try", "macro",
}


def header() -> str:
    return (
        "// SPDX-FileCopyrightText: 2026 Kevin Monaghan\n"
        "// SPDX-License-Identifier: MIT-0\n"
        "// @generated\n"
        "// Generator: tools/generate_openapi.py\n"
        f"// Source: {SOURCE_URL} (snapshot 2026-08-21, sha256 {EXPECTED_SHA256})\n\n"
    )


def markdown_header() -> str:
    return (
        "<!--\n"
        "SPDX-FileCopyrightText: 2026 Kevin Monaghan\n"
        "SPDX-License-Identifier: MIT-0\n\n"
        "@generated by tools/generate_openapi.py from the pinned current Partner OpenAPI.\n"
        "Do not edit this file by hand.\n"
        "-->\n\n"
    )


def snake(name: str) -> str:
    value = re.sub(r"[^A-Za-z0-9]+", "_", name)
    value = re.sub(r"(.)([A-Z][a-z]+)", r"\1_\2", value)
    value = re.sub(r"([a-z0-9])([A-Z])", r"\1_\2", value).lower().strip("_")
    if not value:
        value = "value"
    if value[0].isdigit():
        value = f"value_{value}"
    if value in RUST_KEYWORDS:
        value += "_"
    return value


def pascal(name: str) -> str:
    parts = [part for part in re.split(r"[^A-Za-z0-9]+", name) if part]
    if len(parts) == 1:
        parts = [part for part in snake(parts[0]).split("_") if part]
    value = "".join(part[:1].upper() + part[1:] for part in parts)
    if not value:
        value = "Value"
    if value[0].isdigit():
        value = f"Value{value}"
    if value in RUST_KEYWORDS:
        value += "Value"
    return value


def ref_name(schema: dict[str, Any]) -> str | None:
    ref = schema.get("$ref")
    return ref.rsplit("/", 1)[-1] if isinstance(ref, str) else None


def walk_refs(schema: Any) -> set[str]:
    found: set[str] = set()
    if isinstance(schema, dict):
        ref = ref_name(schema)
        if ref is not None:
            found.add(ref)
        for value in schema.values():
            found.update(walk_refs(value))
    elif isinstance(schema, list):
        for value in schema:
            found.update(walk_refs(value))
    return found


class Generator:
    def __init__(self, spec: dict[str, Any]) -> None:
        self.spec = spec
        self.schemas: dict[str, dict[str, Any]] = spec["components"]["schemas"]
        self.owners: dict[str, str] = {}
        self.operations: list[dict[str, Any]] = []
        self.ids: set[str] = {"ProviderEntityId"}
        self._inventory_operations()
        self.request_schemas = {
            name
            for item in self.operations
            if (name := ref_name(item["operation"].get("requestBody", {}).get("content", {}).get("application/json", {}).get("schema", {})))
            is not None
        }
        self._assign_schema_owners()
        self._collect_ids()

    def _inventory_operations(self) -> None:
        for path, path_item in self.spec["paths"].items():
            for method in ("get", "post"):
                operation = path_item.get(method)
                if operation is None:
                    continue
                tag = operation["tags"][0]
                module = TAG_MODULE[tag]
                classification = (
                    "query"
                    if method == "get" or path in SAFE_POST_QUERIES
                    else "mutation"
                )
                if path == "/auth/renewaccesstoken":
                    classification = "mutation"
                if path in LIFECYCLE_ENDPOINTS:
                    classification = "lifecycle"
                self.operations.append(
                    {
                        "path": path,
                        "method": method,
                        "operation": operation,
                        "module": module,
                        "class": classification,
                    }
                )
        if len(self.operations) != 350:
            raise ValueError(f"expected 350 operations, found {len(self.operations)}")

    def _assign_schema_owners(self) -> None:
        def assign(name: str, module: str) -> None:
            if name in self.owners or name not in self.schemas:
                return
            self.owners[name] = module
            for child in sorted(walk_refs(self.schemas[name])):
                assign(child, module)

        for item in self.operations:
            operation = item["operation"]
            for name in sorted(walk_refs(operation.get("requestBody", {}))):
                assign(name, item["module"])
            for name in sorted(walk_refs(operation.get("responses", {}))):
                assign(name, item["module"])
        for name in self.schemas:
            assign(name, "shared")

    def _collect_ids(self) -> None:
        for schema_name, schema in self.schemas.items():
            for field_name, field_schema in schema.get("properties", {}).items():
                self._record_id(schema_name, field_name, field_schema)
        for item in self.operations:
            resource = item["path"].strip("/").split("/", 1)[0]
            for parameter in item["operation"].get("parameters", []):
                if parameter.get("in") != "query":
                    continue
                schema = parameter["schema"]
                if schema.get("type") == "array":
                    schema = schema["items"]
                self._record_id(pascal(resource), parameter["name"], schema)

    def _record_id(self, context: str, field: str, schema: dict[str, Any]) -> None:
        if schema.get("type") != "integer":
            return
        identity = self.id_name(context, field)
        if identity is not None and identity not in ROOT_IDS:
            self.ids.add(identity)

    @staticmethod
    def id_name(context: str, field: str) -> str | None:
        override = CANONICAL_ID_FIELDS.get((context, field))
        if override is not None:
            return override
        lower = field.lower()
        if lower in {"masterid", "masterids"}:
            return "ProviderEntityId"
        if lower in {"id", "ids"}:
            return f"{pascal(context)}Id"
        singular = field[:-1] if field.endswith("s") else field
        singular_lower = singular.lower()
        for suffix, identity in CANONICAL_ID_SUFFIXES:
            if singular_lower.endswith(suffix):
                return identity
        if singular_lower.endswith("id"):
            return f"{pascal(singular[:-2])}Id"
        return None

    def schema_type(
        self,
        schema: dict[str, Any],
        owner: str,
        context: str,
        field: str,
    ) -> tuple[str, str]:
        reference = ref_name(schema)
        if reference is not None:
            target_owner = self.owners[reference]
            rust_type = reference if target_owner == owner else f"super::{target_owner}::{reference}"
            return rust_type, "plain"
        kind = schema.get("type")
        if kind == "array":
            item_type, _ = self.schema_type(schema["items"], owner, context, field)
            return f"Vec<{item_type}>", "array"
        if kind == "integer":
            identity = self.id_name(context, field)
            if identity is not None:
                return ROOT_IDS.get(identity, f"super::ids::{identity}"), "plain"
            return "i64", "copy"
        if kind == "number":
            return "crate::Decimal", "decimal"
        if kind == "boolean":
            return "bool", "copy"
        if kind == "string" and schema.get("format") == "date-time":
            return "jiff::Timestamp", "plain"
        if kind == "string" and field in SECRET_FIELDS:
            return "crate::api::current::SecretValue", "secret"
        if kind == "string" and field in STRING_NEWTYPE_FIELDS:
            return STRING_NEWTYPE_FIELDS[field], "plain"
        if kind == "string":
            return "String", "string"
        raise ValueError(f"unsupported schema for {context}.{field}: {schema}")

    def render(self, destination: Path) -> None:
        destination.mkdir(parents=True, exist_ok=True)
        self._render_ids(destination / "ids.rs")
        self._render_manifest(destination / "manifest.rs")
        by_owner: dict[str, list[str]] = defaultdict(list)
        for name, owner in self.owners.items():
            by_owner[owner].append(name)
        operations_by_module: dict[str, list[dict[str, Any]]] = defaultdict(list)
        for item in self.operations:
            operations_by_module[item["module"]].append(item)
        modules = sorted(set(by_owner) | set(operations_by_module))
        for module in modules:
            chunks = [
                header(),
                "// Provider wire fields remain schema-auditable even when they repeat\n",
                "// their type name; wide schema-faithful builders remain one generated\n",
                "// unit so regeneration and source review cannot drift field subsets.\n",
                "#![allow(clippy::struct_field_names, clippy::too_many_lines)]\n\n",
                f"//! {MODULE_DOC[module]}\n\n",
            ]
            for name in sorted(by_owner[module]):
                schema = self.schemas[name]
                if schema.get("type") == "string":
                    chunks.append(self._render_enum(name, schema))
                else:
                    chunks.append(self._render_struct(name, schema, module))
                    if name in self.request_schemas:
                        chunks.append(self._render_request_validation(name, schema))
            for item in operations_by_module[module]:
                if (
                    item["path"] not in SPECIAL_ENDPOINTS
                    and item["class"] != "mutation"
                ):
                    chunks.append(self._render_endpoint(item))
            (destination / f"{module}.rs").write_text("".join(chunks), encoding="utf-8")
        self._render_mod(destination / "mod.rs", modules)
        subprocess.run(
            [
                "rustfmt",
                "--edition",
                "2024",
                "--config-path",
                str(ROOT / "rustfmt.toml"),
                *[str(path) for path in sorted(destination.glob("*.rs"))],
            ],
            check=True,
        )

    def _render_ids(self, path: Path) -> None:
        chunks = [
            header(),
            "//! Validated identities used only by the current generated contract.\n\n",
            "use std::fmt;\n\n",
            "use serde::{Deserialize, Deserializer, Serialize, Serializer, de};\n",
            "use thiserror::Error;\n\n",
            "/// A non-positive provider identity was decoded or constructed.\n",
            "#[derive(Clone, Copy, Debug, Eq, Error, PartialEq)]\n",
            "#[error(\"provider identity must be positive\")]\n",
            "pub struct CurrentIdError;\n\n",
            "macro_rules! current_id {\n",
            "    ($name:ident, $label:literal) => {\n",
            "        #[doc = concat!(\"A validated Tradovate \", $label, \" identifier.\")]\n",
            "        #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]\n",
            "        pub struct $name(i64);\n\n",
            "        impl $name {\n",
            "            /// Creates an identity from a positive provider integer.\n",
            "            ///\n",
            "            /// # Errors\n",
            "            ///\n",
            "            /// Returns [`CurrentIdError`] when `value` is not positive.\n",
            "            pub const fn new(value: i64) -> Result<Self, CurrentIdError> {\n",
            "                if value > 0 { Ok(Self(value)) } else { Err(CurrentIdError) }\n",
            "            }\n\n",
            "            /// Returns the provider integer.\n",
            "            #[must_use]\n",
            "            pub const fn get(self) -> i64 { self.0 }\n",
            "        }\n\n",
            "        impl TryFrom<i64> for $name {\n",
            "            type Error = CurrentIdError;\n",
            "            fn try_from(value: i64) -> Result<Self, Self::Error> { Self::new(value) }\n",
            "        }\n\n",
            "        impl fmt::Display for $name {\n",
            "            fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result { self.0.fmt(formatter) }\n",
            "        }\n\n",
            "        impl Serialize for $name {\n",
            "            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>\n",
            "            where S: Serializer { serializer.serialize_i64(self.0) }\n",
            "        }\n\n",
            "        impl<'de> Deserialize<'de> for $name {\n",
            "            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>\n",
            "            where D: Deserializer<'de> {\n",
            "                let value = i64::deserialize(deserializer)?;\n",
            "                Self::new(value).map_err(de::Error::custom)\n",
            "            }\n",
            "        }\n",
            "    };\n",
            "}\n\n",
        ]
        for identity in sorted(self.ids):
            label = snake(identity.removesuffix("Id")).replace("_", " ")
            chunks.append(f'current_id!({identity}, "{label}");\n')
        path.write_text("".join(chunks), encoding="utf-8")

    def _render_enum(self, name: str, schema: dict[str, Any]) -> str:
        variants: list[tuple[str, str]] = []
        used: set[str] = set()
        for wire in schema.get("enum", []):
            variant = pascal(str(wire))
            base = variant
            suffix = 2
            while variant in used or variant == "Unknown":
                variant = f"{base}{suffix}"
                suffix += 1
            used.add(variant)
            variants.append((str(wire), variant))
        lines = [
            f"/// Current provider values for `{name}`.\n",
            "///\n",
            "/// Unknown response values are preserved for forward compatibility but cannot\n",
            "/// be serialized into a request.\n",
            "#[derive(Clone, Debug, Eq, Hash, PartialEq)]\n",
            "#[non_exhaustive]\n",
            f"pub enum {name} {{\n",
        ]
        for wire, variant in variants:
            lines.extend([f"    /// Provider value `{wire}`.\n", f"    {variant},\n"])
        lines.extend([
            "    /// A provider value added after the pinned specification.\n",
            "    Unknown(String),\n",
            "}\n\n",
            f"impl {name} {{\n",
            "    /// Returns the exact provider spelling.\n",
            "    #[must_use]\n",
            "    pub fn as_str(&self) -> &str {\n",
            "        match self {\n",
        ])
        for wire, variant in variants:
            lines.append(f'            Self::{variant} => "{rust_string(wire)}",\n')
        lines.extend([
            "            Self::Unknown(value) => value,\n",
            "        }\n",
            "    }\n",
            "}\n\n",
            f"impl serde::Serialize for {name} {{\n",
            "    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>\n",
            "    where\n",
            "        S: serde::Serializer,\n",
            "    {\n",
            "        if matches!(self, Self::Unknown(_)) {\n",
            "            return Err(serde::ser::Error::custom(\"undocumented enum values cannot be sent\"));\n",
            "        }\n",
            "        serializer.serialize_str(self.as_str())\n",
            "    }\n",
            "}\n\n",
            f"impl<'de> serde::Deserialize<'de> for {name} {{\n",
            "    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>\n",
            "    where\n",
            "        D: serde::Deserializer<'de>,\n",
            "    {\n",
            "        let value = <String as serde::Deserialize>::deserialize(deserializer)?;\n",
            "        Ok(match value.as_str() {\n",
        ])
        for wire, variant in variants:
            lines.append(f'            "{rust_string(wire)}" => Self::{variant},\n')
        lines.extend([
            "            _ => Self::Unknown(value),\n",
            "        })\n",
            "    }\n",
            "}\n\n",
        ])
        return "".join(lines)

    def _mutation_responses_for_module(self, module: str) -> set[str]:
        names: set[str] = set()
        for item in self.operations:
            if item["class"] != "mutation" or item["path"] in SPECIAL_ENDPOINTS:
                continue
            schema = (
                item["operation"]
                .get("responses", {})
                .get("200", {})
                .get("content", {})
                .get("application/json", {})
                .get("schema")
            )
            name = ref_name(schema or {})
            if name is not None and self.owners[name] == module:
                names.add(name)
        return names

    def _render_mutation_response(self, name: str, owner: str) -> str:
        schema = self.schemas[name]
        properties: dict[str, dict[str, Any]] = schema.get("properties", {})
        required = set(schema.get("required", []))
        indicators: list[tuple[str, str, bool]] = []
        for wire, field_schema in properties.items():
            field = snake(wire)
            reference = ref_name(field_schema)
            is_required = wire in required
            if wire in {"ok", "success"} and field_schema.get("type") == "boolean":
                indicators.append((field, "bool", is_required))
                continue
            if wire in {"failureReason", "errorCode"} and reference is not None:
                enum_values = self.schemas.get(reference, {}).get("enum", [])
                if "Success" in enum_values:
                    indicators.append((field, self._qualified_type(reference, owner), is_required))
        custom_expression = MUTATION_CUSTOM_EVIDENCE.get(name)
        if name in MUTATION_ALWAYS_EVIDENCE:
            evidence_any = ["true"]
        else:
            unknown = set(MUTATION_ANY_EVIDENCE.get(name, ())) - properties.keys()
            if unknown:
                raise ValueError(f"unknown mutation evidence fields for {name}: {sorted(unknown)}")
            evidence_any = [
                "true" if wire in required else f"self.{snake(wire)}.is_some()"
                for wire in MUTATION_ANY_EVIDENCE.get(name, ())
            ]
        unknown = set(MUTATION_ALL_EVIDENCE.get(name, ())) - properties.keys()
        if unknown:
            raise ValueError(f"unknown mutation evidence fields for {name}: {sorted(unknown)}")
        evidence_all = [
            "true" if wire in required else f"self.{snake(wire)}.is_some()"
            for wire in MUTATION_ALL_EVIDENCE.get(name, ())
        ]
        configured = (
            name in MUTATION_ALWAYS_EVIDENCE
            or name in MUTATION_ANY_EVIDENCE
            or name in MUTATION_CUSTOM_EVIDENCE
            or name in MUTATION_ALL_EVIDENCE
        )
        if not configured and not indicators:
            raise ValueError(f"mutation response {name} has no reviewed completion policy")

        def or_expression(parts: list[str]) -> str:
            if "true" in parts:
                return "true"
            normalized = list(dict.fromkeys(part for part in parts if part != "false"))
            if not normalized:
                return "false"
            return " || ".join(normalized)

        def evidence_result(
            expression: str,
            if_true: str,
            if_false: str,
            indent: str,
        ) -> list[str]:
            if expression == "true":
                return [f"{indent}crate::client::MutationOutcome::{if_true}\n"]
            if expression == "false":
                return [f"{indent}crate::client::MutationOutcome::{if_false}\n"]
            return [
                f"{indent}if {expression} {{\n",
                f"{indent}    crate::client::MutationOutcome::{if_true}\n",
                f"{indent}}} else {{\n",
                f"{indent}    crate::client::MutationOutcome::{if_false}\n",
                f"{indent}}}\n",
            ]

        any_expression = or_expression(evidence_any)
        all_expression = " && ".join(part for part in evidence_all if part != "true") or (
            "true" if evidence_all else "false"
        )
        evidence_expression = or_expression(
            [any_expression, all_expression, custom_expression or "false"]
        )
        lines = [
            f"impl crate::client::DocumentedMutationResponse for {name} {{\n",
            "    fn mutation_outcome(&self) -> crate::client::MutationOutcome {\n",
        ]
        if indicators:
            success_parts: list[str] = []
            present_parts: list[str] = []
            for field, kind, is_required in indicators:
                if kind == "bool":
                    success_parts.append(f"self.{field}" if is_required else f"self.{field} == Some(true)")
                    present_parts.append("true" if is_required else f"self.{field}.is_some()")
                elif is_required:
                    success_parts.append(f"matches!(&self.{field}, {kind}::Success)")
                    present_parts.append("true")
                else:
                    success_parts.append(
                        f"self.{field}.as_ref().is_some_and(|value| matches!(value, {kind}::Success))"
                    )
                    present_parts.append(f"self.{field}.is_some()")
            success = or_expression(success_parts)
            present = or_expression(present_parts)
            needs_evidence = (
                any(kind != "bool" for _, kind, _ in indicators)
                and evidence_expression != "false"
            )
            lines.extend([
                f"        let indicator_success = {success};\n",
                f"        let indicator_present = {present};\n",
            ])
            lines.append("        if indicator_success {\n")
            if needs_evidence:
                lines.extend(evidence_result(evidence_expression, "Success", "Ambiguous", "            "))
            else:
                lines.append("            crate::client::MutationOutcome::Success\n")
            lines.append("        } else if indicator_present {\n")
            lines.extend(evidence_result(evidence_expression, "Ambiguous", "Rejected", "            "))
            lines.append("        } else {\n")
            lines.extend(evidence_result(evidence_expression, "Success", "Ambiguous", "            "))
            lines.extend([
                "        }\n",
                "    }\n\n",
                "    fn has_success_evidence(&self) -> bool {\n",
                f"        {or_expression([success, evidence_expression])}\n",
                "    }\n",
                "}\n\n",
            ])
        else:
            lines.extend(evidence_result(evidence_expression, "Success", "Ambiguous", "        "))
            lines.extend([
                "    }\n\n",
                "    fn has_success_evidence(&self) -> bool {\n",
                f"        {evidence_expression}\n",
                "    }\n",
                "}\n\n",
            ])
        return "".join(lines)

    def _render_struct(
        self,
        name: str,
        schema: dict[str, Any],
        owner: str,
        id_context: str | None = None,
        validate_required: bool | None = None,
    ) -> str:
        properties: dict[str, dict[str, Any]] = schema.get("properties", {})
        required = set(schema.get("required", []))
        fields: list[dict[str, Any]] = []
        for wire, field_schema in properties.items():
            rust_type, kind = self.schema_type(field_schema, owner, id_context or name, wire)
            if name in SECRET_NAME_SCHEMAS and wire == "name":
                rust_type, kind = "crate::api::current::SecretValue", "secret"
            fields.append(
                {
                    "wire": wire,
                    "name": snake(wire),
                    "type": rust_type,
                    "kind": kind,
                    "required": wire in required,
                    # Secrets never get a public value getter. Selected
                    # reviewed validators get a crate-private borrowed view.
                    "secret_accessor": (name, wire) in SECRET_ACCESSOR_FIELDS,
                }
            )
        model_doc = (
            f"/// Documentation-blocked current wire placeholder `{name}`.\n"
            "///\n"
            "/// The pinned contract publishes no member grammar. Deserialization\n"
            "/// therefore accepts only an empty object and fails closed on provider data.\n"
            if name in INCOMPLETE_COMPONENT_SCHEMAS
            else f"/// Current wire model `{name}`.\n"
        )
        lines = [
            model_doc,
            (
                "#[derive(Clone, Debug, serde::Deserialize)]\n"
                if name in SECRET_RESPONSE_SCHEMAS
                else "#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]\n"
            ),
            (
                "#[serde(deny_unknown_fields)]\n"
                if name in INCOMPLETE_COMPONENT_SCHEMAS
                else ""
            ),
            "#[non_exhaustive]\n",
            f"pub struct {name} {{\n",
        ]
        for field in fields:
            lines.extend(self._field_attributes(field))
            rust_type = field["type"] if field["required"] else f"Option<{field['type']}>"
            lines.append(f"    {field['name']}: {rust_type},\n")
        lines.extend(["}\n\n", f"impl {name} {{\n"])
        validate_required = name in self.request_schemas if validate_required is None else validate_required
        for field in fields:
            lines.extend(self._render_getter(field))
        if name in SECRET_RESPONSE_SCHEMAS:
            lines.append("}\n\n")
            return "".join(lines)
        lines.extend([
            f"    /// Starts a builder for [`{name}`].\n",
            f"    pub fn builder() -> {name}Builder {{ {name}Builder::default() }}\n\n",
        ])
        lines.extend(
            [
                "}\n\n",
                f"/// Builder for [`{name}`].\n",
                "#[must_use = \"a wire-model builder does nothing until build is called\"]\n",
            ]
        )
        lines.extend([
            "#[derive(Clone, Debug, Default)]\n",
            f"pub struct {name}Builder {{\n",
        ])
        for field in fields:
            lines.append(f"    {field['name']}: Option<{field['type']}>,\n")
        lines.extend(["}\n\n", f"impl {name}Builder {{\n"])
        for field in fields:
            lines.extend(self._render_setter(field))
        lines.extend([
            f"    /// Validates required fields and builds [`{name}`].\n",
            "    ///\n",
            "    /// # Errors\n",
            "    ///\n",
            "    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.\n",
            f"    pub fn build(self) -> Result<{name}, crate::api::current::BuildError> {{\n",
        ])
        for field in fields:
            if not field["required"]:
                continue
            lines.append(
                f"        let {field['name']} = self.{field['name']}.ok_or(crate::api::current::BuildError::missing(\"{rust_string(field['wire'])}\"))?;\n"
            )
            if validate_required and field["kind"] == "array":
                lines.extend([
                    f"        if {field['name']}.is_empty() {{\n",
                    f"            return Err(crate::api::current::BuildError::invalid(\"{rust_string(field['wire'])}\", \"must not be empty\"));\n",
                    "        }\n",
                ])
            if validate_required and field["kind"] == "string":
                lines.extend([
                    f"        if {field['name']}.is_empty() || {field['name']}.trim() != {field['name']} {{\n",
                    f"            return Err(crate::api::current::BuildError::invalid(\"{rust_string(field['wire'])}\", \"must be non-empty and have no surrounding whitespace\"));\n",
                    "        }\n",
                ])
        lines.append(f"        Ok({name} {{\n")
        for field in fields:
            if field["required"]:
                lines.append(f"            {field['name']},\n")
            else:
                lines.append(f"            {field['name']}: self.{field['name']},\n")
        lines.extend(["        })\n", "    }\n", "}\n\n"])
        return "".join(lines)

    @staticmethod
    def _field_attributes(field: dict[str, Any]) -> list[str]:
        lines = [f"    #[serde(rename = \"{rust_string(field['wire'])}\""]
        if not field["required"]:
            lines[0] += ", default, skip_serializing_if = \"Option::is_none\""
        lines[0] += ")]\n"
        if field["kind"] == "decimal":
            adapter = "crate::decimal" if field["required"] else "crate::decimal::option"
            lines.append(f"    #[serde(with = \"{adapter}\")]\n")
        return lines

    @staticmethod
    def _render_getter(field: dict[str, Any]) -> list[str]:
        wire = rust_string(field["wire"])
        name = field["name"]
        kind = field["kind"]
        required = field["required"]
        if kind == "secret":
            lines = [
                f"    /// Reports whether secret field `{wire}` is present.\n",
                "    #[must_use]\n",
                f"    pub const fn has_{name}(&self) -> bool {{ {('true' if required else f'self.{name}.is_some()')} }}\n\n",
            ]
            if field.get("secret_accessor", False):
                lines.extend(
                    [
                        f"    pub(crate) fn {name}_secret(&self) -> {'&' if required else 'Option<&'}crate::api::current::SecretValue{'' if required else '>'} {{\n",
                        f"        {'&self.' + name if required else 'self.' + name + '.as_ref()'}\n",
                        "    }\n\n",
                    ]
                )
            return lines
        if kind == "array":
            inner = field["type"][4:-1]
            if required:
                signature, body = f"&[{inner}]", f"&self.{name}"
            else:
                signature, body = f"Option<&[{inner}]>", f"self.{name}.as_deref()"
        elif kind == "string":
            if required:
                signature, body = "&str", f"&self.{name}"
            else:
                signature, body = "Option<&str>", f"self.{name}.as_deref()"
        elif required:
            signature, body = f"&{field['type']}", f"&self.{name}"
        else:
            signature, body = f"Option<&{field['type']}>", f"self.{name}.as_ref()"
        return [
            f"    /// Returns wire field `{wire}`.\n",
            "    #[must_use]\n",
            f"    pub fn {name}(&self) -> {signature} {{ {body} }}\n\n",
        ]

    @staticmethod
    def _render_setter(field: dict[str, Any]) -> list[str]:
        name = field["name"]
        wire = rust_string(field["wire"])
        rust_type = field["type"]
        if field["kind"] == "string":
            argument = "impl Into<String>"
            value = "value.into()"
        else:
            argument = rust_type
            value = "value"
        return [
            f"    /// Sets wire field `{wire}`.\n",
            f"    pub fn {name}(mut self, value: {argument}) -> Self {{\n",
            f"        self.{name} = Some({value});\n",
            "        self\n",
            "    }\n\n",
        ]

    def _render_request_validation(self, name: str, schema: dict[str, Any]) -> str:
        properties: dict[str, dict[str, Any]] = schema.get("properties", {})
        required = set(schema.get("required", []))
        lines = [
            f"impl crate::api::current::support::CurrentRequest for {name} {{\n",
            "    fn validate_current(&self) -> Result<(), crate::Error> {\n",
        ]
        for wire in sorted(required):
            field_schema = properties[wire]
            field = snake(wire)
            if field_schema.get("type") == "array":
                lines.extend([
                    f"        if self.{field}.is_empty() {{\n",
                    f"            return Err(crate::Error::InvalidRequest {{ field: \"{rust_string(wire)}\", reason: \"must not be empty\" }});\n",
                    "        }\n",
                ])
            elif (
                field_schema.get("type") == "string"
                and field_schema.get("format") != "date-time"
                and wire not in SECRET_FIELDS
                and wire not in STRING_NEWTYPE_FIELDS
                and not (
                    name in SECRET_NAME_SCHEMAS and wire == "name"
                )
            ):
                lines.extend([
                    f"        if self.{field}.is_empty() || self.{field}.trim() != self.{field} {{\n",
                    f"            return Err(crate::Error::InvalidRequest {{ field: \"{rust_string(wire)}\", reason: \"must be non-empty and have no surrounding whitespace\" }});\n",
                    "        }\n",
                ])
        lines.extend(["        Ok(())\n", "    }\n", "}\n\n"])
        return "".join(lines)

    def _render_endpoint(self, item: dict[str, Any]) -> str:
        path = item["path"]
        operation = item["operation"]
        resource_name = snake(path.strip("/").split("/", 1)[0])
        method_name = snake(operation["operationId"])
        if not method_name.startswith(f"{resource_name}_"):
            method_name = f"{resource_name}_{method_name}"
        query_parameters = [
            parameter
            for parameter in operation.get("parameters", [])
            if parameter.get("in") == "query"
        ]
        request_ref = ref_name(
            operation.get("requestBody", {})
            .get("content", {})
            .get("application/json", {})
            .get("schema", {})
        )
        response_type = self._response_type(item)
        module = item["module"]
        query_name = f"{pascal(operation['operationId'])}Query"
        chunks: list[str] = []
        if query_parameters:
            chunks.append(self._render_query(query_name, query_parameters, module, path))
        arguments: list[str] = []
        call: str
        if query_parameters:
            arguments.append(f"query: &{query_name}")
        if request_ref is not None:
            request_type = self._qualified_type(request_ref, module)
            arguments.append(f"request: &{request_type}")
        args = ", ".join(["&self", *arguments])
        if item["method"] == "get":
            call = (
                f'self.get_current("{path}", query).await'
                if query_parameters
                else f'self.get_without_query("{path}").await'
            )
            return_type = response_type
        elif item["class"] == "query":
            call = (
                f'self.post_query("{path}", request).await'
                if request_ref is not None
                else f'self.post_query_without_body("{path}").await'
            )
            return_type = response_type
        else:
            call = (
                f'self.post_documented_mutation("{path}", request).await'
                if request_ref is not None
                else f'self.post_documented_mutation_without_body("{path}").await'
            )
            return_type = response_type
        chunks.extend([
            "impl crate::Client {\n",
            f"    /// Calls the current `{item['method'].upper()} {path}` operation.\n",
            "    ///\n",
            "    /// # Errors\n",
            "    ///\n",
            "    /// Returns a typed local validation, authentication, rate, transport,\n",
            "    /// provider-control, response-bound, or decoding failure. Mutations may\n",
            "    /// additionally return an ambiguous outcome requiring reconciliation.\n",
            f"    pub async fn {method_name}({args}) -> Result<{return_type}, crate::Error> {{\n",
        ])
        if request_ref is not None:
            chunks.append(
                "        crate::api::current::support::CurrentRequest::validate_current(request)?;\n"
            )
        chunks.extend([f"        {call}\n", "    }\n", "}\n\n"])
        return "".join(chunks)

    def _render_query(
        self,
        name: str,
        parameters: list[dict[str, Any]],
        owner: str,
        path: str,
    ) -> str:
        properties: dict[str, Any] = {}
        required: list[str] = []
        resource = pascal(path.strip("/").split("/", 1)[0])
        for parameter in parameters:
            properties[parameter["name"]] = parameter["schema"]
            if parameter.get("required", False):
                required.append(parameter["name"])
        schema = {"type": "object", "properties": properties, "required": required}
        rendered = self._render_struct(name, schema, owner, resource, True).replace(
            f"Current wire model `{name}`", f"Typed query parameters for `{path}`"
        )
        lines = [
            rendered,
            f"impl crate::api::current::support::CurrentQuery for {name} {{\n",
            "    fn encode_pairs(&self) -> Result<Vec<(&'static str, String)>, crate::Error> {\n",
        ]
        for parameter in parameters:
            if not parameter.get("required", False):
                continue
            wire = parameter["name"]
            field = snake(wire)
            schema = parameter["schema"]
            if schema.get("type") == "array":
                lines.extend([
                    f"        if self.{field}.is_empty() {{\n",
                    f"            return Err(crate::Error::InvalidRequest {{ field: \"{rust_string(wire)}\", reason: \"must not be empty\" }});\n",
                    "        }\n",
                ])
            elif (
                schema.get("type") == "string"
                and schema.get("format") != "date-time"
                and wire not in STRING_NEWTYPE_FIELDS
            ):
                lines.extend([
                    f"        if self.{field}.is_empty() || self.{field}.trim() != self.{field} {{\n",
                    f"            return Err(crate::Error::InvalidRequest {{ field: \"{rust_string(wire)}\", reason: \"must be non-empty and have no surrounding whitespace\" }});\n",
                    "        }\n",
                ])
        lines.append("        let mut pairs = Vec::new();\n")
        for parameter in parameters:
            wire = parameter["name"]
            field = snake(wire)
            required_field = parameter.get("required", False)
            array = parameter["schema"].get("type") == "array"
            if array and required_field:
                lines.extend(
                    [
                        f"        for value in &self.{field} {{\n",
                        f"            crate::api::current::support::push_query_value(&mut pairs, \"{rust_string(wire)}\", value)?;\n",
                        "        }\n",
                    ]
                )
            elif array:
                lines.extend(
                    [
                        f"        if let Some(values) = &self.{field} {{\n",
                        "            for value in values {\n",
                        f"                crate::api::current::support::push_query_value(&mut pairs, \"{rust_string(wire)}\", value)?;\n",
                        "            }\n",
                        "        }\n",
                    ]
                )
            elif required_field:
                lines.append(
                    f"        crate::api::current::support::push_query_value(&mut pairs, \"{rust_string(wire)}\", &self.{field})?;\n"
                )
            else:
                lines.extend(
                    [
                        f"        if let Some(value) = &self.{field} {{\n",
                        f"            crate::api::current::support::push_query_value(&mut pairs, \"{rust_string(wire)}\", value)?;\n",
                        "        }\n",
                    ]
                )
        lines.extend(["        Ok(pairs)\n", "    }\n", "}\n\n"])
        return "".join(lines)

    def _response_type(self, item: dict[str, Any]) -> str:
        schema = (
            item["operation"]
            .get("responses", {})
            .get("200", {})
            .get("content", {})
            .get("application/json", {})
            .get("schema")
        )
        path = item["path"]
        if schema is None:
            if path not in UNSPECIFIED_RESPONSES:
                raise ValueError(f"unresolved response contract for {path}")
            return "crate::api::current::DocumentedAcknowledgement"
        reference = ref_name(schema)
        if reference is not None:
            return self._qualified_type(reference, item["module"])
        if schema.get("type") == "array":
            item_ref = ref_name(schema["items"])
            if item_ref is None:
                raise ValueError(f"non-reference response array for {path}")
            return f"Vec<{self._qualified_type(item_ref, item['module'])}>"
        raise ValueError(f"unsupported response for {path}: {schema}")

    def _qualified_type(self, name: str, current_module: str) -> str:
        owner = self.owners[name]
        return name if owner == current_module else f"super::{owner}::{name}"

    @staticmethod
    def _surface(item: dict[str, Any]) -> str:
        if item["path"] in DOCUMENTATION_BLOCKED_ENDPOINTS:
            return "DocumentationBlocked"
        if item["path"] in SPECIAL_ENDPOINTS:
            return "Specialized"
        if item["class"] == "mutation":
            return "Modeled"
        return "Generated"

    @staticmethod
    def _response_contract(item: dict[str, Any]) -> str:
        if item["path"] in UNSPECIFIED_RESPONSES:
            return "Unspecified"
        if item["path"] in INCOMPLETE_RESPONSE_ENDPOINTS:
            return "Incomplete"
        return "Typed"

    @staticmethod
    def _response_label(item: dict[str, Any]) -> str:
        schema = (
            item["operation"]
            .get("responses", {})
            .get("200", {})
            .get("content", {})
            .get("application/json", {})
            .get("schema")
        )
        if schema is None:
            return "Unspecified"
        reference = ref_name(schema)
        if reference is not None:
            return reference
        if schema.get("type") == "array":
            item_reference = ref_name(schema.get("items", {}))
            if item_reference is not None:
                return f"[{item_reference}]"
        return "Typed inline schema"

    def render_coverage(self, path: Path) -> None:
        class_counts = {
            name: sum(item["class"] == name for item in self.operations)
            for name in ("query", "mutation", "lifecycle")
        }
        surface_counts = {
            name: sum(self._surface(item) == name for item in self.operations)
            for name in ("Generated", "Specialized", "Modeled", "DocumentationBlocked")
        }
        response_counts = {
            name: sum(self._response_contract(item) == name for item in self.operations)
            for name in ("Typed", "Unspecified", "Incomplete")
        }
        callable_count = surface_counts["Generated"] + surface_counts["Specialized"]
        lines = [
            markdown_header(),
            "# Current Tradovate REST API coverage\n\n",
            "This matrix is generated only from the hash-pinned current Tradovate Partner "
            "`OpenAPI` snapshot. Older explorers, archived examples, and guide-only "
            "fragments are not implementation inputs.\n\n",
            "## Audited result\n\n",
            f"- Snapshot: `spec/official/openapi-2026-08-21.json`\n",
            f"- Source: [{SOURCE_URL}]({SOURCE_URL})\n",
            f"- SHA-256: `{EXPECTED_SHA256}`\n",
            f"- Operations: **{len(self.operations)}**\n",
            f"- Component schemas: **{len(self.schemas)}**\n",
            f"- Callable operations: **{callable_count}**\n",
            f"- Documentation-blocked operations: **{surface_counts['DocumentationBlocked']}**\n\n",
            "Every operation is represented exactly once. `Generated` operations use the "
            "bounded typed query executor. `Specialized` operations use reviewed handwritten "
            "validation, authentication/lifecycle ownership, rate admission, request-aware "
            "completion evidence, and mutation ambiguity fencing. `DocumentationBlocked` "
            "means the current pin does not contain enough information to transmit or decode "
            "the operation without guessing; the crate deliberately exposes no raw escape "
            "hatch.\n\n",
            "| Dimension | Count |\n",
            "| --- | ---: |\n",
            f"| Query | {class_counts['query']} |\n",
            f"| Mutation | {class_counts['mutation']} |\n",
            f"| Lifecycle | {class_counts['lifecycle']} |\n",
            f"| Generated | {surface_counts['Generated']} |\n",
            f"| Specialized | {surface_counts['Specialized']} |\n",
            f"| Modeled but not callable | {surface_counts['Modeled']} |\n",
            f"| Documentation blocked | {surface_counts['DocumentationBlocked']} |\n",
            f"| Typed response contract | {response_counts['Typed']} |\n",
            f"| Unspecified response contract | {response_counts['Unspecified']} |\n",
            f"| Incomplete response contract | {response_counts['Incomplete']} |\n\n",
            "## Current-contract blockers\n\n",
            "These rows remain unavailable until a future reviewed current Partner snapshot "
            "supplies the missing grammar or schema.\n\n",
            "| Operation | Reason |\n",
            "| --- | --- |\n",
        ]
        operation_by_path = {item["path"]: item for item in self.operations}
        for blocked_path, reason in sorted(DOCUMENTATION_BLOCKED_REASONS.items()):
            item = operation_by_path[blocked_path]
            lines.append(
                f"| `{item['method'].upper()} {blocked_path}` | {reason} |\n"
            )
        lines.extend([
            "\n## Component-schema blockers\n\n",
            "The generated placeholder objects deny unknown fields, so provider data cannot "
            "be silently discarded.\n\n",
            "| Component | Affected operation | Missing contract |\n",
            "| --- | --- | --- |\n",
            "| `ExtraPreTradeRiskProducts` | `POST /order/dryrun` | object value schema |\n",
            "| `ExtraPreTradeRiskContracts` | `POST /order/dryrun` | object value schema |\n",
            "| `RollContractsResponseContracts` | `POST /contract/rollcontracts` | map key and value schemas |\n\n",
            "## Full current operation matrix\n\n",
            "The class is semantic: token/session transitions and `user/syncrequest` are "
            "lifecycle operations even when their HTTP method resembles a query. Response "
            "contract status reports only what the pinned `OpenAPI` publishes.\n\n",
        ])
        capability_order = list(TAG_MODULE)
        for capability in capability_order:
            tagged = [
                item
                for item in self.operations
                if item["operation"]["tags"][0] == capability
            ]
            if not tagged:
                continue
            lines.extend([
                f"### {capability} ({len(tagged)})\n\n",
                "| Operation | Operation ID | Class | Surface | Response | Contract |\n",
                "| --- | --- | --- | --- | --- | --- |\n",
            ])
            for item in sorted(tagged, key=lambda value: (value["path"], value["method"])):
                operation = item["operation"]
                lines.append(
                    f"| `{item['method'].upper()} {item['path']}` | "
                    f"`{operation['operationId']}` | {item['class']} | "
                    f"{self._surface(item)} | `{self._response_label(item)}` | "
                    f"{self._response_contract(item)} |\n"
                )
            lines.append("\n")
        # Keep generated Markdown compatible with `git diff --check`: exactly
        # one newline terminates the file, regardless of how many capability
        # sections were rendered above.
        path.write_text("".join(lines).rstrip("\n") + "\n", encoding="utf-8")

    def _render_manifest(self, path: Path) -> None:
        lines = [header(), "//! Exhaustive current REST operation manifest.\n\n", "use crate::api::current::{HttpMethod, Operation, OperationClass, OperationSurface, ResponseContract};\n\n", "/// Every operation in the pinned current Partner `OpenAPI`.\n", "pub const OPERATIONS: &[Operation] = &[\n"]
        for item in self.operations:
            operation = item["operation"]
            response_contract = f"ResponseContract::{self._response_contract(item)}"
            operation_class = pascal(item["class"])
            surface = self._surface(item)
            lines.extend([
                "    Operation {\n",
                f"        method: HttpMethod::{pascal(item['method'])},\n",
                f'        path: "{item["path"]}",\n',
                f'        id: "{rust_string(operation["operationId"])}",\n',
                f'        capability: "{rust_string(operation["tags"][0])}",\n',
                f"        class: OperationClass::{operation_class},\n",
                f"        surface: OperationSurface::{surface},\n",
                f"        response_contract: {response_contract},\n",
                "    },\n",
            ])
        lines.append("];\n")
        path.write_text("".join(lines), encoding="utf-8")

    @staticmethod
    def _render_mod(path: Path, modules: list[str]) -> None:
        lines = [header(), "//! Generated modules for the pinned current Partner contract.\n\n"]
        for module in modules:
            lines.extend([f"/// {MODULE_DOC[module]}\n", f"pub mod {module};\n"])
        lines.extend([
            "/// Validated generated provider identities.\n",
            "pub mod ids;\n",
            "mod manifest;\n\n",
            "pub use manifest::OPERATIONS;\n",
        ])
        path.write_text("".join(lines), encoding="utf-8")


def rust_string(value: str) -> str:
    return value.replace("\\", "\\\\").replace('"', '\\"').replace("\n", "\\n")


def generate(check: bool) -> int:
    raw = SPEC.read_bytes()
    digest = hashlib.sha256(raw).hexdigest()
    if digest != EXPECTED_SHA256:
        raise ValueError(f"pinned spec hash changed: expected {EXPECTED_SHA256}, got {digest}")
    spec = json.loads(raw)
    with tempfile.TemporaryDirectory(prefix="tradovate-openapi-") as temporary:
        generated = Path(temporary) / "generated"
        coverage = Path(temporary) / "api-coverage-rest.md"
        generator = Generator(spec)
        generator.render(generated)
        generator.render_coverage(coverage)
        if check:
            if not OUTPUT.exists() or not directories_equal(generated, OUTPUT):
                print("generated current API is stale; run tools/generate_openapi.py", file=sys.stderr)
                return 1
            if not COVERAGE_OUTPUT.exists() or coverage.read_bytes() != COVERAGE_OUTPUT.read_bytes():
                print("generated REST coverage is stale; run tools/generate_openapi.py", file=sys.stderr)
                return 1
            return 0
        if OUTPUT.exists():
            shutil.rmtree(OUTPUT)
        shutil.copytree(generated, OUTPUT)
        shutil.copyfile(coverage, COVERAGE_OUTPUT)
    return 0


def directories_equal(left: Path, right: Path) -> bool:
    left_files = sorted(path.relative_to(left) for path in left.rglob("*") if path.is_file())
    right_files = sorted(path.relative_to(right) for path in right.rglob("*") if path.is_file())
    return left_files == right_files and all(
        (left / relative).read_bytes() == (right / relative).read_bytes()
        for relative in left_files
    )


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--check", action="store_true", help="fail when checked-in output is stale")
    arguments = parser.parse_args()
    return generate(arguments.check)


if __name__ == "__main__":
    raise SystemExit(main())
