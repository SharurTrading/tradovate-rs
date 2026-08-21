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

Use the official [Tradovate API documentation](https://api.tradovate.com/) as the
authority for endpoints, request fields, authentication, rate limits, realtime
messages, and account permissions. Repository documentation records the additional
safety and lifecycle guarantees supplied by this client.

## Status

The initial vertical slice is implemented, but this is not a complete binding for
Tradovate's API.

| Capability | Current coverage |
| --- | --- |
| Authentication | Direct API-key login, expiry/revision-fenced renewal, redacted session metadata, and client-bound delayed penalty retry |
| REST queries | Account list, contract find, position list, and order list |
| REST commands | Validated order placement/cancellation, no automatic retry, and a shared reconciliation latch after ambiguity |
| Realtime transport | Test-only architecture harness for bounded framing, authorization, request correlation, heartbeat, lifecycle state, and user bootstrap; excluded from the production facade |
| Market data | Test-only typed command fixtures; production subscription and event APIs are deferred |

Remaining REST endpoints, the public typed realtime API, chart and replay commands,
configurable/split user synchronization, automatic reconnect, subscription replay,
and live probes are deliberately deferred. Feature coverage must change alongside
each implementation slice.

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
- Deterministic synthetic tests; any future live probe must be deliberate, ignored,
  and read-only.

## Quick start

The caller owns credentials, their storage, and the Tokio runtime. The crate does
not read `.env` files or discover secrets from ambient process state.

```rust
use tradovate_client::{Client, DeviceId, Environment, auth::Credentials};

# async fn connect() -> Result<(), Box<dyn std::error::Error>> {
let credentials = Credentials::builder("user", "dedicated-api-password")
    .app_id("my-app")
    .app_version("1.0")
    .numeric_client_id(123)
    .secret("api-key-secret")
    .device_id(DeviceId::new("stable-device-id")?)
    .build()?;

let client = Client::builder(Environment::Demo).build()?;
let session = client.authenticate(&credentials).await?;
println!("authenticated user {}", session.user_id());
# Ok(())
# }
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
  api/                   account, contract, order, and position capabilities
  realtime/              connection, codec, lifecycle, events, flow control
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

### Realtime execution

The test-only realtime architecture harness does not consider a connection ready
until authentication and protocol negotiation succeed. Frames have both byte and
message-count ceilings; queue and pending-reply capacities also obey aggregate byte
budgets. Each connection is exactly one immutable generation; the harness does not
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
The initial user-sync profile uses a fixed entity list and `splitResponses: false`;
configurable entity selection, sharding, and multipart bootstrap completion are not
yet implemented. The harness is compiled only for crate unit tests, raw envelopes are
never part of the production crate, and public realtime access is withheld until each
exposed capability has bounded typed event models.

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
cargo fmt --all -- --check
bash scripts/ci/check_file_sizes.sh
cargo check --no-default-features --locked
cargo clippy --all-targets --all-features --locked -- -D warnings -D clippy::pedantic -D clippy::await_holding_lock -D clippy::expect_used -D clippy::unwrap_used
RUSTDOCFLAGS="-D warnings" cargo doc --all-features --no-deps --locked
cargo nextest run --all-features --locked --no-fail-fast
cargo test --doc --all-features --locked
cargo deny --locked check
cargo audit --file Cargo.lock --deny warnings
gitleaks git --no-banner --redact --log-opts="--all" .
```

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
