<!--
SPDX-FileCopyrightText: 2026 Kevin Monaghan
SPDX-License-Identifier: MIT-0
-->

# ADR 0002: Preserve socket lifetime during local continuity recovery

Status: proposed for 0.2.0; supersedes the recovery details in ADR 0001.
Date: 2026-09-09.

## Provider evidence and scope

The audit was prompted by [ProjectX PR 28](https://github.com/SharurTrading/projectx-rs/pull/28).
Its failure categories informed regressions; no ProjectX protocol or proprietary
consumer implementation is used here.

Official Tradovate sources reviewed on 2026-09-09:

- [Connection overview](https://partner.tradovate.com/overview/core-concepts/web-sockets/connection-overview):
  SockJS-derived text framing, a 2.5-second `[]` heartbeat, and a server idle limit
  based on outbound traffic. Shutdown notices precede deliberate provider closure.
  Its illustrative inactivity watchdog is not evidence that silent application data
  means a dead WebSocket; this client uses a matching active ping/pong probe instead.
- [User synchronization](https://partner.tradovate.com/overview/core-concepts/web-sockets/user-syncrequest):
  user/account filtering and bootstrap semantics remain provider-specific. Exactly
  one synchronization request per authenticated generation is retained. Failed
  bootstrap/penalty setup still terminates without automatic retry.
- [Market-data request reference](https://partner.tradovate.com/overview/core-concepts/web-sockets/market-data/market-data-request-reference):
  quote/DOM/histogram subscriptions use explicit symbols or contract IDs; chart
  cancellation requires the allocated realtime ID. Its contradictory historical-ID
  parameter comment remains documented in the chart module.
- [Rate limits](https://partner.tradovate.com/overview/core-concepts/rate-limits):
  authenticated budgets and full penalty/429 cooldowns remain shared and unchanged.

The public connection already represents exactly one socket generation. This change
preserves that ownership model. There is no Connect-intent supervisor, automatic
reconnect loop, or backoff timer to migrate from ProjectX. Callers own replacements
and their desired subscription set. Cancelling a replacement's establishment future
cancels its handshake; it cannot authorize or publish readiness later. Shutdown and
owner drop never create another connection.

## Termination-site audit

| Site | Previous trigger | Result after this change |
| --- | --- | --- |
| `connection` request wait guard | Abandonment cancelled the generation | Atomically prevent queued transmission or retain an unknown admitted outcome; wake pending cleanup only. |
| `actor/pending` | Completion deadline ended the generation | Expire only the wait with `RequestOutcomeUncertain`; do not resend or reuse its ID. |
| Active record decoding | One malformed envelope/payload propagated a terminal error | Retain a continuity gap and continue independently framed records, replies, and keepalives. |
| Event publication | Full queue/unsupported event ended the generation | Preserve accepted prefix; retain one gap independently of data capacity; fence subsequent data. |
| Active response controls | HTTP-200 business refusal or penalty terminated the socket | Preserve the original typed request error and provider cooldown; keep the socket. |
| Inactivity watchdog | Ordinary inbound silence caused teardown | Transmit an active ping after idle time; fail only an unanswered matching-pong deadline. |
| Active writer | The next heartbeat or caller deadline cancelled a write | Separate socket-write deadline; expired caller wait does not interrupt an admitted transmission. |
| Remote physical/logical close or I/O failure | Generation termination | Retained, with original terminal error after both socket halves are dropped. |
| Setup/open/auth/sync validation | Failed establishment cleanup | Retained, including auth revision checks, bounded waits, one-sync and penalty rules. |
| `shutdown` / final-owner `Drop` | Cancel; take/drop or abort a join handle | Cancel and retain TaskTracker accounting; shutdown/session wait can prove actual termination. |

The only production socket owner is the actor. Both split halves remain owned by its
future, with no detached reader/writer children. On active exit it drains pending
requests as uncertain, closes admission, cancels its generation, drops both halves,
and only then publishes `GenerationEnded` and the latest terminal state. TaskTracker
accounts for actual future destruction, including cancellation and runtime shutdown.
An outer task wrapper explicitly drops the producer future before retaining failure
evidence on panic/abort/runtime shutdown, so a stopped task cannot strand event reception.
No destructor requires an ambient runtime or spawns a hidden runtime. The original
caller runtime must continue running for asynchronous cleanup to finish.

An in-progress WebSocket frame cannot safely be abandoned just to send `[]`.
The 2.5-second tick is a scheduling requirement, not independent proof that a
socket has failed. An active write therefore retains its separate bounded transport
deadline (ten seconds by default); after it completes, overdue heartbeats precede
ordinary work. Restoring the old heartbeat-deadline disconnect would reintroduce
the local-pressure teardown this migration removes. Failed establishment retains
its original stricter setup sequencing.

## Continuity and admission

`RealtimeSession` captures one immutable generation before dispatch. It shares only
that generation's command sender, admission gate, cancellation signal, and task
accounting. It has no owner, recovery, or reconnect authority. Closing admission and
enqueueing are serialized by one short mutex section without I/O or await. The actor
also checks generation and cancellation before transmission. Chart IDs carry the
allocating generation to prevent provider-ID reuse across sockets from cancelling a
replacement's subscription.

Request IDs are allocated once under the admission gate, including requests which
later fail locally. A per-invocation atomic transition distinguishes queued work from
possible transmission. Abandoning queued work prevents the actor from starting it;
otherwise the result stays unknown. No subscription registry or total-count cap is
introduced. Consumers retain uncertain subscription ownership until authoritative
provider evidence or actual producer termination, never inferred reset/unsubscribe
success. Late response metadata alone does not establish business success.
Teardown during queue admission returns `StaleGeneration`, proving no enqueue;
it does not invent a termination category from an earlier `Ready` state. The exact
terminal result remains in the retained generation-ended event and shutdown result.

The delivery owner retains bounded accepted events and independent fixed-size gap
and terminal slots. The first provider shutdown notice is separately retained at its
ordered position, even during a gap; additional queued notices explicitly signal loss. Consumers drain the prefix before receiving the gap. Only the
exact delivered marker can acknowledge recovery. A delivery epoch captured before
batch decoding prevents previously buffered records from publishing or installing a
new gap after recovery; requests still complete while application data is fenced.
A transport end is retained behind the prefix and gap even if no recovery is possible.
The latest-state watch is not an ordered log; ordered consumers use `recv_event`.

## Resource controls and throughput

Capacities are configurable independently, without speculative maximum-payload times
queue-capacity budget checks. Finite frame/record/collection defaults remain client
memory controls, not provider quotas. The former fixed per-DTO cardinalities can be
raised through `max_collection_entries`. Serde's synchronous decoder receives a
scoped, RAII-restored bound; it cannot leak between tasks or remain installed across
an await. The existing finite defaults of direct private decoder tests remain useful
lower-limit fixtures.

The WebSocket retains one incoming message while records are processed incrementally.
Each record yields to the executor, and fair request/data selection prevents a
continuous data stream from excluding command admission. Heartbeats and cancellation
are polled before ordinary work. Liveness expiration pauses while the reader is
deliberately parked behind a buffered batch; finishing the first such batch grants
one full pong-read grace interval per probe. The original probe deadline is retained,
and subsequent batches cannot renew the grace interval. Buffered decoding alone
cannot manufacture a failed probe, while continuous data without pong cannot keep
an unanswered probe alive indefinitely.
Pending deadlines use an ordered index, avoiding a
full pending-map scan on every data record. No completed subscription consumes a
pending slot. Caller-owned user-sync filter lists have no hard ID-count quota;
uniqueness/cross-field validation and the configured outbound byte limit still apply. Keep finite payload controls and size event queues for real consumer
latency; their actual payloads determine memory consumption.

## Regression and migration requirements

Initial red regressions demonstrated cancellation, timeout, malformed co-batched
records, silence, and independent-capacity failures before production changes.
Synthetic local WebSockets exercise the public API, including:

- same-socket timeout/cancellation/refusal recovery and unrelated successful requests;
- accepted-prefix/gap/recovery ordering and terminal evidence under saturation;
- malformed JSON/envelopes plus later valid co-batched completions;
- matching active-pong liveness, failed probe, and preserved terminal errors;
- stale sessions/chart IDs and cancellation during replacement establishment;
- final-owner drop outside Tokio and cancellation of a shutdown wait;
- 3,000 concurrent subscriptions and 20,000 events with paused or actively draining
  consumers, including a 64-event queue on the current-thread runtime;
- queued/start admission races, stale/undelivered acknowledgements, and old delivery epochs.

Consumers migrating from 0.1 must acknowledge nonterminal gaps while still connected,
retain uncertain subscription ownership, and use generation-ended evidence separately.
0.2 is the pre-1.0 breaking version. REST execution, exact decimal contracts, generated
OpenAPI inputs, provider rate budgets, and the registry publishing prohibition remain
unchanged.
