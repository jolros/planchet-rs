//! Deserialization helpers.
use serde::{Deserialize, Deserializer};
use std::fmt::Display;
use std::str::FromStr;

pub fn de_from_str_or_int<'de, D, T>(deserializer: D) -> Result<T, D::Error>
where
    D: Deserializer<'de>,
    T: FromStr + Deserialize<'de>,
    <T as FromStr>::Err: Display,
{
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum StringOrInt<T> {
        String(String),
        Int(T),
    }

    match StringOrInt::<T>::deserialize(deserializer)? {
        StringOrInt::String(s) => s.parse::<T>().map_err(serde::de::Error::custom),
        StringOrInt::Int(i) => Ok(i),
    }
}

pub fn de_optional_from_str_or_int<'de, D, T>(deserializer: D) -> Result<Option<T>, D::Error>
where
    D: Deserializer<'de>,
    T: FromStr + Deserialize<'de>,
    <T as FromStr>::Err: Display,
{
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum StringOrInt<T> {
        String(String),
        Int(T),
    }

    match Option::<StringOrInt<T>>::deserialize(deserializer)? {
        Some(StringOrInt::String(s)) => s.parse::<T>().map(Some).map_err(serde::de::Error::custom),
        Some(StringOrInt::Int(i)) => Ok(Some(i)),
        None => Ok(None),
    }
}
