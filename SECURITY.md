<!--
Copyright (C) 2026 Kevin Monaghan. All rights reserved.

This file is proprietary and confidential.
Unauthorized copying, use, modification, distribution, or disclosure of this file,
via any medium, is strictly prohibited except under a written agreement with the
copyright owner.
-->

# Security

Use [GitHub private vulnerability reporting](https://github.com/SharurTrading/tradovate-rs/security/advisories/new)
for suspected vulnerabilities. Do not open a normal issue containing credentials,
tokens, account identifiers, order identifiers, provider payloads, or exploit details.

If a secret may have reached Git history, logs, CI output, an issue, or a pull request,
treat it as compromised immediately. Stop further disclosure, notify an authorized
maintainer privately, revoke/rotate the affected credential through the provider,
and preserve only the minimum redacted evidence needed for investigation.

## Security boundary

The library accepts caller-supplied credentials and uses them only for documented
Tradovate authentication flows. It must not:

- read `.env` or shell configuration;
- discover credentials from ambient process state;
- persist credentials or tokens;
- expose public secret/token accessors;
- include secrets in `Debug`, `Display`, errors, tracing fields, URLs shown to callers,
  or provider payload diagnostics.

Callers own secret acquisition, storage, process isolation, and rotation policy.

## Transport requirements

Remote endpoints require HTTPS/WSS with certificate validation. Plain HTTP/WS is
permitted only for exact loopback hosts used by deterministic tests. Redirects,
ambient proxies, dependency-owned retries, and token-bearing dependency request logs
must be disabled unless a security review proves the complete path safe.

HTTP bodies, WebSocket frames/messages, queues, pending requests, and decoded-event
memory are bounded. Malformed remote input is untrusted until validated. Transport
errors exposed to callers must remain opaque when their source could contain an
authorization header, token-bearing URL, or provider body with private data.

## Money-moving operations

Order and position mutations are single-attempt. A timeout, cancellation, disconnect,
malformed response, or other failure after possible provider admission is ambiguous,
not safely retryable. The client blocks subsequent mutations across all clones until
the application reconciles orders, fills, and positions and explicitly acknowledges
that reconciliation.

## Testing and incident artifacts

Normal tests use synthetic loopback fixtures. Live probes are not implemented; any
future probe must be ignored, feature-gated, read-only, and deliberately invoked with
process-scoped credentials. Never commit or attach live payload captures, HAR/PCAP
files, terminal transcripts, or unredacted logs.

Security fixes should include a deterministic regression test that proves the secret,
transport, lifecycle, or ambiguity boundary without embedding sensitive material.

## Supply-chain exceptions

An advisory may be excluded only when its package is provably absent from every
compiled all-feature dependency graph and CI fails if that fact changes. Current
exceptions, evidence, and removal conditions are recorded in
[`docs/security/advisory-exceptions.md`](docs/security/advisory-exceptions.md).
