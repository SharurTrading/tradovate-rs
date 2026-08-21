<!--
Copyright (C) 2026 Kevin Monaghan. All rights reserved.

This file is proprietary and confidential.
Unauthorized copying, use, modification, distribution, or disclosure of this file,
via any medium, is strictly prohibited except under a written agreement with the
copyright owner.
-->

# ADR 0001: Provider-native single-crate architecture

- Status: Accepted
- Date: 2026-08-21
- Decision owners: SharurTrading maintainers

## Context

`tradovate-rs` must provide a production-quality Rust interface to Tradovate's REST
and realtime APIs without becoming part of a consuming trading platform. It will be
used by systems with stronger domain, routing, risk, and portfolio contracts of their
own. Allowing those contracts into this crate would couple releases, duplicate truth,
and make the provider client unusable independently.

API clients also tend to accumulate a giant client file, one global models bucket,
unbounded WebSocket state, and a generic retry layer. Those shapes are especially
dangerous for trading: a timeout after order submission may be ambiguous, stale token
refresh can replace a newer session, and dropped realtime events can silently corrupt
a downstream projection.

The design must therefore make exact values, secrets, cancellation, admission,
connection generations, and recovery explicit while remaining small enough to audit.

## Decision

### One focused crate

Start with one private library crate whose intended package name is
`tradovate-client`. Do not introduce a workspace, protocol crate, model crate, or
provider-generic abstraction until a real dependency, trust, ownership, or runtime
boundary exists.

The crate owns:

- caller-configured endpoints and transport construction;
- credential wrappers and revision-fenced token/session lifecycle;
- shared provider rate admission and cooldowns;
- exact provider wire models and request validation;
- REST encoding, bounded decoding, and typed provider failures;
- realtime connection, protocol codec, flow control, and lifecycle events.

Consumers own:

- route and account selection policy;
- canonical instruments and cross-provider domain translation;
- portfolios, order mirrors, positions, P&L, risk, and storage;
- canonical realtime subscriptions and snapshot-before-delta recovery;
- GUI, strategy, and business logic.

### Capability-oriented modules

Use this ownership layout:

```text
src/
  lib.rs
  error.rs
  environment.rs
  ids.rs
  decimal.rs
  auth/{mod,credentials,token,wire}.rs
  client/{mod,builder,execute}.rs
  rate_limit/{mod,tests}.rs
  api/{mod,accounts,contracts,orders,positions}.rs
  api/current/{mod,support}.rs
  api/current/mutations/{customer,entity_write,risk_control,...}.rs
  api/current/generated/{accounting,alerts,authentication,configuration,
    contracts,fees,funds,ids,manifest,orders,positions,risks,users}.rs
  realtime/{mod,actor,codec,config,connection,error,event,types}.rs
  realtime/{user_stream,market_data,chart}.rs
```

The exact filenames may evolve, but ownership may not collapse into catch-all
`models`, `helpers`, `utils`, or `manager` modules. `lib.rs` remains documentation,
module declarations, selective re-exports, and narrow wiring.

### Current REST contract generation

The sole REST generation input is
`spec/official/openapi-2026-08-21.json`, retrieved from the current Partner OpenAPI
on 2026-08-21. Its reviewed SHA-256 is
`37caeccf4b0913460a788fcaf4c902497059b8ffe6f6355512e6c08eaacde769`.

`tools/generate_openapi.py` validates that digest and deterministically emits the
checked-in `api/current/generated` boundary. The generated boundary owns:

- all 278 current component schemas, including exact-decimal field adapters,
  validated identity newtypes, private fields, builders, and accessors;
- generated query methods grouped by the eleven current capability tags, while
  lifecycle-sensitive and state-changing methods require a separately reviewed
  handwritten implementation;
- an exhaustive manifest of 350 unique method/path pairs, classified as 271 queries,
  73 mutations, and 6 lifecycle operations;
- surface status for every row: 263 generated, 71 specialized, 0 modeled-only, and
  16 documentation blocked;
- response status for every row: 338 typed, 11 unspecified by the pin, and 1
  incomplete in the pin; and
- forward-compatible response enums whose unknown values remain observable but
  cannot be serialized into requests.

Generated query methods may call only handwritten query execution hooks. They do not
construct a second HTTP client, acquire tokens directly, retry requests, or decode
unbounded bodies. A mutation schema produces models and a manifest entry, not
transmission authority: every callable mutation requires reviewed validation and
endpoint-specific completion evidence on the single-attempt mutation path.
Authentication and per-connection user synchronization similarly stay in their
handwritten session or realtime lifecycle owner.

The resulting public surface has 334 callable typed operations: 333 direct REST
methods plus `/user/syncrequest`, which is owned by the realtime lifecycle. The remaining
16 rows are not left as implicit TODOs: each is classified
`DocumentationBlocked` with the exact missing request, response, or completion
evidence. No blocked row gains invented fields, invented success semantics, or a raw
response escape hatch.

Generated Rust is never hand-edited. `tools/generate_openapi.py --check` is a CI
contract, and changing the pinned input or digest requires a semantic, safety,
query/mutation/lifecycle classification, and schema-gap review.

### Public API

- Expose provider-native types only.
- Use validated, private-field newtypes for provider identities.
- Use builders for cross-field request/configuration invariants.
- Keep response types forward-compatible with `#[non_exhaustive]` and observable
  unknown provider enum codes.
- Keep raw JSON and raw command/endpoint escape hatches private by default.
- Borrow inputs unless the callee stores, queues, or transfers them.
- Use focused `thiserror` error enums with redacted public context.

### Exact values

Financial JSON tokens are decoded directly to `rust_decimal::Decimal`; they never
pass through `f32` or `f64`. Provider-defined IDs, counts, and integral quantities
use their exact documented integer widths. Timestamps and dates are parsed types,
not unchecked strings.

### Authentication state

One private session store is the token writer. Requests use owned snapshots tagged
with a revision. Login, refresh, expiry, replacement, and invalidation may commit
only when their basis revision remains current. Refresh is single-flight across
client clones, and both refresh and WebSocket authorization recheck freshness at the
last pre-send boundary. RAII attempt guards distinguish pre-send cancellation from an
ambiguous transmission-started refresh. Penalty retries use a monotonic not-before
deadline and bind one claim to the issuing client, endpoint, and exact request bytes.
No synchronization guard is held across network I/O.

### REST execution policy

Separate query and mutation executors:

- queries asynchronously wait for shared capacity and are currently single-attempt;
  a future retry policy may retry only failures proven safe, with bounded delay and
  fresh admission;
- mutations require immediate capacity, transmit at most once, and return an
  explicit ambiguous result after possible admission without trustworthy completion;
  ambiguity installs a shared reconciliation-required latch across client clones,
  while queries remain available to rebuild authoritative state.

Every attempt receives local admission. HTTP redirects and dependency-level retries
are disabled so no request can bypass this policy. Response bodies are streamed under
a configurable hard byte limit. HTTP-200 business failures and penalty-ticket bodies
are decoded before success; captcha penalties require operator action.

The pinned OpenAPI is the operation inventory, while the current Partner rate-limit
table is the operational quota authority. All-request and failed-response-only
endpoint windows are reserved atomically with the user window. Proven pre-send
failures release every reservation; cancellation after the send boundary retains
all-request capacity and counts conservatively as failure. Demo balance admission
combines the 1,000/hour endpoint window with the one-change-per-account-per-hour
guard because caller administrator status is not trustworthy local state. A 429 is
always a global one-hour minimum stop; only a validated penalty ticket uses the
endpoint-specific `p-time`.

### Realtime lifecycle

Each connection attempt receives a monotonically increasing generation.
Authentication and protocol negotiation complete before readiness is published, and
every event carries that immutable generation identifier. The current connection
handle owns one generation only; it does not reconnect or replace itself.

Frames, messages, writer/event queues, pending requests, decoded memory, and waits
have named bounds. Queue capacities are cross-validated against the frame limit to
enforce aggregate byte budgets, and client requests are preflighted before allocation
or enqueue. Overflow or ambiguous request completion ends the generation.
Event-buffer overflow terminates with `ResyncRequired(EventBufferOverflow)`; every
unexpected post-readiness termination publishes `ResyncRequired(ConnectionLost)`.
Dropping an admitted request poisons the generation with
`ResyncRequired(RequestAbandoned)`, and a response observed at or after its stored
deadline cannot win a timeout race. Only caller-requested shutdown publishes an
ordinary `Closed` state.

The transport never owns canonical subscriptions or account/order/position truth.
The caller creates a new connection after failure, replays its idempotent set, and
performs snapshot-before-delta recovery. Recovery acknowledgement remains
caller-owned.

Tradovate's SockJS-derived connection uses one exact four-field request per WebSocket
frame, bounded request correlation, and the required `[]` client heartbeat on an
independent monotonic 2.5-second schedule. Every non-shutdown post-readiness use of the
sole writer is bounded by the next heartbeat deadline, so writer backpressure ends the
generation instead of starving the heartbeat. Each authenticated user-socket
generation sends exactly one `user/syncrequest`; readiness waits for its bootstrap
contract. A validated penalty installs its full monotonic cooldown and ends setup
without an automatic retry.
Authorization-era messages and messages co-batched with the sync response are staged
under the event budget, then published strictly after bootstrap and before readiness.
The realtime module is production compiled and selectively public. Its validated,
single-response user bootstrap requests all 31 current `SyncMessage` entity families
by default and requires the schema-mandatory `users` and `contractGroups` collections
before readiness. `UserSyncConfig` models current filters, cutoff, entity selection,
the closed socket-sharding grammar, and `fullOrgSnapshot`, and rejects forbidden
filter/sharding combinations before spawning the actor. Typed bootstrap and `props`
deltas, including `OtherEnvAdminAlertSignal`, reuse current REST entity models.
Quote, depth, histogram, regular-bar, and compact-tick events use exact
`Decimal` values; chart requests validate their cross-field invariants and retain
distinct historical and realtime IDs. Raw JSON exists only inside bounded private
decoders and never crosses the public event boundary.

The current Partner documentation does not define a completion marker for B2B
`splitResponses: true`, nor the replay `clock` payload or replay control protocol.
Those contracts are represented by explicit `DocumentationBlockedCapability` values
and remain unavailable until a current provider contract or synthetic staging fixture
supplies the missing evidence.

### Validation and maintainability

Normal tests are deterministic and credential-free. Public integration fixtures
assert exact wire requests and responses. Private state machines receive unit,
cancellation, race, timeout, overflow, and property/fixture tests as appropriate.
Live probes are currently deferred; any future probe must be ignored, deliberate,
and read-only.

Handwritten source files target 400 physical lines and fail above 600. `lib.rs` and
`mod.rs` fail above 200, tests above 800, and only documented generated code is
exempt. Production functions over 100 lines require explicit manual review.

## Consequences

Positive consequences:

- the crate remains reusable independently of SHARUR;
- money-moving ambiguity is visible rather than hidden by generic retry behavior;
- exact values and typed identities prevent silent conversion and routing mistakes;
- token and connection races have explicit commit fences;
- bounded transport makes overload and data loss observable;
- capability modules and file limits keep code reviewable as coverage grows.
- the operation manifest makes current REST coverage auditable without exposing a
  raw endpoint API;
- hash-fenced deterministic generation makes provider drift explicit in review.

Costs and tradeoffs:

- more small types and explicit state transitions than a thin generated binding;
- callers must own subscription replay and reconciliation;
- adding an endpoint requires request validation, error classification, tests, and
  documentation rather than only a convenience method;
- generated code is large even though it is capability-split and mechanically
  reproducible;
- 16 REST operations remain intentionally documentation blocked until Tradovate
  publishes the request, response, or completion evidence identified in the manifest;
- B2B multipart user synchronization and replay clock/control remain intentionally
  unavailable until their current wire contracts are published or confirmed.

## Rejected alternatives

### Depend on SHARUR domain types

Rejected because it reverses the adapter boundary, couples release cycles, and turns
a provider library into an application-specific adapter.

### Begin as a multi-crate workspace

Rejected as premature abstraction. One crate with private modules provides the needed
boundaries without extra public APIs or dependency graph complexity.

### Put every endpoint and model in two large files

Rejected because capability ownership disappears and security/lifecycle review
becomes impractical. The ProjectX reference demonstrated valuable semantics but also
showed how quickly `client`, `models`, and realtime files can become monolithic.

### Generic retry middleware

Rejected because it cannot safely infer whether a money-moving request was admitted
or whether an endpoint-specific rejection is definitive.

### Realtime client owns subscriptions and projections

Rejected because reconnect, overflow, and process recovery would turn transport into
an alternate source of trading truth.

### Make raw JSON the primary public API

Rejected because it bypasses validation, weakens compatibility, and makes malformed
or future provider behavior difficult to classify safely.

## Follow-up decisions

Later ADRs must record material choices such as the exact authentication refresh
state machine, official provider rate-budget mapping, realtime wire protocol and
limits, and any change to the schema-generation process. Those decisions must
preserve this boundary and the stable rules in `AGENTS.md`.
