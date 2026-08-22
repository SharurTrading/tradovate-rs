<!--
SPDX-FileCopyrightText: 2026 Kevin Monaghan
SPDX-License-Identifier: MIT-0
-->

# Tradovate realtime API coverage

Status: production-public typed coverage with explicit documentation boundaries.

- Contract reviewed: 2026-08-21 (Australia/Sydney)
- User-sync operational guide rechecked: 2026-08-22 (Australia/Sydney)
- Authority: current `partner.tradovate.com` documentation indexed by
  `https://partner.tradovate.com/llms.txt` on the review date
- REST authority: the hash-pinned current Partner OpenAPI snapshot recorded in
  [`../spec/official/openapi-2026-08-21.json`](../spec/official/openapi-2026-08-21.json)

## Executive result

The crate exposes a production-public, bounded, generation-fenced realtime API.
Downstream callers can establish user, market-data, and replay socket generations;
observe typed lifecycle state; receive typed user bootstrap and `props` events;
subscribe to quotes, depth, and histograms; request and cancel charts; and consume
exact regular-bar and compact-tick payloads.

Raw JSON is private implementation detail. Public events contain typed current REST
entities, exact `rust_decimal::Decimal` financial values, validated provider IDs, or
bounded structural metadata. A malformed, oversized, unsupported, or abandoned
event/request cannot silently become caller truth: it fails the generation or publishes
an explicit resynchronization requirement.

Two current documentation boundaries remain:

1. B2B `splitResponses: true` user synchronization names multipart delivery but does
   not publish a completion marker or complete assembly contract.
2. Replay documentation names a `clock` family and replay routing, but does not publish
   the `clock` payload or replay startup/control request contracts.

These gaps are represented by
`realtime::DOCUMENTATION_BLOCKED_CAPABILITIES`. The crate does not guess the missing
wire shapes or expose their raw payloads.

## Public coverage matrix

| Current Partner capability | State | Public contract |
| --- | --- | --- |
| User, market-data, and replay socket endpoints | Implemented | `SocketKind` plus `Client::connect_realtime` select one validated service endpoint and create one immutable `ConnectionId` generation. |
| SockJS-derived `o`, `a[...]`, `h`, and logical-close framing | Implemented | Bounded private codec; malformed, binary, oversized, or invalid-state frames fail closed. |
| Authorization | Implemented | Access-token authorization for user/replay and market-data token with access-token fallback for MD; token freshness is rechecked before send and errors are secret-safe. |
| Client heartbeat and inbound liveness | Implemented | Independent monotonic `[]` heartbeat every 2.5 seconds, ping/pong support, bounded liveness timeout, and generation failure when writer backpressure would miss the heartbeat. |
| Correlated responses | Implemented | Bounded request IDs and pending map; late responses become typed `UnmatchedResponse` metadata with response bodies discarded. |
| Generation lifecycle and recovery | Implemented | `Connecting`, `Ready`, `Closed`, and `ResyncRequired` state with explicit overflow, loss, abandonment, heartbeat, and unsupported-event reasons. |
| User bootstrap | Implemented | `UserSyncConfig` always uses `splitResponses: false`, explicitly requests all 31 pinned current entity families by default, supports the documented safe filter/sharding fields, and requires `users` plus `contractGroups` before co-batched deltas and readiness. |
| User `props` deltas | Implemented | Typed `Created`, `Updated`, and `Deleted` events; all 31 current sync entity families plus the documented `OtherEnvAdminAlertSignal` wrapper decode to typed current REST entities. |
| B2B split-response bootstrap | Documentation blocked | No current completion marker or multipart assembly contract is published; `B2bSplitUserSync` records the withheld capability. |
| Graceful `shutdown` | Implemented | Typed maintenance, connection-quota, IP-quota, and bounded forward-compatible provider reasons. |
| Quote subscribe/unsubscribe | Implemented | `Symbol` or `ContractId` target, typed quote entries, exact optional price/size, validated timestamps, and bounded entry counts. |
| DOM subscribe/unsubscribe | Implemented | `Symbol` or `ContractId` target, exact depth levels, bounded sides, descending bids, and ascending offers. |
| Histogram subscribe/unsubscribe | Implemented | `Symbol` or `ContractId` target, typed trade date, exact base/buckets, signed offsets, refresh flag, and bounded map. |
| `md/getChart` | Implemented | Validated builder for every current underlying type, element unit, symbol/contract target, and documented time-range selector; `closestTickId` uses the validated reusable `TickId` newtype while retaining exact integer wire encoding. |
| `md/cancelChart` | Implemented | Cancellation uses the distinct validated realtime chart ID returned with the subscription. |
| Regular chart bars | Implemented | Typed OHLC, volumes, tick counts, bid/offer volumes, timestamps, trade date, and end-of-history packets using exact decimals. |
| Compact tick charts | Implemented | Checked timestamp/price reconstruction, exact trade/bid/ask values, optional sizes, bounded packets, provider order, tick IDs, and end-of-history. |
| Replay common socket lifecycle | Implemented | Current replay endpoint selection, authorization, framing, generation, typed common events, and recovery semantics. |
| Replay `clock` payload | Documentation blocked | The raw body is discarded and a `ReplayClockPayload` metadata event forces resynchronization. |
| Replay startup/control | Documentation blocked | No public control method is exposed; `ReplayControl` records the missing current request contract. |
| Unknown current extensions | Implemented fail-closed boundary | Bounded `ProviderCode`/item-count metadata is observable, raw data is discarded, and the generation requires resynchronization. |

The complete typed REST operation surface remains available through the REST client.
The generic WebSocket request primitive is intentionally private: public realtime
methods are reviewed typed capabilities, not arbitrary endpoint strings or raw JSON.

## Connection and ownership model

`RealtimeConnection` is intentionally not cloneable. One actor owns the socket writer,
request correlation, event sender, and lifecycle state; one handle owns the event
receiver and task join handle. This keeps each mutable transport concern single-writer
and makes teardown observable.

Each successful attempt receives a process-local `ConnectionId`. Events carry that ID,
and the handle never replaces its socket with a new generation. It does not retain a
canonical desired-subscription set, account projection, order mirror, or replay
session. After an unexpected gap, callers:

1. stop accepting events from the failed generation;
2. reconcile or obtain a fresh authoritative snapshot;
3. create a new authorized generation;
4. replay their own idempotent desired subscriptions; and
5. accept deltas only after their application-level recovery fence is complete.

Only caller-requested shutdown is an ordinary `Closed` state. Event overflow,
connection loss, an abandoned admitted request, a missed heartbeat deadline, or an
event whose semantics cannot be preserved becomes `ResyncRequired`.

## User stream contract

Every authorized user socket sends exactly one `user/syncrequest` before readiness.
`UserSyncConfig::default()` sends the mandatory `splitResponses: false` and an
explicit `entityTypes` list containing every collection in the pinned current
`SyncMessage` schema: user/user-property/property; account/risk/margin/auto-liq;
cash/currency/position; fill-pair/order/contract/maturity/product/exchange/spread;
command/report/execution/order-version/fill/fill-fee; order strategy/link/type;
user plugin/annual review/read status/promo code; and contract group.

The public configuration path also covers the OAS `cutoffTimestamp` and
`fullOrgSnapshot` fields and the current operational guide's filters and sharding.
User and account filters use validated `UserId`/`AccountId` values. Sharding uses the
closed documented `modAccountId | modUserId` grammar with a positive divisor and a
remainder in `0..divisor`. The provider's cross-field rules are enforced locally:
`users` cannot be combined with sharding or `entityTypes`, and `accounts` cannot be
combined with sharding. Entity lists are non-empty, explicit, unique, and bounded;
omission is never mistaken for the provider's now-empty default subscription.

These variants reuse the generated current REST models; realtime does not own duplicate
entity definitions. The pin requires both `users` and `contractGroups` in every
`SyncMessage`; absence or malformed values fail setup before readiness. The validated
bootstrap is emitted first, followed by any bounded `props` messages received during
authorization/synchronization, and only then is the generation published as ready.

`props.eventType` is typed as `Created`, `Updated`, `Deleted`, or bounded unknown
metadata. Both one and many entity forms are accepted. An unknown entity family is
represented only by its validated name and item count; its raw objects are discarded
and recovery is required, preventing partial provider truth from entering a projection.
The current user-management guide's `OtherEnvAdminAlertSignal` signature notification
is decoded from its documented wrapper into the pinned typed `AdminAlertSignal`; these
system/signature events remain safe even though the provider states they can bypass
`entityTypes` filtering on sharded sockets.

B2B multipart synchronization remains withheld as one contract unit. The crate will
not infer completion from a quiet period, publish a partial snapshot as complete, or
offer split synchronization until current provider evidence defines its completion
and ordering semantics. Configurable unsplit filters and sharding remain available.

## Market-data contract

`subscribe_market_data` and `unsubscribe_market_data` accept a closed
`MarketDataChannel` and borrowed `MarketDataTarget`. Targets are validated `Symbol` or
`ContractId` values. The transport retains only the in-flight invocation; callers own
the desired set and replay it after recovery.

Quote events carry a bounded provider timestamp, contract ID, and typed entries for
bid, offer, trade, total volume, open interest, opening price, high, low, settlement,
or bounded unknown response names. Price and size are independently optional and use
`Decimal` when present.

Depth events carry full bounded bid and offer sides. The decoder validates bids in
descending price order and offers in ascending order. It never silently reorders an
invalid provider packet.

Histogram events carry contract ID, timestamp, validated trade date, exact base,
signed provider bucket offsets, exact bucket values, and the refresh flag. Neither keys
nor values pass through floating point.

## Chart contract

`ChartRequest` uses a validating builder. It accepts a symbol or contract ID and the
current aggregation families `Tick`, `DailyBar`, `MinuteBar`, `Custom`, and DOM. The
current element units are volume, range, underlying units, Renko, momentum range,
point-and-figure, and OFA range. Validation requires a positive element size, locks
tick charts to size one, and requires at least one valid history boundary.

One successful request returns distinct `HistoricalChartId` and `RealtimeChartId`
newtypes. Cancellation accepts only the realtime ID, preventing the two provider
identities from being accidentally interchanged.

Regular packets expose exact OHLC, up/down volume, up/down tick values, bid/offer
volume, timestamp, and trade date. Compact packets reconstruct timestamps and
trade/bid/ask prices with checked integer and decimal arithmetic. Invalid tick size,
overflow, malformed timestamps, and unrepresentable values fail the event contract.
Provider arrival order is preserved because current documentation permits tick packets
to arrive out of chronological order.

## Replay documentation boundary

The current Partner material publishes replay endpoints and states that replay market
data and simulated user traffic use the replay socket. It also names a `clock` server
event. It does not publish the control requests needed to create/join a replay session,
the session state machine, or the `clock` data schema.

Accordingly, the crate exposes the common replay socket generation lifecycle but not a
guessed replay session API. A received `clock` family becomes
`DocumentationBlocked(ReplayClockPayload)` without its raw body. Replay control and a
typed clock event remain unavailable until the current contract or a reviewed
synthetic staging fixture supplies the missing fields and completion semantics.

## Bounds and failure behavior

`RealtimeConfig` defines positive hard limits for frame bytes, messages per frame,
pending requests, command capacity, event capacity, request timeout, and inbound
liveness. Configuration also validates worst-case aggregate byte budgets for command,
event, and pending-response queues.

The private codec validates endpoint/query newline rules, exact four-field requests,
frame and message counts, correlated response structure, and text-only application
frames. Public errors contain stable typed categories and no access tokens, request
bodies, penalty tickets, or raw remote payloads.

All financial values decode directly to `Decimal`. Timestamps, trade dates, IDs,
provider codes, collection counts, and depth ordering are validated before a public
event is constructed. Unsupported typed meaning never degrades to a public
`serde_json::Value`.

## Deterministic validation

Credential-free tests cover:

- open, authorization, user bootstrap, staged-delta, ready, shutdown, and recovery
  ordering;
- token rejection, expiry/revision fences, penalties, `429`, timeouts, cancellation,
  late responses, queue overflow, liveness, and heartbeat backpressure;
- SockJS-derived frames, correlated/batched messages, malformed input, byte/count
  ceilings, and unknown events;
- all 31 pinned bootstrap collections, required-field rejection, one/many typed
  `props` forms, custom filters/sharding, and `OtherEnvAdminAlertSignal` decoding;
- quote, DOM, and histogram decoding, exact decimals, depth ordering, invalid dates,
  and count limits;
- chart request cross-field validation, historical/realtime ID separation, exact bars,
  compact-tick reconstruction, overflow, optional bid/ask values, provider ordering,
  cancellation, and end-of-history; and
- public-surface checks that raw payload types are not exported.

Provider staging probes, if added, remain deliberate, ignored, credential-safe, and
read-only where possible. No live probe is required for normal tests.

## Completion rule for blocked capabilities

A documentation-blocked capability may become public only when all of the following
land in one reviewed change:

1. a current provider contract or sanitized synthetic staging fixture supplies the
   missing payload/completion semantics;
2. bounded private wire decoding and validated public types are added without a raw
   escape hatch;
3. cancellation, ordering, timeout, overflow, and recovery behavior is explicit;
4. deterministic fixtures cover success, rejection, malformed data, limits, and
   forward-compatible response behavior; and
5. rustdoc, README, ADR, changelog, and this matrix are updated together.

## Official sources

All sources were accessed 2026-08-21; the mutable user-sync operational guide was
rechecked 2026-08-22.

- Partner documentation index: <https://partner.tradovate.com/llms.txt>
- Architecture and WebSocket frame/event contract:
  <https://partner.tradovate.com/overview/core-concepts/architecture-overview>
- Authentication overview:
  <https://partner.tradovate.com/overview/quick-setup/auth-overview>
- Connection, authorization, request, heartbeat, and recovery guide:
  <https://partner.tradovate.com/overview/core-concepts/web-sockets/connection-overview>
- User synchronization:
  <https://partner.tradovate.com/overview/core-concepts/web-sockets/user-syncrequest>
- Cross-environment administrator-alert signature event:
  <https://partner.tradovate.com/overview/prop-firm-management/create-and-manage-users>
- Market-data overview:
  <https://partner.tradovate.com/overview/core-concepts/web-sockets/market-data/market-data>
- Market-data request and response reference:
  <https://partner.tradovate.com/overview/core-concepts/web-sockets/market-data/market-data-request-reference>
- Tick charts and compact packet conversion:
  <https://partner.tradovate.com/overview/core-concepts/web-sockets/market-data/tick-charts>
- WebSocket conformance Stage 2:
  <https://partner.tradovate.com/overview/conformance-testing/stage-2-websocket-management>
- Market-data conformance Stage 5:
  <https://partner.tradovate.com/overview/conformance-testing/stage-5-market-data-access>
- Rate limits and penalty controls:
  <https://partner.tradovate.com/overview/core-concepts/rate-limits>
- WebSocket error response guidance:
  <https://partner.tradovate.com/overview/core-concepts/error-handling>
- Best practices:
  <https://partner.tradovate.com/resources/reference/best-practices>
- Official service endpoints:
  <https://partner.tradovate.com/resources/reference/environments>
