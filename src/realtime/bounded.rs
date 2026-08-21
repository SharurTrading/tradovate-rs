// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: LicenseRef-Proprietary

//! Private bounded Serde containers for untrusted realtime payloads.

use std::{collections::BTreeMap, fmt, marker::PhantomData};

use serde::{
    Deserialize,
    de::{DeserializeOwned, Error as _, IgnoredAny, MapAccess, SeqAccess, Visitor},
};

const LIMIT_ERROR_MARKER: &str = "tradovate realtime collection limit exceeded";

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum DecodeError {
    LimitExceeded,
    Malformed,
}

pub(super) fn from_str<T>(payload: &str) -> Result<T, DecodeError>
where
    T: DeserializeOwned,
{
    let mut deserializer = serde_json::Deserializer::from_str(payload);
    let value = T::deserialize(&mut deserializer).map_err(|error| classify(&error))?;
    deserializer.end().map_err(|error| classify(&error))?;
    Ok(value)
}

pub(super) fn one_or_many<T, const LIMIT: usize>(payload: &str) -> Result<Vec<T>, DecodeError>
where
    T: DeserializeOwned,
{
    if payload.trim_start().starts_with('[') {
        from_str::<BoundedVec<T, LIMIT>>(payload).map(BoundedVec::into_vec)
    } else {
        from_str(payload).map(|value| vec![value])
    }
}

pub(super) fn count_one_or_many<const LIMIT: usize>(payload: &str) -> Result<usize, DecodeError> {
    if payload.trim_start().starts_with('[') {
        from_str::<BoundedVec<IgnoredObject, LIMIT>>(payload).map(|values| values.len())
    } else {
        from_str::<IgnoredObject>(payload).map(|_| 1)
    }
}

struct IgnoredObject;

impl<'de> Deserialize<'de> for IgnoredObject {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_map(IgnoredObjectVisitor)
    }
}

struct IgnoredObjectVisitor;

impl<'de> Visitor<'de> for IgnoredObjectVisitor {
    type Value = IgnoredObject;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("a user-stream entity object")
    }

    fn visit_map<A>(self, mut entries: A) -> Result<Self::Value, A::Error>
    where
        A: MapAccess<'de>,
    {
        while entries.next_entry::<IgnoredAny, IgnoredAny>()?.is_some() {}
        Ok(IgnoredObject)
    }
}

fn classify(error: &serde_json::Error) -> DecodeError {
    if error.to_string().contains(LIMIT_ERROR_MARKER) {
        DecodeError::LimitExceeded
    } else {
        DecodeError::Malformed
    }
}

#[derive(Debug)]
pub(super) struct BoundedVec<T, const LIMIT: usize>(Vec<T>);

impl<T, const LIMIT: usize> BoundedVec<T, LIMIT> {
    pub(super) fn into_vec(self) -> Vec<T> {
        self.0
    }

    pub(super) const fn len(&self) -> usize {
        self.0.len()
    }

    pub(super) fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl<T, const LIMIT: usize> Default for BoundedVec<T, LIMIT> {
    fn default() -> Self {
        Self(Vec::new())
    }
}

impl<'de, T, const LIMIT: usize> Deserialize<'de> for BoundedVec<T, LIMIT>
where
    T: Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_seq(BoundedVecVisitor::<T, LIMIT>(PhantomData))
    }
}

struct BoundedVecVisitor<T, const LIMIT: usize>(PhantomData<T>);

impl<'de, T, const LIMIT: usize> Visitor<'de> for BoundedVecVisitor<T, LIMIT>
where
    T: Deserialize<'de>,
{
    type Value = BoundedVec<T, LIMIT>;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("an array within its configured realtime limit")
    }

    fn visit_seq<A>(self, mut sequence: A) -> Result<Self::Value, A::Error>
    where
        A: SeqAccess<'de>,
    {
        let capacity = sequence.size_hint().unwrap_or(0).min(LIMIT);
        let mut values = Vec::with_capacity(capacity);
        while values.len() < LIMIT {
            let Some(value) = sequence.next_element::<T>()? else {
                return Ok(BoundedVec(values));
            };
            values.push(value);
        }
        if sequence.next_element::<IgnoredAny>()?.is_some() {
            return Err(A::Error::custom(LIMIT_ERROR_MARKER));
        }
        Ok(BoundedVec(values))
    }
}

#[derive(Debug)]
pub(super) struct BoundedMap<K, V, const LIMIT: usize>(BTreeMap<K, V>);

impl<K, V, const LIMIT: usize> BoundedMap<K, V, LIMIT> {
    pub(super) fn into_map(self) -> BTreeMap<K, V> {
        self.0
    }

    pub(super) fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub(super) const fn as_map(&self) -> &BTreeMap<K, V> {
        &self.0
    }
}

impl<K, V, const LIMIT: usize> Default for BoundedMap<K, V, LIMIT> {
    fn default() -> Self {
        Self(BTreeMap::new())
    }
}

impl<'de, K, V, const LIMIT: usize> Deserialize<'de> for BoundedMap<K, V, LIMIT>
where
    K: Deserialize<'de> + Ord,
    V: Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_map(BoundedMapVisitor::<K, V, LIMIT>(PhantomData))
    }
}

struct BoundedMapVisitor<K, V, const LIMIT: usize>(PhantomData<(K, V)>);

impl<'de, K, V, const LIMIT: usize> Visitor<'de> for BoundedMapVisitor<K, V, LIMIT>
where
    K: Deserialize<'de> + Ord,
    V: Deserialize<'de>,
{
    type Value = BoundedMap<K, V, LIMIT>;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("an object within its configured realtime limit")
    }

    fn visit_map<A>(self, mut entries: A) -> Result<Self::Value, A::Error>
    where
        A: MapAccess<'de>,
    {
        let mut values = BTreeMap::new();
        let mut count = 0_usize;
        while count < LIMIT {
            let Some((key, value)) = entries.next_entry::<K, V>()? else {
                return Ok(BoundedMap(values));
            };
            values.insert(key, value);
            count += 1;
        }
        if entries.next_key::<IgnoredAny>()?.is_some() {
            return Err(A::Error::custom(LIMIT_ERROR_MARKER));
        }
        Ok(BoundedMap(values))
    }
}

#[cfg(test)]
mod tests {
    use std::sync::atomic::{AtomicUsize, Ordering};

    use super::*;

    static ARRAY_MATERIALIZED: AtomicUsize = AtomicUsize::new(0);
    static MAP_MATERIALIZED: AtomicUsize = AtomicUsize::new(0);

    struct ArrayCounted;

    impl<'de> Deserialize<'de> for ArrayCounted {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            IgnoredAny::deserialize(deserializer)?;
            ARRAY_MATERIALIZED.fetch_add(1, Ordering::Relaxed);
            Ok(Self)
        }
    }

    struct MapCounted;

    impl<'de> Deserialize<'de> for MapCounted {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            IgnoredAny::deserialize(deserializer)?;
            MAP_MATERIALIZED.fetch_add(1, Ordering::Relaxed);
            Ok(Self)
        }
    }

    #[test]
    fn oversized_array_does_not_materialize_the_overflow_element() {
        ARRAY_MATERIALIZED.store(0, Ordering::Relaxed);
        let result = from_str::<BoundedVec<ArrayCounted, 2>>("[0,1,2,3]");
        assert!(matches!(result, Err(DecodeError::LimitExceeded)));
        assert_eq!(ARRAY_MATERIALIZED.load(Ordering::Relaxed), 2);
    }

    #[test]
    fn oversized_map_does_not_materialize_the_overflow_value() {
        MAP_MATERIALIZED.store(0, Ordering::Relaxed);
        let result = from_str::<BoundedMap<String, MapCounted, 2>>(r#"{"a":0,"b":1,"c":2}"#);
        assert!(matches!(result, Err(DecodeError::LimitExceeded)));
        assert_eq!(MAP_MATERIALIZED.load(Ordering::Relaxed), 2);
    }

    #[test]
    fn malformed_input_is_distinct_from_a_collection_limit() {
        let result = from_str::<BoundedVec<IgnoredAny, 2>>("[0,");
        assert!(matches!(result, Err(DecodeError::Malformed)));
    }

    #[test]
    fn unknown_entity_count_preserves_the_object_shape_contract() {
        assert!(matches!(count_one_or_many::<2>(r#"[{"a":1},{}]"#), Ok(2)));
        assert!(matches!(
            count_one_or_many::<2>("[{},0]"),
            Err(DecodeError::Malformed)
        ));
    }
}
