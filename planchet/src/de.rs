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

#[cfg(test)]
mod tests {
    use super::*;
    use serde::Deserialize;
    use serde_json::from_str;

    #[test]
    fn test_de_from_str_or_int() {
        #[derive(Deserialize, Debug, PartialEq)]
        struct TestStructI32 {
            #[serde(deserialize_with = "de_from_str_or_int")]
            val: i32,
        }

        // Test i32
        let json = r#"{"val": 15}"#;
        let res: TestStructI32 = from_str(json).unwrap();
        assert_eq!(res.val, 15);

        let json = r#"{"val": "51"}"#;
        let res: TestStructI32 = from_str(json).unwrap();
        assert_eq!(res.val, 51);

        let json = r#"{"val": "text"}"#;
        let res = from_str::<TestStructI32>(json);
        assert!(res.is_err(), "Expected error for invalid string, got {:?}", res);

        let json = r#"{"val": null}"#;
        let res = from_str::<TestStructI32>(json);
        assert!(res.is_err(), "Expected error for null, got {:?}", res);

        #[derive(Deserialize, Debug, PartialEq)]
        struct TestStructI64 {
            #[serde(deserialize_with = "de_from_str_or_int")]
            val: i64,
        }

        // Test i64
        let json = r#"{"val": 15}"#;
        let res: TestStructI64 = from_str(json).unwrap();
        assert_eq!(res.val, 15);

        let json = r#"{"val": "51"}"#;
        let res: TestStructI64 = from_str(json).unwrap();
        assert_eq!(res.val, 51);
    }

    #[test]
    fn test_de_optional_from_str_or_int() {
        #[derive(Deserialize, Debug, PartialEq)]
        struct TestStructOptionalI32 {
            #[serde(deserialize_with = "de_optional_from_str_or_int")]
            #[serde(default)] // Important to test null handling vs missing key if needed, but here we test explicit values
            val: Option<i32>,
        }

        // Test i32
        let json = r#"{"val": 15}"#;
        let res: TestStructOptionalI32 = from_str(json).unwrap();
        assert_eq!(res.val, Some(15));

        let json = r#"{"val": "51"}"#;
        let res: TestStructOptionalI32 = from_str(json).unwrap();
        assert_eq!(res.val, Some(51));

        let json = r#"{"val": null}"#;
        let res: TestStructOptionalI32 = from_str(json).unwrap();
        assert_eq!(res.val, None);

        // Missing field (should be None due to Option or Default)
        let json = r#"{}"#;
        let res: TestStructOptionalI32 = from_str(json).unwrap();
        assert_eq!(res.val, None);

        let json = r#"{"val": "text"}"#;
        let res = from_str::<TestStructOptionalI32>(json);
        assert!(res.is_err(), "Expected error for invalid string, got {:?}", res);

        #[derive(Deserialize, Debug, PartialEq)]
        struct TestStructOptionalI64 {
            #[serde(deserialize_with = "de_optional_from_str_or_int")]
            #[serde(default)]
            val: Option<i64>,
        }

        // Test i64
        let json = r#"{"val": 15}"#;
        let res: TestStructOptionalI64 = from_str(json).unwrap();
        assert_eq!(res.val, Some(15));

        let json = r#"{"val": "51"}"#;
        let res: TestStructOptionalI64 = from_str(json).unwrap();
        assert_eq!(res.val, Some(51));
    }
}
