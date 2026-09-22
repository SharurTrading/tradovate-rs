<!--
SPDX-FileCopyrightText: 2026 Kevin Monaghan
SPDX-License-Identifier: MIT-0
-->

# Tradovate Rust Client Agent Guide

This file is the repository contract for writing and reviewing `tradovate-client`.
Every rule has a stable identifier so reviews, pull requests, and commits can cite
the exact contract being applied.

## Mission and boundary

Build a small, provider-native, asynchronous Rust client for the official Tradovate
API. The crate owns authentication, secure transport, rate admission, provider wire
models, request validation, and realtime protocol lifecycle. Consumers own trading
strategy, routing policy, portfolio and risk truth, storage, GUI state, and translation
into their own canonical domain.

The repository is maintained publicly under `SharurTrading` and licensed under
MIT-0. It is not a SHARUR engine crate and must not depend on SHARUR or any other
consuming application.

## Priority and review severity

Resolve conflicts in this order:

1. correctness, money safety, security, and cancellation safety;
2. non-negotiable `TV-*` rules and `PROC-*` process rules;
3. locked architecture and lifecycle patterns;
4. API, idiom, documentation, and file-size standards;
5. tooling and optional polish.

Review severities:

- **BLOCKER** — incorrect, insecure, money-unsafe, or violates a non-negotiable.
- **MAJOR** — breaks a locked architecture/lifecycle rule or introduces a banned
  antipattern with material risk.
- **MINOR** — maintainability, documentation, or Rust-idiom gap.
- **NIT** — optional polish; omit unless several related nits cluster together.

## Non-negotiables

| ID | Rule |
| --- | --- |
| **TV-BOUNDARY-01** | The public contract is provider-native and application-independent. No SHARUR, strategy, GUI, portfolio, RMS, or consumer-domain dependency or type may enter this crate. |
| **TV-DECIMAL-01** | Prices, money, balances, margin, fees, commissions, and P&L use `rust_decimal::Decimal` without an `f32`/`f64` round-trip. Provider-defined counts and integral quantities may use their exact documented integer width. |
| **TV-SECRET-01** | Credentials and access/refresh tokens are never logged, formatted into errors, persisted, exposed through public accessors, or included in test fixtures. `Debug` is redacted. Remote endpoints require authenticated encryption; plaintext is allowed only for exact loopback fixtures. |
| **TV-AUTH-01** | One session store is the sole token writer. Network work reads owned, revisioned token snapshots and never holds a lock across I/O. Login, single-flight refresh, expiry, replacement, and invalidation are revision-fenced so stale completion cannot overwrite or erase a newer session. Refresh and WebSocket authorization recheck their basis immediately before transmission; only an ambiguous, transmission-started refresh fails closed. |
| **TV-IDENTITY-01** | Account, contract, order, position, fill, and other provider identities use validated newtypes at public boundaries. Money-moving requests name explicit account and instrument identity; neither is guessed from display text or ambient state. |
| **TV-ENVIRONMENT-01** | Demo/live REST and user-stream endpoints are selected as one validated environment. Shared market-data/replay and explicit partner/custom endpoints cannot be silently mixed with an unrelated environment or downgraded to plaintext. |
| **TV-RUNTIME-01** | The caller owns the Tokio runtime. The library creates no hidden runtime and never blocks an async executor thread. |
| **TV-RESPONSE-01** | Every HTTP response body is bounded while streaming. Structured responses are accepted only after HTTP and provider-level status validation. Missing, contradictory, malformed, or oversized responses are typed failures. |
| **TV-EXEC-01** | Money-moving mutations are never automatically retried, including by HTTP dependency defaults; the shared client must configure `reqwest::retry::never()`. Cancellation or failure after possible admission is an explicit ambiguous outcome and installs a reconciliation-required latch shared by client clones. Queries remain available; mutations resume only after the caller explicitly acknowledges authoritative provider-state reconciliation. Only proven pre-send failures may be classified as not sent. |
| **TV-RATE-01** | Provider rate budgets and cooldowns are shared across client clones. Every outbound attempt receives admission. Queries may wait asynchronously; mutations fail locally before transmission when immediate capacity is unavailable. Provider `429` handling never makes a mutation automatically retryable. |
| **TV-PENALTY-01** | Tradovate penalty-ticket bodies are control responses even when HTTP status is 200. The client enforces `p-time` with a monotonic deadline and correlates `p-ticket` to one client instance, endpoint, exact serialized original safe request, and single claim. `p-captcha` requires operator action. Mutation penalty handling never bypasses `TV-EXEC-01`. |
| **TV-TRANSPORT-01** | Realtime connections authenticate and validate their protocol handshake before readiness. Frames, messages, queues, pending requests, and decoded-event memory are bounded. Lifecycle publication is generation-fenced; one half failing tears down the whole session. |
| **TV-SUBSCRIPTION-01** | Realtime transport owns no canonical subscription or projection truth. Reconnect and transport-gap events tell the caller when to replay its idempotent subscription set and perform snapshot-before-delta recovery. |
| **TV-SYNC-01** | A user socket sends exactly one `user/syncrequest` per authenticated connection generation. A penalty installs the full monotonic cooldown and ends setup; the crate does not retry user synchronization automatically. Its initial entities are a snapshot followed by `props` deltas with an explicit resync path; the transport does not publish a delta stream as complete truth before bootstrap succeeds. |
| **TV-AUTOMATION-01** | Automated order builders require the caller to make automated origin explicit and transmit the provider-required `isAutomated` value. The library does not silently default an automated client to manual order origin. |
| **TV-VALIDATE-01** | Normal CI is deterministic and credential-free. Live probes are feature-gated, ignored, read-only, deliberately invoked, and use no committed captures or account data. Contract, cancellation, race, ambiguity, and recovery tests land with the behavior they validate. |
| **TV-DOC-01** | Every public item has rustdoc; fallible APIs document `# Errors`; unsafe APIs, if ever approved, document `# Safety`. Non-obvious provider and lifecycle invariants are documented beside the code and in the same change. |
| **TV-LICENSE-01** | Repository-authored source, tests, configuration, and documentation are MIT-0 and carry the repository SPDX header where appropriate. `Cargo.toml`, `LICENSE`, generated headers, and CI checks agree. Third-party material retains its own terms. Cargo registry publishing requires a separate reviewed release-policy change. |
| **TV-SIZE-01** | Handwritten `src` files target 400 physical lines and may not exceed 600; `lib.rs` and every `mod.rs` may not exceed 200; test files may not exceed 800. Only documented generated files under a `generated/` boundary are exempt. |
| **TV-SUPPLY-01** | Never suppress an active dependency advisory. A lockfile-only false positive may be ignored only with a checked-in evidence record, a CI feature-graph guard proving the package is inactive under all crate features, and an explicit removal condition. |
| **TV-CURRENT-01** | The REST surface is generated only from the reviewed, hash-pinned current Partner OpenAPI snapshot. The older API explorer and guide-only fragments are evidence of drift, not implementation inputs. Generated files are checked in, never hand-edited, and must reproduce byte-for-byte under `tools/generate_openapi.py --check`; changing the snapshot or hash requires a semantic, safety, and legacy-divergence review. |
| **TV-FOLLOWUP-01** | Every follow-up identified anywhere in a change, including its PR description, commit messages, review replies, source comments, coverage ledgers, plans, and research notes, must be completed in that change or tracked by a GitHub issue before merge. Cite the issue number or link at every mention of the deferred work. This includes gaps discovered in adjacent work, exhausted investigation paths, and deferred reviewer questions. Promises of later work, a separate PR, or work outside the current scope do not count as tracking. An issue does not waive required fixes, tests, or documentation in the current change. |

## Process rules

`PROC-*` rules are operator rulings that govern repository workflow — issue
triage, pull-request triage, review and landing, and agent attribution. They are
enforced with the severities above and cited by rule ID, like the non-negotiables.

### PROC-ATTRIB — attribution of agent posts

An AI agent posting to GitHub under the operator's login — a PR description, an
issue, a comment or review reply, an inline comment, a release, any other
publication — states the EXACT MODEL that authored it in the artifact's own body
(a footer line naming the model is the usual shape), because the post carries the
operator's identity while speaking with the agent's judgment, and a reader — the
operator's future self, a reviewer, an auditor — is owed the distinction between
the operator's voice and the machine's. The attribution names the model identifier
the harness reports (e.g. GLM-5.3), never a generic "an AI" and never the harness
or client standing in the model's place, and it lives in the text every reader
sees — a machine-readable trailer the GitHub UI hides is not disclosure.

```text
AI-Agent: GLM-5.3
```

- Never invent, embellish, or round a version. A vendor family and a product
  persona are not model names.
- Disclose in the agent's own words. A signature, badge, or hidden trailer that
  the hosting platform adds to everything is not the agent stating its model.
- Write it in every surface the agent publishes: pull request and issue
  descriptions, pull request and issue comments, review bodies, each inline
  review comment, release notes, and any later comment that answers review
  feedback.
- Each inline review comment needs its own line. Review UIs surface them as
  standalone comments, so a disclosure elsewhere does not cover them.
- Re-post or edit a surface and the line stays accurate and current. If work
  moves to a different model, the post names the new model.
- Stay truthful and current. A model cannot verify its own routing identity with
  certainty, so the operator may correct the line; the correction must be
  applied rather than argued.
- This governs agent posts only. The operator's own posts carry no such line,
  and an agent must not add one to a human post or remove one the operator
  wrote.
- An agent that publishes any surface without its model disclosed violates this
  rule. That is a governance BLOCKER under the severity definitions above, not
  a NIT. A reviewer that notices a missing line cites `PROC-ATTRIB` with the
  offending surface and requires the disclosure before merge. An agent that sees
  its own surface missing the line fixes it immediately instead of waiting for
  review.

Commit messages, branches, and source comments are not posts; they are covered
by the ownership and follow-up rules that already apply to them.

### PROC-ISSUE-TRIAGE — issue classification

Every issue is classified when it is created, and a mis-classified one is
corrected whenever it is touched. Four things, all mandatory: the native issue
type (exactly one of Bug, Feature, Task), the kind label that spells it
(bug / enhancement / task — a fixed 1:1 mapping onto the type, so a reader
filtering by label and a reader filtering by type see the same set of work), and
the Priority ISSUE FIELD on the issue itself (Urgent / High / Medium / Low),
defined once at the ORGANIZATION level. It is an issue field — not a project
field, and never a label: the value travels with the issue instead of living on
one board's item, it holds exactly one value that is re-set as urgency changes
rather than accumulating stale ones, and a label would be free to disagree with
it. It is read and written through the issue under ordinary repo scope —
GET/POST on the issue's issue-field-values, with the field id from the
organization's issue-fields — so a lane that can read the issue can read its
priority, and no project scope is involved.

The priority ladder is: Urgent — the platform is wrong about money, orders, or
account state right now, or a live session is blocked; other work stops for it.
High — it blocks the next live session, or the next step of an active plan.
Medium — ordinary work, and the DEFAULT: an issue nobody has argued is urgent,
high, or low is Medium, never unset. Low — polish, nits, and anything deferrable
without loss.

The fourth is the difficulty label — exactly one of difficulty: hard /
difficulty: medium / difficulty: easy (operator direction 2026-09-18) — and it
routes the issue to the class of agent that should take it, which is why it is a
label where priority is a field: it is working state the repository's own issue
list filters by, not a fact that must travel with the issue. The ladder is:
hard — a frontier agent: architecture or identity refactors, money-path and
reconciliation semantics, concurrency or lifecycle decisions, research-heavy
evidence work, wide cross-crate changes. medium — a strong coding agent: real
engineering on a bounded surface the issue itself already specifies. easy — a
basic coding agent: mechanical, well-scoped work with a clear acceptance check;
operator-only trackers (live verification, armed probes, decisions awaiting the
operator under PROC-DECIDE) are easy because no agent performs them at all.
Difficulty is judged from the issue's own scope at creation and re-set whenever
understanding changes.

A documentation label sits beside the kind label when the work is docs, and any
other repository label is welcome; none of those is required. An issue missing
one of the four is a finding on the next PR that touches it, and in the tracker
it is a gap the operator is asked to fill rather than a state to leave standing.

### PROC-PR-TRIAGE — pull-request classification

A PR carries the same classification as the issue it closes, so the tracker and
the PR list read as one body of work: exactly one kind label (bug / enhancement
/ task, matching what the diff changes), and the same Priority issue field set
to its linked issue's value — a PR is an issue to the field API, so a reviewer
reads the value from the PR itself rather than taking it on trust. The label is
not redundant with the issue's type — native issue types exist on issues only,
so on a PR the kind label IS the type, and a PR that omits it is unclassified
however well its issue is labelled. The priority is INHERITED, never re-argued:
a PR whose value differs from its issue's is a finding, and a genuine
disagreement is settled on the issue, where the ladder lives. A PR closing no
issue sets its own priority by that same ladder, and says in its description why
it closes none. The value is never restated as prose in the description: one
field, one home, and a copy could only be free to disagree with it.

### PROC-REVIEW — review and landing

All work lands via PR into main. The operator manually starts the configured
review agent for every PR; its findings cite rule IDs from this file. Every
review conversation — inline, including outdated threads, and recommendations
outside the diff in review-summary comments — receives an explicit disposition
and is resolved before merge, whether relevant or adopted or not: relevant
adopted findings are fixed, while irrelevant or declined findings are resolved
with the recorded reason and require no unnecessary code. Human (operator)
review is required on: anything in crates/domain/ or crates/application/,
order/RMS/ledger logic, and any new dependency. One FEATURE per PR (operator
ruling 2026-09-02): a PR is split only when its verification needs separate
diffs — a pure-move proof, a red-first pin that must land before the change it
guards — or when parallel lanes need disjoint files; never by step count or
description length. A plan's steps are the implementer's checklist, not PR
boundaries. A plan whose groups only make sense together lands as ONE PR when
the operator rules so (2026-09-05: the keyed-connection-set and
provider-definition plan); the split criteria above govern everything else.

## Locked architecture

### Capability-oriented source tree

Organize source by ownership and provider capability, not by a generic collection
of all structs or functions:

```text
src/
  lib.rs
  error.rs
  environment.rs
  ids.rs
  decimal.rs
  auth/
  client/
  rate_limit/
  api/
  realtime/
```

- `lib.rs` contains crate docs, module declarations, selective re-exports, and
  narrow wiring only.
- `environment` owns validated live/demo/custom endpoint sets.
- `auth` owns credential wrappers and session/token lifecycle.
- `client` owns construction and shared HTTP execution policy.
- `api` is split by provider capability such as accounts, contracts, market data,
  orders, positions, and fills. Requests live with the capability that uses them.
- `realtime` is split into lifecycle, codec, flow control, event delivery, and
  connection/upgrade concerns before any file approaches the hard line limit.
- Shared wire helpers remain private unless a stable consumer need is proven.

Do not create a workspace or additional crate merely to satisfy the file-size rule.
Split crates only when a real ownership, dependency, trust, or runtime boundary exists.

### Locked patterns

| ID | Pattern |
| --- | --- |
| **TV-PAT-FACADE-01** | A thin crate-root facade selectively re-exports the intended API. Internal modules and transport details remain private. |
| **TV-PAT-CAP-01** | Provider operations and models are grouped by capability. New capabilities are additive modules rather than edits to a giant client or model bucket. |
| **TV-PAT-CQS-01** | Query execution and mutation execution are separate policy paths. Retry, rate admission, cancellation, and error classification must make the distinction visible. |
| **TV-PAT-STATE-01** | Mutable session and connection truth has one writer. Readers receive owned snapshots or events; stale work is rejected by revision/generation checks. |
| **TV-PAT-RAII-01** | Attempt, connection, subscription-invocation, and background-task guards make cleanup and cancellation explicit. Every spawned task has a cancellation mechanism and tracked teardown; dropping a `JoinHandle` is not cancellation. |
| **TV-PAT-BOUNDS-01** | All remote-input and live-path buffers have named limits and explicit overflow behavior. Continuing after a dropped realtime update is prohibited unless the caller performs explicit recovery. |
| **TV-PAT-BUILDER-01** | Use a validated builder when optional configuration exceeds two fields or construction has cross-field invariants. Builders are `#[must_use]`; invalid intermediate values never reach transport. |
| **TV-PAT-NEWTYPE-01** | Public provider IDs and invariant-bearing scalar values have private fields, validated constructors/parsers, and validating Serde implementations. |
| **TV-PAT-ERROR-01** | Library failures use focused `thiserror` enums with sources and public-safe context. Expected remote or input failures return `Result`; they do not panic. |
| **TV-PAT-FUTURE-01** | Response enums that may grow are `#[non_exhaustive]` and preserve unknown provider codes. Request builders reject unknown or undocumented values rather than transmitting them. |
| **TV-PAT-BORROW-01** | APIs accept `&str`, slices, and borrowed IDs when they do not retain ownership. Owned inputs are used only when stored, queued, or transferred. |
| **TV-PAT-ASYNC-01** | Never hold a lock guard across `.await`. CPU/blocking work uses an explicit blocking boundary. Cancellation is considered at every admission and commit point. |
| **TV-PAT-EXACT-01** | Decimal JSON is encoded and decoded from exact tokens. Do not enable a dependency-global feature that silently changes downstream Serde behavior without a documented compatibility analysis. |
| **TV-PAT-TEST-01** | Public integration tests exercise exact wire contracts through synthetic loopback fixtures; unit tests cover private codecs, validation, state machines, and invariants. |

## Rust API standards

- Prefer the smallest API that expresses a provider contract completely.
- Use private fields for invariant-bearing request and configuration types.
- Use `From`/`TryFrom`, `FromStr`, `AsRef`, and standard iterator conventions.
- Add `Debug`, `Clone`, `Copy`, equality, ordering, and hashing only when their
  semantics are correct. Secret types implement redacted `Debug` manually.
- Mark public enums/response structs `#[non_exhaustive]` when the provider can add
  variants or fields.
- Use `#[must_use]` on builders, guards, and pure results whose loss is suspicious.
- Return borrowed views and iterators rather than exposing internal collection choices.
- Prefer enums and concrete state types to string flags or unstructured maps.
- Keep raw protocol escape hatches crate-private by default. A public raw API needs
  a documented consumer requirement, bounds, security analysis, and typed alternative.
- Public errors must not embed raw response bodies, request URLs containing tokens,
  authorization headers, credentials, or provider messages that may contain secrets.
- `unsafe` is forbidden unless an ADR establishes why it is necessary, states its
  invariants, and confines it to the smallest possible module behind a safe API.

## Realtime lifecycle contract

1. Snapshot a valid token, then revalidate its revision and expiry immediately before
   sending connection authorization.
2. Bound connect, authentication/handshake, request completion, and close waits.
3. Publish readiness only after the authenticated protocol handshake succeeds.
4. Allocate one generation for each connection attempt and fence every task/event by it.
5. Send Tradovate's `[]` client heartbeat from an independent monotonic 2.5-second
   schedule; incoming traffic does not replace the client heartbeat requirement.
6. Issue exactly one user synchronization request for an authenticated user-socket generation; a penalty ends setup without an automatic retry.
7. On actual reader/writer failure, remote closure, or a failed active WebSocket
   ping/pong check, end the generation. Ordinary silence, application-record errors,
   queue pressure, and subscription refusals do not end healthy sockets.
8. Correlate provider replies with bounded pending requests and reclaim slots on every exit.
9. If a request may have been admitted but its completion is lost, retain an unknown
   outcome without resending or reusing its identity. Cancel only its wait; preserve
   the socket and unrelated operations.
10. On event overflow or malformed application records, fence data publication and
   retain a nonterminal continuity gap after the accepted event prefix. Keep replies
   and keepalives running. Resume only after the consumer installs its recovery
   boundary and acknowledges the exact delivered marker; stale buffered records
   must not cross that boundary. Retain terminal evidence after the ordered gap.
11. The library does not automatically reconnect. A caller-created replacement uses a
   fresh token snapshot, and the caller replays its canonical subscriptions; transport
   retains neither subscriptions nor a replay queue.
12. Dropping the last caller-owned handle cancels every library task, including
   outside an ambient Tokio context. Track actual termination; dropping a join
   handle is not proof of termination. Operational handles retain immutable
   generation identity without lifecycle authority; reject stale admission before
   enqueueing. Publish generation-ended evidence only after socket producers stop.

## Banned antipatterns

| ID | Reject |
| --- | --- |
| **TV-ANTI-FLOAT-01** | `f32`/`f64` for any financial value or an exact provider decimal token. |
| **TV-ANTI-STRINGLY-01** | Raw strings/integers for public IDs, states, sides, order types, units, or structured errors. |
| **TV-ANTI-SECRET-01** | Secret getters, derived `Debug` on secrets, token-bearing errors, persisted credentials, captured live payloads, or logging raw requests. |
| **TV-ANTI-ENV-01** | Library-owned `.env` loading or ambient credential/proxy discovery. Callers inject secrets and explicit proxy configuration. |
| **TV-ANTI-RETRY-01** | Generic middleware retrying mutations, retries that bypass admission, or treating cancellation as proof a request was not sent. |
| **TV-ANTI-UNBOUNDED-01** | Unbounded response reads, channels, pending maps, frame aggregation, event queues, or reconnect loops. |
| **TV-ANTI-LOCKAWAIT-01** | Holding any mutex/rwlock guard across `.await`. |
| **TV-ANTI-HIDDENRT-01** | Creating or blocking a Tokio runtime inside the library. |
| **TV-ANTI-TRUTH-01** | Realtime transport retaining canonical subscriptions, order state, positions, or account projections. |
| **TV-ANTI-GIANTFILE-01** | Giant `client.rs`, `models.rs`, or realtime monoliths; splitting unrelated capabilities only after a file already exceeds its hard limit. |
| **TV-ANTI-BUCKET-01** | A catch-all models, helpers, utils, manager, or common module without one precise ownership purpose. |
| **TV-ANTI-CLONE-01** | Cloning merely to silence the borrow checker, especially on hot or per-message paths. Fix ownership or state shape. |
| **TV-ANTI-PANIC-01** | `unwrap`, `expect`, `panic!`, unchecked indexing, or assertions in production paths for expected input/remote failures. |
| **TV-ANTI-SUPPRESS-01** | `#[allow]`, `#[expect]`, ignored tests, or formatting suppression without a nearby, specific rationale. |
| **TV-ANTI-ABSTRACT-01** | Provider-generic traits, excessive generics, or extra crates created without a demonstrated boundary and at least one real use. |
| **TV-ANTI-RAW-01** | Making raw JSON values, endpoint paths, command strings, or dependency transport types the primary public API. |

## File and function size policy

`scripts/ci/check_file_sizes.sh` enforces physical line counts:

- handwritten `src/**/*.rs`: warning above 400, failure above 600;
- `src/lib.rs` and every `mod.rs`: failure above 200;
- `tests/**/*.rs`: failure above 800.

Only files inside a path segment named `generated` may bypass the hard maximum, and
each must declare `@generated`, `Generator:`, and `Source:` in its first 20 lines.
Generated code is never hand-edited. Prefer splitting generated output when the
generator can do so without damaging its contract.

Production functions over 100 physical lines require explicit manual review in the
pull request. The reviewer must confirm that sequencing genuinely benefits from one
function and cite `TV-SIZE-01`; otherwise decompose it by responsibility. Tests and
declarative match tables may justify a longer function but still require review.

## Validation requirements

Every behavior change supplies the applicable layers:

- unit tests for validation, codecs, state transitions, and exact values;
- public integration tests for endpoint request/response contracts;
- cancellation and race tests for token and connection lifecycle;
- deterministic overflow, timeout, retry, and rate-limit tests;
- penalty-ticket, captcha, HTTP-200 business-failure, and lockout tests;
- heartbeat, one-sync-per-generation, reconnect, and resubscription tests;
- fixture/schema tests proving required, optional, null, and unknown fields;
- secret-redaction and dependency-logging regression tests;
- ignored read-only live probes only when deterministic fixtures cannot verify the
  provider interaction.

Fixtures contain synthetic IDs, accounts, orders, tokens, and market data. Never copy
live account data, WebSocket captures, HAR files, or credentials into the repository.

## Documentation and change discipline

- Update rustdoc, README, changelog, and ADRs in the same slice as behavior.
- Cite official Tradovate documentation for wire-contract decisions in code comments
  or ADRs; record the access/version date when the provider document is mutable.
- Run `python3 tools/generate_openapi.py --check` whenever the current REST contract,
  generator, or generated output is touched. Never patch generated Rust directly.
- Commit `Cargo.lock` and use locked dependency commands in CI.
- Keep third-party dependencies minimal, audited, license-approved, and free of
  default features that widen the network or TLS stack unintentionally.
- Never weaken a test, bound, lint, advisory, or security check merely to make CI pass.

## Review output

One finding per block:

```text
[MAJOR] TV-PAT-CQS-01 — src/api/orders.rs:142
What: order placement uses the generic query retry loop.
Why: a timeout can duplicate a money-moving request.
Fix: route placement through the single-attempt mutation executor and return an
ambiguous outcome after possible admission.
```

End reviews with exactly one verdict:

- `REVIEW: REQUEST_CHANGES — <reason>` when any BLOCKER or MAJOR exists.
- `REVIEW: PASS — <summary>` when only MINOR/NIT findings remain or none exist.

## Release gate

Every release originates from a reviewed pull request and the exact merged
commit on protected `main`. Before tagging:

1. confirm official provider terms permit the implemented use;
2. scan the entire reachable history for secrets and unauthorized confidential or
   third-party material;
3. verify fixtures are synthetic and documentation is public-safe;
4. classify the semantic-versioning impact of public API changes;
5. update version, lockfile, changelog, and release notes in the same PR;
6. pass formatting, size, strict Clippy, tests, rustdoc, dependency, license,
   advisory, and secret gates;
7. create an immutable annotated tag and matching GitHub release.

Do not publish the package to crates.io or another registry until a separate
release-policy change removes `publish = false` and adds the registry gates.
