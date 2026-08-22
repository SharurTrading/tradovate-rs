<!--
SPDX-FileCopyrightText: 2026 Kevin Monaghan
SPDX-License-Identifier: MIT-0
-->

# Third-party notices

The MIT-0 license in [`LICENSE`](LICENSE) covers repository-authored source,
configuration, tests, and documentation. It does not relicense third-party
materials or trademarks. [`RELICENSING.md`](RELICENSING.md) records the scope of
the copyright holder's grant for historical repository revisions.

## Tradovate Partner OpenAPI

`spec/official/openapi-2026-08-21.json` is an unmodified snapshot of the Tradovate
Partner OpenAPI obtained from <https://partner.tradovate.com/openapi.json>. Tradovate
and NinjaTrader retain their rights in that documentation. The snapshot is excluded
from the repository's MIT-0 grant and remains subject to the provider's applicable
terms and any separate Partner agreement. The current public terms were reviewed at
<https://www.tradovate.com/terms-and-eula/> on 22 August 2026; they do not themselves
establish public redistribution or sublicensing rights for the snapshot.

The `Tradovate` and `NinjaTrader` names are trademarks of their respective owners.
This independent project is not affiliated with, endorsed by, or sponsored by
Tradovate or NinjaTrader.

## Rust dependencies

Third-party Rust crates are not relicensed by this repository. Their SPDX license
expressions are reviewed by the `cargo-deny` policy in `deny.toml`. Distributors of
compiled artifacts remain responsible for satisfying the notice and distribution
terms of the dependencies they include.
