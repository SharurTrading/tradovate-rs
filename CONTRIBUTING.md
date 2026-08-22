<!--
SPDX-FileCopyrightText: 2026 Kevin Monaghan
SPDX-License-Identifier: MIT-0
-->

# Contributing

Contributions are welcome through reviewed pull requests. Every change must follow
[`AGENTS.md`](AGENTS.md) and remain within the provider-client boundary.

By intentionally submitting a contribution, you agree that it is licensed under
the repository's MIT No Attribution License (`MIT-0`) and that you have the right to
submit it on those terms.

## Before changing code

1. Read the stable `TV-*` rules in `AGENTS.md`.
2. Confirm the official Tradovate contract for the operation being changed.
3. Record the documentation URL and access/version date in the pull request when
   the provider document can change without versioning.
4. Decide whether the operation is a safe query or a money-moving mutation before
   designing retry, rate-limit, cancellation, and error behavior.
5. Keep the slice complete: implementation, tests, rustdoc, README/changelog, and
   ADR changes land together when applicable.

## Required local gate

Install `cargo-nextest`, `cargo-deny`, `cargo-audit`, and Gitleaks using approved
versions, then run from the repository root:

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

The narrow audit exception and its mandatory inactive-feature proof are documented
in [`docs/security/advisory-exceptions.md`](docs/security/advisory-exceptions.md).
CI is authoritative even when a local hook or tool is unavailable.

## File and function size

The repository intentionally avoids the large client/model/realtime files common in
API bindings:

- handwritten files under `src/` target 400 physical lines and fail above 600;
- `lib.rs` and every `mod.rs` fail above 200 lines;
- files under `tests/` fail above 800 lines;
- only documented generator output beneath a `generated/` path is exempt.

Generated files must contain `@generated`, `Generator:`, and `Source:` markers in
their first 20 lines and must never be edited manually.

Every production function over 100 physical lines requires explicit manual review.
The pull request must name the function and explain why one cohesive sequence is
clearer than decomposition. The reviewer cites `TV-SIZE-01` when accepting it.

## Tests

Use the lowest layer that proves the contract and add broader coverage when risk
requires it:

- unit tests for validation, exact decoding, protocol codecs, and state machines;
- public integration tests for request paths, headers, bodies, and response models;
- deterministic local TCP/WebSocket fixtures for cancellation and lifecycle races;
- paused Tokio time for retry, timeout, refresh, and rolling-window behavior;
- synthetic schema fixtures for required, optional, null, and unknown values;
- secret-redaction and trace-level dependency-logging regression tests.

Tests use descriptive behavior names and make the arrange/act/assert sequence clear.
Do not weaken an assertion, increase a timeout, or add an ignored marker merely to
hide nondeterminism.

Live tests are exceptional. They must be feature-gated, ignored with a precise
reason, read-only, serialized where provider limits require it, and deliberately
invoked. Never use order placement, cancellation, position mutation, or another
money-moving operation as a routine live probe.

## Security and privacy

Never commit or paste into a pull request:

- credentials, access tokens, refresh tokens, client secrets, or device secrets;
- real account, order, position, fill, or billing data;
- `.env` files, shell transcripts, HAR files, packet captures, or provider dumps;
- material copied from another proprietary or confidential source without
  authorization.

Use synthetic values in fixtures and redact logs before attaching them. Report a
suspected leak or vulnerability through the process in [`SECURITY.md`](SECURITY.md),
not a normal issue.

## Dependency changes

- Prefer no new dependency when a small, readable implementation is sufficient.
- Disable default features and enable only the features the crate actually uses.
- Do not add Git or unknown-registry dependencies without an approved ADR and an
  update to `deny.toml`.
- Review transitive TLS, proxy, retry, logging, and serialization behavior.
- Commit `Cargo.lock` and run the supply-chain gate in the same pull request.

## Pull request checklist

- [ ] The change stays inside the provider-client boundary.
- [ ] Query or mutation semantics are explicit.
- [ ] Secrets remain redacted and transport remains encrypted/bounded.
- [ ] Session and realtime state are revision/generation fenced where applicable.
- [ ] Public IDs and requests cannot represent invalid provider state.
- [ ] Tests cover success, rejection, malformed input, cancellation, and ambiguity
      as applicable.
- [ ] Public rustdoc and operational documentation are current.
- [ ] File-size checks pass; functions over 100 lines are identified for review.
- [ ] The changelog and semantic-versioning impact are recorded.
- [ ] No live credentials, captures, or unauthorized confidential material is
      present.

## Releases

Releases are cut only from the exact reviewed commit merged to protected `main`,
after all quality, supply-chain, and secret checks pass. Tags are annotated,
immutable, and matched by a GitHub release and changelog entry. Cargo registry
publishing remains disabled until a dedicated release-policy change is reviewed.
