<!--
Copyright (C) 2026 Kevin Monaghan. All rights reserved.

This file is proprietary and confidential.
Unauthorized copying, use, modification, distribution, or disclosure of this file,
via any medium, is strictly prohibited except under a written agreement with the
copyright owner.
-->

# tradovate-rs

Private, provider-native Rust client development for the official Tradovate API.
The intended Cargo package name is `tradovate-client`; the `-rs` suffix belongs to
the repository name, not the crate import path.

This repository is proprietary to Kevin Monaghan and hosted privately under
`SharurTrading`. It is an independent client and is not affiliated with, endorsed
by, or sponsored by Tradovate. Provider access and use remain subject to Tradovate's
terms and entitlements.

The sole REST implementation authority is the current
[Tradovate Partner OpenAPI](https://partner.tradovate.com/openapi.json), pinned in
this repository with its retrieval date and digest. Repository documentation records
the additional safety and lifecycle guarantees supplied by this client.

## Status

The pinned current Partner REST contract is inventoried operation-for-operation.
Every operation is either callable through a typed reviewed surface or carries an
explicit documentation-blocked record. A manifest entry or generated model does not
by itself make a state-changing operation callable.

| Capability | Current coverage |
| --- | --- |
| Authentication | Direct API-key login, OAuth token exchange plus `/auth/me`, expiry/revision-fenced renewal, redacted session metadata, and client-bound delayed penalty retry |
| Current REST contract | All 350 operations and all 278 component schemas in the 2026-08-21 Partner OpenAPI pin |
| Operation classes | 271 query, 73 mutation, and 6 lifecycle operations |
| Public surfaces | 263 generated query methods, 71 reviewed specialized methods, 0 modeled-only operations, and 16 explicit documentation-blocked operations |
| Response contracts | 338 typed, 11 unspecified by the pin, and 1 incomplete in the pin |
| Callable documented operations | 334 typed operations: 333 direct REST methods plus `/user/syncrequest` owned by the realtime lifecycle; every non-callable row has a provider-contract blocker in the exhaustive manifest |
| Realtime transport | Production-public bounded, generation-fenced connection with typed user, shutdown, market-data, chart, and recovery events |
| Market data | Quote/DOM/histogram subscriptions by `Symbol` or `ContractId`, exact typed payloads, validated chart requests/cancellation, bars, and checked compact ticks |

The 16 REST blockers cannot become safe callable operations until Tradovate publishes
the missing request, response, or completion evidence identified for each row.
Configurable B2B split synchronization and replay control/clock payloads are also
withheld because the current Partner documentation omits their completion or payload
contracts; `realtime::DOCUMENTATION_BLOCKED_CAPABILITIES` records that boundary. A
documented mutation is exposed only after its request validation and
success/rejection evidence have been reviewed. Public typed realtime coverage, its
documented gaps, and recovery responsibilities are tracked separately from the REST
manifest.

## Design goals

- Standalone provider client with no dependency on SHARUR or another consumer.
- Exact `rust_decimal::Decimal` values for financial data.
- Validated provider IDs, timestamps, enums, and request builders.
- Redacted credentials and revision-fenced access-token lifecycle.
- Caller-owned Tokio runtime with no blocking network path.
- Bounded HTTP bodies, WebSocket frames, queues, pending requests, and timeouts.
- Explicit query versus money-moving mutation policies.
- No automatic retry when an order or position mutation may have reached Tradovate.
- Generation-fenced realtime lifecycle and explicit transport-gap recovery.
- Tradovate SockJS-derived framing, request correlation, heartbeat, and user bootstrap.
- Typed HTTP-200 business failures and provider penalty-ticket handling.
- Deterministic generation from one hash-pinned current Partner OpenAPI snapshot.
- A public, exhaustive operation manifest for coverage and audit tooling.
- Deterministic synthetic tests; any future live probe must be deliberate, ignored,
  and read-only.

## Quick start

The caller owns credentials, their storage, and the Tokio runtime. The crate does
not read `.env` files or discover secrets from ambient process state.

```rust
use tradovate_client::{Client, Environment, auth::Credentials};

async fn connect() -> Result<(), Box<dyn std::error::Error>> {
    let credentials = Credentials::builder("user", "dedicated-api-password")
        .build()?;

    let client = Client::builder(Environment::Demo).build()?;
    let session = client.authenticate(&credentials).await?;
    println!("authenticated user {}", session.user_id());
    Ok(())
}
```

The current Partner schema requires only `name` and `password`. `hibpCheck`,
`appId`, `appVersion`, `cid`, `sec`, and `deviceId` are optional and are omitted
from the request rather than sent as `null` unless their builder methods are
called. Partner API-key deployments may still require the metadata issued for that
key operationally; provide those exact values when applicable:

```rust
use tradovate_client::{DeviceId, auth::Credentials};

fn credentials() -> Result<Credentials, Box<dyn std::error::Error>> {
    let credentials = Credentials::builder("user", "dedicated-api-password")
        .app_id("registered-api-key-name")
        .app_version("1.0")
        .numeric_client_id(123)
        .secret("issued-api-key-secret")
        .device_id(DeviceId::new("stable-device-id")?)
        .hibp_check(true)
        .build()?;
    Ok(credentials)
}
```

## Architecture

The initial repository is one focused library crate. It is split by capability and
ownership rather than placed into giant client/model files:

```text
src/
  lib.rs                crate documentation and selective public facade
  error.rs              public-safe typed errors
  environment.rs        validated environment and endpoint configuration
  ids.rs                validated provider identity newtypes
  decimal.rs            exact provider JSON decimal boundary
  auth/                  credentials and revision-fenced session state
  client/                builder and shared HTTP execution policy
  rate_limit/            shared provider admission and cooldown state
  api/current/           complete current Partner REST surface by capability
    generated/           checked-in models, builders, methods, and manifest
    mutations/           reviewed state-changing and lifecycle implementations
  realtime/              connection, codec, lifecycle, events, flow control
    user_stream/         typed bootstrap and provider entity deltas
    market_data/         exact quote, DOM, and histogram payloads
    chart/               validated requests, bars, and compact ticks
```

The crate owns provider transport and provider-native wire contracts. Consuming
applications own routing policy, portfolios, risk, storage, GUI state, canonical
subscriptions, and translation into their own domain types.

The architecture decision and rejected alternatives are recorded in
[`docs/adr/0001-architecture.md`](docs/adr/0001-architecture.md).

## Safety model

### Authentication and secrets

Callers acquire and inject credentials. The library does not read `.env` files,
persist tokens, or expose raw token accessors. Secret-bearing types have redacted
`Debug`, and token revisions prevent delayed login/refresh work from overwriting a
newer session. Empty or expired session responses are rejected, and renewal is
single-flight across client clones. Penalty retries enforce the provider delay on a
monotonic clock and are single-use bindings to the issuing client, endpoint, and exact
serialized request.

Remote endpoints require HTTPS/WSS. Plain transport is accepted only for exact
loopback fixture hosts. Ambient proxy discovery is disabled; proxying, if supported,
is an explicit builder choice.

### REST execution

HTTP response bodies are streamed under configurable limits. Queries wait
asynchronously for shared rate capacity but are currently single-attempt. Any future
retry policy must be limited to failures proven safe to retry, with fresh admission
for every attempt. Money-moving mutations do not retry automatically and do not wait
in a local throttle queue.

When a mutation may have reached the provider but no trustworthy completion is
available, the result is explicitly ambiguous and every clone of that client rejects
further mutations. Queries remain available for reconciling current orders, fills,
and positions. Only after authoritative reconciliation should a caller invoke
`Client::acknowledge_mutation_reconciliation` and permit another submission.

The current surface is generated into capability modules under
`api::current::{accounting, alerts, authentication, configuration, contracts, fees,
funds, orders, positions, risks, users}`. Financial `number` tokens decode directly
to `Decimal`; provider identities are validated newtypes; request secrets have
redacted `Debug` and no public getter; and response enums preserve unknown provider
values while refusing to serialize those unknown values back into a request.

Generated query methods do not own transport policy. GET and safe POST queries use
the bounded query path. A state-changing operation becomes callable only through a
reviewed handwritten path with single-attempt mutation policy and endpoint-specific
completion evidence; schema presence alone is insufficient. Request bodies,
response bodies, query cardinality, and query size are bounded before or during
transport. No public API accepts an arbitrary endpoint, command string, or raw JSON
response.

The exhaustive manifest records 16 `DocumentationBlocked` operations. Eleven have no
published response schema, one has an incomplete response schema, and other blocked
rows lack the request or completion evidence needed to call them without guessing.
None has a raw JSON escape hatch or invented success contract. See
[`docs/api-coverage-rest.md`](docs/api-coverage-rest.md) for the exact rows and
provider blockers.

### Rate admission

The pinned OpenAPI defines which current operations exist; the current Partner
[rate-limit table](https://partner.tradovate.com/overview/core-concepts/rate-limits),
verified 2026-08-22, is the operational authority for their quotas. In particular,
`submitpartnersubaccountdocument` uses the table's current 750/hour all-request
budget rather than the stale 10/hour prose embedded in that operation's pinned
description. Failed-response-only reservations cover `auth/me`, both evaluation
batch endpoints, and `requesttradingpermission` in addition to direct login.

Admission is shared by every clone and is atomic across user, endpoint, and demo
account scopes. A proven pre-send failure rolls those reservations back. Once send
may have started, cancellation or ambiguous transport conservatively retains every
all-request charge and records a failed-only charge, so local concurrency cannot
overrun the provider. Demo balance changes enforce both the current 1,000/hour
aggregate endpoint budget and the stricter one-change-per-account-per-hour guard;
the latter stays conservative because the client cannot infer the caller's
organization-admin exemption. Any HTTP or WebSocket 429 installs the documented
global one-hour minimum stop; endpoint penalty tickets instead use their exact
provider `p-time`.

### Realtime execution

The production realtime transport does not consider a connection ready
until authentication and protocol negotiation succeed. Frames have both byte and
message-count ceilings; queue and pending-reply capacities also obey aggregate byte
budgets. Each connection is exactly one immutable generation; the transport does not
automatically reconnect it.

The transport deliberately owns no canonical subscription or account projection.
An unexpected termination after readiness publishes
`ResyncRequired(ConnectionLost)`; event overflow publishes
`ResyncRequired(EventBufferOverflow)`. Abandoning an admitted request publishes
`ResyncRequired(RequestAbandoned)` because its outcome can no longer be observed.
Only caller-requested graceful shutdown is an ordinary `Closed` state. Callers then
create a new connection, replay their idempotent subscription set, and obtain a fresh
snapshot/reconciliation before accepting new deltas. The crate currently emits no
replay instruction and has no recovery-fence acknowledgement API.

Tradovate realtime sockets use SockJS-derived server frames and exact four-field
client requests. The client heartbeat is the text frame `[]` on an independent
2.5-second monotonic schedule. Every non-shutdown post-readiness writer operation is
fenced by the next heartbeat deadline; a backpressured writer invalidates the
generation instead of remaining ready after a missed heartbeat. A user-data socket
performs exactly one `user/syncrequest` per authenticated connection generation
before its snapshot-plus-delta stream is ready. Bounded messages received during authorization
and in the same frame as user sync are staged; the validated bootstrap is always
delivered before those deltas and before readiness is published. A validated penalty
installs the full monotonic cooldown and ends setup with a typed error; the foundation
does not retry user synchronization automatically.
The default user-sync profile explicitly requests all 31 entity families in the
pinned current `SyncMessage` schema with `splitResponses: false`. A validated
`UserSyncConfig` also supports the documented user/account filters, cutoff timestamp,
closed `modAccountId`/`modUserId` sharding grammar, entity-family selection, and
`fullOrgSnapshot` flag while rejecting forbidden field combinations locally. The
bootstrap must contain the pin-required `users` and `contractGroups` collections
before readiness. Only B2B multipart completion remains documentation blocked.
Public events contain no raw JSON values: known user entities and the documented
`OtherEnvAdminAlertSignal` signature event reuse current typed REST models; quotes,
DOM, histograms, bars, and ticks cross the boundary as exact `Decimal` values.
Malformed, partial, oversized, or semantically unknown payloads end the generation
rather than publishing partial truth. The complete public surface and its explicit
documentation boundaries are recorded in
[`docs/api-coverage-realtime.md`](docs/api-coverage-realtime.md).

## Official protocol references

- [Partner API documentation](https://partner.tradovate.com/)
- [Environment reference](https://partner.tradovate.com/resources/reference/environments)
- [Authentication](https://partner.tradovate.com/api/rest-api-endpoints/authentication/access-token-request)
- [WebSocket architecture](https://partner.tradovate.com/overview/core-concepts/architecture-overview)
- [User synchronization](https://partner.tradovate.com/overview/core-concepts/web-sockets/user-syncrequest)
- [Current rate limits](https://partner.tradovate.com/overview/core-concepts/rate-limits)
- [Error handling](https://partner.tradovate.com/overview/core-concepts/error-handling)

Provider documentation is mutable and not internally consistent in every schema.
Contract work records the source URL and access date and is backed by reviewed wire
fixtures. A live documentation export is never treated as an infallible generated
public API.

## Development

Read [`AGENTS.md`](AGENTS.md) before changing code. It contains the stable `TV-*`
rules used for implementation and review.

The normal local gate is:

```text
python3 tools/generate_openapi.py --check
cargo fmt --all -- --check
bash scripts/ci/check_file_sizes.sh
cargo check --no-default-features --locked
cargo check --all-targets --all-features --locked
cargo clippy --all-targets --all-features --locked -- -D warnings -D clippy::all -D clippy::pedantic -D clippy::await_holding_lock -D clippy::expect_used -D clippy::unwrap_used -F unsafe-code
RUSTDOCFLAGS="-D warnings" cargo doc --all-features --no-deps --locked
cargo nextest run --all-features --locked --no-fail-fast
cargo test --doc --all-features --locked
cargo deny --locked check
cargo tree --locked --all-features --edges normal,build,dev --prefix none | grep --extended-regexp '^rkyv v0\.7\.' && exit 1 || true
cargo audit --file Cargo.lock --deny warnings --ignore RUSTSEC-2026-0235
gitleaks git --no-banner --redact --log-opts="--all" .
```

Regenerate the checked-in current surface with
`python3 tools/generate_openapi.py`. The generator refuses a snapshot whose SHA-256
does not match the reviewed constant, and `--check` fails when generated output is
stale. Never hand-edit files under `src/api/current/generated/`.

The secret scan requires Git history and is authoritative in CI.

See [`CONTRIBUTING.md`](CONTRIBUTING.md) for test, file-size, security, and pull
request requirements.

## Live validation

Normal tests use synthetic loopback fixtures and require no credentials. Any live
probe must be compiled behind an explicit feature, marked ignored with a reason, use
only read-only operations, and be run deliberately with process-scoped secrets. Do
not store credentials in `.env`, shell startup files, command arguments, transcripts,
HAR files, or packet captures.

## Versioning and distribution

Internal releases follow Semantic Versioning. Source-breaking API changes require a
new major version; additive APIs and compatible fixes use minor and patch versions.
Provider contract changes may still require operational changes, so review the
private changelog before updating.

This package is private and must not be published to crates.io, docs.rs, or another
public registry. Approved consumers use the private repository or an approved
private package channel.

## License

Proprietary and confidential. Copyright (C) 2026 Kevin Monaghan. All rights reserved.
See [`LICENSE`](LICENSE).
