// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: LicenseRef-Proprietary

//! Validated Tradovate service endpoints.

use std::net::{Ipv4Addr, Ipv6Addr};

use url::{Host, Url};

use crate::ConfigError;

const DEMO_REST: &str = "https://demo.tradovateapi.com/v1";
const LIVE_REST: &str = "https://live.tradovateapi.com/v1";
const DEMO_USER_WS: &str = "wss://demo.tradovateapi.com/v1/websocket";
const LIVE_USER_WS: &str = "wss://live.tradovateapi.com/v1/websocket";
const MARKET_DATA_WS: &str = "wss://md.tradovateapi.com/v1/websocket";
const REPLAY_WS: &str = "wss://replay.tradovateapi.com/v1/websocket";

/// A supported retail Tradovate environment.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum Environment {
    /// Tradovate's simulated-trading environment.
    Demo,
    /// Tradovate's live-trading environment.
    Live,
}

impl Environment {
    /// Returns the validated endpoints for this environment.
    #[must_use]
    pub fn endpoints(self) -> EndpointSet {
        let (rest, user) = match self {
            Self::Demo => (DEMO_REST, DEMO_USER_WS),
            Self::Live => (LIVE_REST, LIVE_USER_WS),
        };
        EndpointSet {
            rest: parse_builtin(rest),
            user_websocket: parse_builtin(user),
            market_data_websocket: parse_builtin(MARKET_DATA_WS),
            replay_websocket: parse_builtin(REPLAY_WS),
        }
    }
}

/// Validated REST and WebSocket endpoints used by one client.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EndpointSet {
    rest: Url,
    user_websocket: Url,
    market_data_websocket: Url,
    replay_websocket: Url,
}

impl EndpointSet {
    /// Validates a custom endpoint set.
    ///
    /// Custom plaintext URLs are accepted only for exact loopback hosts so
    /// deterministic local fixtures do not weaken production transport.
    ///
    /// # Errors
    ///
    /// Returns [`ConfigError`] when a URL is invalid, carries credentials or a
    /// fragment, uses the wrong scheme, or uses plaintext away from loopback.
    pub fn custom(
        rest: &str,
        user_websocket: &str,
        market_data_websocket: &str,
        replay_websocket: &str,
    ) -> Result<Self, ConfigError> {
        Ok(Self {
            rest: validate_url("REST", rest, UrlKind::Rest)?,
            user_websocket: validate_url("user WebSocket", user_websocket, UrlKind::WebSocket)?,
            market_data_websocket: validate_url(
                "market-data WebSocket",
                market_data_websocket,
                UrlKind::WebSocket,
            )?,
            replay_websocket: validate_url(
                "replay WebSocket",
                replay_websocket,
                UrlKind::WebSocket,
            )?,
        })
    }

    /// Returns the REST base URL.
    #[must_use]
    pub const fn rest(&self) -> &Url {
        &self.rest
    }

    /// Returns the user-data WebSocket URL.
    #[must_use]
    pub const fn user_websocket(&self) -> &Url {
        &self.user_websocket
    }

    /// Returns the market-data WebSocket URL.
    #[must_use]
    pub const fn market_data_websocket(&self) -> &Url {
        &self.market_data_websocket
    }

    /// Returns the replay WebSocket URL.
    #[must_use]
    pub const fn replay_websocket(&self) -> &Url {
        &self.replay_websocket
    }

    /// Reports whether demo-only REST operations may use this endpoint set.
    pub(crate) fn permits_demo_only_rest(&self) -> bool {
        self.rest.as_str().trim_end_matches('/') == DEMO_REST || is_loopback(&self.rest)
    }

    /// Reports whether live-only Partner operations may use this endpoint set.
    pub(crate) fn permits_live_only_rest(&self) -> bool {
        self.rest.as_str().trim_end_matches('/') == LIVE_REST || is_loopback(&self.rest)
    }
}

#[derive(Clone, Copy)]
enum UrlKind {
    Rest,
    WebSocket,
}

fn validate_url(label: &'static str, raw: &str, kind: UrlKind) -> Result<Url, ConfigError> {
    let url = Url::parse(raw).map_err(|source| ConfigError::InvalidUrl { label, source })?;
    if !url.username().is_empty() || url.password().is_some() {
        return Err(ConfigError::UrlCredentials { label });
    }
    if url.query().is_some() || url.fragment().is_some() {
        return Err(ConfigError::UrlSuffix { label });
    }
    let secure_scheme = match kind {
        UrlKind::Rest => "https",
        UrlKind::WebSocket => "wss",
    };
    let fixture_scheme = match kind {
        UrlKind::Rest => "http",
        UrlKind::WebSocket => "ws",
    };
    let loopback = is_loopback(&url);
    if url.scheme() != secure_scheme && !(loopback && url.scheme() == fixture_scheme) {
        return Err(ConfigError::InsecureUrl { label });
    }
    Ok(url)
}

fn is_loopback(url: &Url) -> bool {
    match url.host() {
        Some(Host::Domain("localhost")) => true,
        Some(Host::Ipv4(address)) => address == Ipv4Addr::LOCALHOST,
        Some(Host::Ipv6(address)) => address == Ipv6Addr::LOCALHOST,
        _ => false,
    }
}

fn parse_builtin(raw: &str) -> Url {
    match Url::parse(raw) {
        Ok(url) => url,
        Err(error) => unreachable!("built-in Tradovate URL must be valid: {error}"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_loopback_fixtures() {
        let result = EndpointSet::custom(
            "http://127.0.0.1:8080/v1",
            "ws://localhost:8081/v1/websocket",
            "ws://127.0.0.1:8082/v1/websocket",
            "ws://localhost:8083/v1/websocket",
        );
        assert!(result.is_ok());
    }

    #[test]
    fn rejects_remote_plaintext() {
        let result = EndpointSet::custom(
            "http://example.com/v1",
            DEMO_USER_WS,
            MARKET_DATA_WS,
            REPLAY_WS,
        );
        assert!(matches!(result, Err(ConfigError::InsecureUrl { .. })));
    }

    #[test]
    fn demo_only_operations_reject_the_builtin_live_rest_endpoint() {
        assert!(Environment::Demo.endpoints().permits_demo_only_rest());
        assert!(!Environment::Live.endpoints().permits_demo_only_rest());
    }

    #[test]
    fn live_only_operations_reject_the_builtin_demo_rest_endpoint() {
        assert!(Environment::Live.endpoints().permits_live_only_rest());
        assert!(!Environment::Demo.endpoints().permits_live_only_rest());
    }
}
