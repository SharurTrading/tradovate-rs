// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

//! Bearer-token syntax validation.

pub(super) fn is_valid_bearer_token(token: &str) -> bool {
    if token.is_empty() {
        return false;
    }
    let mut padding_started = false;
    let mut token_character_seen = false;
    let valid = token.bytes().all(|byte| {
        if byte == b'=' {
            padding_started = true;
            true
        } else {
            let allowed = byte.is_ascii_alphanumeric()
                || matches!(byte, b'-' | b'.' | b'_' | b'~' | b'+' | b'/');
            token_character_seen |= allowed;
            !padding_started && allowed
        }
    });
    valid && token_character_seen
}
