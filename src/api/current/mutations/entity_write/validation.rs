// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

//! Validation shared by current entity-write mutations.

use crate::{Error, api::current::users::TradeDate};

pub(super) fn required_text(value: &str, field: &'static str) -> Result<(), Error> {
    if value.is_empty() || value.trim() != value {
        return Err(Error::InvalidRequest {
            field,
            reason: "must be non-empty and have no surrounding whitespace",
        });
    }
    if value.chars().any(char::is_control) {
        return Err(Error::InvalidRequest {
            field,
            reason: "must not contain control characters",
        });
    }
    Ok(())
}

pub(super) fn optional_text(value: Option<&str>, field: &'static str) -> Result<(), Error> {
    value.map_or(Ok(()), |value| required_text(value, field))
}

pub(super) fn country_code(value: &str, field: &'static str) -> Result<(), Error> {
    if value.len() == 2 && value.bytes().all(|byte| byte.is_ascii_alphabetic()) {
        Ok(())
    } else {
        Err(Error::InvalidRequest {
            field,
            reason: "must be a two-letter ISO country code",
        })
    }
}

pub(super) fn combined_names(first_name: &str, last_name: &str) -> Result<(), Error> {
    let count = first_name
        .chars()
        .count()
        .checked_add(last_name.chars().count())
        .ok_or(Error::InvalidRequest {
            field: "firstName/lastName",
            reason: "combined character count overflowed",
        })?;
    if count > 60 {
        return Err(Error::InvalidRequest {
            field: "firstName/lastName",
            reason: "combined length must not exceed 60 characters",
        });
    }
    Ok(())
}

pub(super) fn trade_date(value: &TradeDate, field: &'static str) -> Result<(), Error> {
    let (year, month, day) = date_key(value);
    if year <= 0 {
        return Err(Error::InvalidRequest {
            field,
            reason: "year must be positive",
        });
    }
    let max_day = match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 if is_leap_year(year) => 29,
        2 => 28,
        _ => {
            return Err(Error::InvalidRequest {
                field,
                reason: "month must be between 1 and 12",
            });
        }
    };
    if !(1..=max_day).contains(&day) {
        return Err(Error::InvalidRequest {
            field,
            reason: "day is not valid for the supplied month and year",
        });
    }
    Ok(())
}

pub(super) fn ordered_dates(start: &TradeDate, end: &TradeDate) -> Result<(), Error> {
    trade_date(start, "startDate")?;
    trade_date(end, "expirationDate")?;
    if date_key(start) > date_key(end) {
        return Err(Error::InvalidRequest {
            field: "expirationDate",
            reason: "must not precede startDate",
        });
    }
    Ok(())
}

pub(super) fn same_date(left: &TradeDate, right: &TradeDate) -> bool {
    date_key(left) == date_key(right)
}

fn date_key(value: &TradeDate) -> (i64, i64, i64) {
    (*value.year(), *value.month(), *value.day())
}

fn is_leap_year(year: i64) -> bool {
    (year % 4 == 0 && year % 100 != 0) || year % 400 == 0
}
