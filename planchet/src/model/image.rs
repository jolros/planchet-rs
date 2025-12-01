use base64::prelude::*;
use serde::{Deserialize, Serialize};
use std::io;
use std::path::Path;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Base64Image(pub String);

impl From<String> for Base64Image {
    fn from(s: String) -> Self {
        Base64Image(s)
    }
}

impl From<Base64Image> for String {
    fn from(val: Base64Image) -> Self {
        val.0
    }
}

pub trait ImageSource {
    fn to_base64(&self) -> io::Result<Base64Image>;
}

impl ImageSource for [u8] {
    fn to_base64(&self) -> io::Result<Base64Image> {
        let b64 = BASE64_STANDARD.encode(self);
        Ok(Base64Image(b64))
    }
}

impl<const N: usize> ImageSource for [u8; N] {
    fn to_base64(&self) -> io::Result<Base64Image> {
        let b64 = BASE64_STANDARD.encode(self);
        Ok(Base64Image(b64))
    }
}

impl ImageSource for Vec<u8> {
    fn to_base64(&self) -> io::Result<Base64Image> {
        let b64 = BASE64_STANDARD.encode(self);
        Ok(Base64Image(b64))
    }
}

impl ImageSource for str {
    fn to_base64(&self) -> io::Result<Base64Image> {
        let b64 = BASE64_STANDARD.encode(self);
        Ok(Base64Image(b64))
    }
}

impl ImageSource for String {
    fn to_base64(&self) -> io::Result<Base64Image> {
        let b64 = BASE64_STANDARD.encode(self);
        Ok(Base64Image(b64))
    }
}

impl ImageSource for Path {
    fn to_base64(&self) -> io::Result<Base64Image> {
        let bytes = std::fs::read(self)?;
        let b64 = BASE64_STANDARD.encode(bytes);
        Ok(Base64Image(b64))
    }
}
