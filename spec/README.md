<!--
SPDX-FileCopyrightText: 2026 Kevin Monaghan
SPDX-License-Identifier: MIT-0
-->

# Official contract snapshot

`official/openapi-2026-08-21.json` is the pinned current Tradovate Partner
OpenAPI 3.1 contract used to generate `src/api/current/generated`.

- Official source: <https://partner.tradovate.com/openapi.json>
- Retrieved: 2026-08-21 (Australia/Sydney)
- API title/version: `REST API Endpoints` / `1.0.0`
- Operations: 350
- Component schemas: 278
- SHA-256: `37caeccf4b0913460a788fcaf4c902497059b8ffe6f6355512e6c08eaacde769`

The older `api.tradovate.com` explorer is intentionally not an implementation
source. Regeneration fails if the pinned file hash changes. Review a new
official snapshot and its semantic/API-safety differences before updating the
hash and generated output.

## Third-party status

The snapshot is Tradovate documentation obtained from the official URL above. It is
not covered by this repository's MIT-0 license and remains subject to Tradovate's
terms. The snapshot is retained verbatim only as the reviewed generator input; the
repository does not claim ownership of it or grant downstream rights to it. See the
repository-level [`THIRD_PARTY_NOTICES.md`](../THIRD_PARTY_NOTICES.md) before any
public distribution.
