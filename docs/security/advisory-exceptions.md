<!--
Copyright (C) 2026 Kevin Monaghan. All rights reserved.

This file is proprietary and confidential.
Unauthorized copying, use, modification, distribution, or disclosure of this file,
via any medium, is strictly prohibited except under a written agreement with the
copyright owner.
-->

# Advisory exceptions

Every entry is a narrow lockfile-scanner exception, not acceptance of an active
vulnerability. CI must independently prove that the affected package is absent from
the compiled dependency graph under every crate feature.

## RUSTSEC-2026-0235 — `rkyv` 0.7

- Reviewed: 2026-08-21.
- Advisory: <https://rustsec.org/advisories/RUSTSEC-2026-0235.html>.
- Lockfile path: the optional `rust_decimal` 1.42.1 `rkyv` feature records `rkyv`
  0.7.46 in `Cargo.lock` even though the feature is not enabled.
- Runtime/build exposure: none. `rust_decimal` has default features disabled and
  enables only `std`. The repository defines no feature that enables
  `rust_decimal/rkyv` or `rust_decimal/rkyv-safe`.
- Enforced evidence: CI runs `cargo tree --locked --all-features --edges
  normal,build,dev --prefix none` and fails if any `rkyv` 0.7 package is active
  before passing this advisory ID to `cargo audit --ignore`.
- Removal condition: remove the exception as soon as `rust_decimal` no longer
  resolves vulnerable `rkyv` into the lockfile, `cargo audit` becomes
  feature-graph-aware, or any crate feature activates `rkyv` (the last condition
  must fail CI rather than extending this exception).

Upstream documents that its `rkyv` feature is optional and intentionally pinned to
0.7 for compatibility: <https://github.com/paupino/rust-decimal#rkyv>.
