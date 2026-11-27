// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

use super::*;

fn token_store() -> Arc<TokenStore> {
    Arc::new(TokenStore::new(WallClock::supplied(|| {
        Timestamp::UNIX_EPOCH
    })))
}

fn session(token: &str) -> InstalledSession {
    let timestamp = "2099-01-01T00:00:00Z"
        .parse::<Timestamp>()
        .unwrap_or_else(|error| panic!("fixture timestamp: {error}"));
    let user_id = UserId::new(1).unwrap_or_else(|error| panic!("fixture user ID: {error}"));
    InstalledSession::try_new(
        token.to_owned(),
        None,
        SessionInfo::new(user_id, timestamp, false),
        Timestamp::UNIX_EPOCH,
    )
    .unwrap_or_else(|error| panic!("fixture session: {error}"))
}

#[test]
fn delayed_authentication_cannot_replace_newer_attempt() {
    let store = token_store();
    let old = store.begin_authentication();
    let current = store.begin_authentication();
    assert!(current.commit(session("current")).is_ok());
    assert!(matches!(
        old.commit(session("old")),
        Err(Error::SupersededAuthentication)
    ));
    let snapshot = store.snapshot(TokenKind::Access);
    assert!(matches!(
        snapshot.as_ref().map(TokenSnapshot::expose),
        Ok("current")
    ));
}

#[test]
fn cancelled_renewal_before_transmission_retains_its_basis_session() {
    let store = token_store();
    let auth = store.begin_authentication();
    assert!(auth.commit(session("current")).is_ok());
    let renewal = store.begin_renewal();
    assert!(renewal.is_ok());
    drop(renewal);
    assert!(store.snapshot(TokenKind::Access).is_ok());
    let next = store
        .begin_renewal()
        .unwrap_or_else(|error| panic!("released renewal slot: {error}"));
    next.retain();
}

#[test]
fn cancelled_renewal_after_transmission_started_invalidates_its_basis_session() {
    let store = token_store();
    let auth = store.begin_authentication();
    assert!(auth.commit(session("current")).is_ok());
    let mut renewal = store
        .begin_renewal()
        .unwrap_or_else(|error| panic!("fixture renewal: {error}"));
    assert!(renewal.arm().is_ok());
    drop(renewal);
    assert!(matches!(
        store.snapshot(TokenKind::Access),
        Err(Error::Unauthenticated)
    ));
}

#[test]
fn definitive_renewal_rejection_can_retain_current_session() {
    let store = token_store();
    let auth = store.begin_authentication();
    assert!(auth.commit(session("current")).is_ok());
    let renewal = store
        .begin_renewal()
        .unwrap_or_else(|error| panic!("fixture renewal: {error}"));
    renewal.retain();
    assert!(store.snapshot(TokenKind::Access).is_ok());
}

#[test]
fn renewal_is_single_flight_and_releases_its_slot() {
    let store = token_store();
    let auth = store.begin_authentication();
    assert!(auth.commit(session("current")).is_ok());
    let renewal = store
        .begin_renewal()
        .unwrap_or_else(|error| panic!("fixture renewal: {error}"));
    assert!(matches!(
        store.begin_renewal(),
        Err(Error::RenewalInProgress)
    ));
    renewal.retain();
    let next = store
        .begin_renewal()
        .unwrap_or_else(|error| panic!("released renewal slot: {error}"));
    next.retain();
}

#[test]
fn session_install_rejects_invalid_tokens_and_expiration() {
    let future = "2099-01-01T00:00:00Z"
        .parse::<Timestamp>()
        .unwrap_or_else(|error| panic!("fixture timestamp: {error}"));
    let expired = "2000-01-01T00:00:00Z"
        .parse::<Timestamp>()
        .unwrap_or_else(|error| panic!("fixture timestamp: {error}"));
    let user_id = UserId::new(1).unwrap_or_else(|error| panic!("fixture user ID: {error}"));

    assert!(matches!(
        InstalledSession::try_new(
            String::new(),
            None,
            SessionInfo::new(user_id, future, false),
            expired,
        ),
        Err(Error::InvalidAuthenticationResponse { .. })
    ));
    assert!(matches!(
        InstalledSession::try_new(
            "token with whitespace".to_owned(),
            None,
            SessionInfo::new(user_id, future, false),
            expired,
        ),
        Err(Error::InvalidAuthenticationResponse { .. })
    ));
    assert!(matches!(
        InstalledSession::try_new(
            "=".to_owned(),
            None,
            SessionInfo::new(user_id, future, false),
            expired,
        ),
        Err(Error::InvalidAuthenticationResponse { .. })
    ));
    assert!(matches!(
        InstalledSession::try_new(
            "valid-token".to_owned(),
            None,
            SessionInfo::new(user_id, expired, false),
            expired,
        ),
        Err(Error::InvalidAuthenticationResponse { .. })
    ));
}

#[test]
fn a_session_is_current_until_the_supplied_clock_reaches_its_expiry() {
    let before = Timestamp::constant(1_700_000_000, 999_999_999);
    let expires_at = Timestamp::constant(1_700_000_001, 0);
    let now = Arc::new(Mutex::new(before));
    let clock = Arc::clone(&now);
    let store = Arc::new(TokenStore::new(WallClock::supplied(move || *clock.lock())));
    let user = UserId::new(1).unwrap_or_else(|error| panic!("fixture user ID: {error}"));
    let session = InstalledSession::try_new(
        "valid-token".to_owned(),
        None,
        SessionInfo::new(user, expires_at, false),
        before,
    )
    .unwrap_or_else(|error| panic!("fixture session: {error}"));
    assert!(store.begin_authentication().commit(session).is_ok());

    let access = store
        .snapshot(TokenKind::Access)
        .unwrap_or_else(|error| panic!("access one nanosecond before expiry: {error}"));
    assert!(store.is_current(&access));
    *now.lock() = expires_at;
    assert!(!store.is_current(&access));
    assert!(matches!(
        store.snapshot(TokenKind::Access),
        Err(Error::Unauthenticated)
    ));
    assert!(store.session_info().is_none());
}

#[test]
fn snapshot_defensively_invalidates_an_expired_session() {
    let now = Timestamp::constant(1_700_000_001, 0);
    let expired = Timestamp::constant(1_700_000_000, 0);
    let user = UserId::new(1).unwrap_or_else(|error| panic!("fixture user ID: {error}"));
    let store = TokenStore::new(WallClock::supplied(move || now));
    store.state.lock().session = Some(InstalledSession {
        access_token: SecretString::from("valid-token".to_owned()),
        market_data_token: None,
        info: SessionInfo::new(user, expired, false),
    });

    assert!(matches!(
        store.snapshot(TokenKind::Access),
        Err(Error::Unauthenticated)
    ));
    assert!(store.state.lock().session.is_none());
}
