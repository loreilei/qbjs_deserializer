use encoding::all::{ISO_8859_1, UTF_16LE};
use encoding::{DecoderTrap, Encoding};

use serde_json::Value;

use crate::analysis::{data, metadata};
use crate::type_conversions::{as_i27, as_u32, as_u64};

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum ReadError {
    InvalidBoolDataPosition,
    InvalidSelfContainedNumberDataPosition,
    InvalidNumberDataRange,
    InvalidLatin1StringDataRange,
    InvalidUtf16StringDataRange,
    FailedToDecodeLatin1String,
    FailedToDecodeUtf16String,
}

fn read_latin1_string(data: &[u8], bytefield: &data::ByteField) -> Result<String, ReadError> {
    match data.get(bytefield.range.start..bytefield.range.end) {
        Some(data) => match ISO_8859_1.decode(data, DecoderTrap::Strict) {
            Ok(s) => Ok(s),
            Err(_) => Err(ReadError::FailedToDecodeLatin1String),
        },
        None => Err(ReadError::InvalidLatin1StringDataRange),
    }
}

fn read_utf16_string(data: &[u8], bytefield: &data::ByteField) -> Result<String, ReadError> {
    match data.get(bytefield.range.start..bytefield.range.end) {
        Some(data) => match UTF_16LE.decode(data, DecoderTrap::Strict) {
            Ok(s) => Ok(s),
            Err(_) => Err(ReadError::FailedToDecodeUtf16String),
        },
        None => Err(ReadError::InvalidUtf16StringDataRange),
    }
}

fn read_key(data: &[u8], key: &data::Key) -> Result<String, ReadError> {
    match key {
        data::Key::Latin1String(bytefield) => read_latin1_string(data, bytefield),
        data::Key::Utf16String(bytefield) => read_utf16_string(data, bytefield),
    }
}

pub fn read_value(data: &[u8], value: &data::Value) -> Result<Value, ReadError> {
    match value {
        data::Value::Null(_) => Ok(Value::Null),
        data::Value::Bool(position) => read_bool(data, *position),
        data::Value::SelfContainedNumber(position) => read_self_contained_number(data, *position),
        data::Value::Number(bytefield) => read_number(data, bytefield),
        data::Value::Latin1String(bytefield) => read_latin1_string_value(data, bytefield),
        data::Value::Utf16String(bytefield) => read_utf16_string_value(data, bytefield),
        data::Value::Array(array) => read_array(data, array),
        data::Value::Object(object) => read_object(data, object),
    }
}

fn read_bool(data: &[u8], position: usize) -> Result<Value, ReadError> {
    match data.get(position) {
        Some(data) => Ok(Value::Bool((*data & 0b100000) != 0)),
        None => Err(ReadError::InvalidBoolDataPosition),
    }
}

fn read_self_contained_number(data: &[u8], position: usize) -> Result<Value, ReadError> {
    match data.get(position..(position + metadata::VALUE_HEADER_BYTE_SIZE)) {
        Some(data) => Ok(Value::Number(serde_json::Number::from(as_i27(as_u32(
            data,
        ))))),
        None => Err(ReadError::InvalidSelfContainedNumberDataPosition),
    }
}

fn read_number(data: &[u8], bytefield: &data::ByteField) -> Result<Value, ReadError> {
    match data.get(bytefield.range.start..bytefield.range.end) {
        Some(data) => {
            let value = f64::from_bits(as_u64(data));
            Ok(Value::Number(serde_json::Number::from_f64(value).unwrap()))
        }
        None => Err(ReadError::InvalidNumberDataRange),
    }
}

fn read_latin1_string_value(data: &[u8], bytefield: &data::ByteField) -> Result<Value, ReadError> {
    match read_latin1_string(data, bytefield) {
        Ok(latin1_string) => Ok(Value::String(latin1_string)),
        Err(e) => Err(e),
    }
}

fn read_utf16_string_value(data: &[u8], bytefield: &data::ByteField) -> Result<Value, ReadError> {
    match read_utf16_string(data, bytefield) {
        Ok(utf16_string) => Ok(Value::String(utf16_string)),
        Err(e) => Err(e),
    }
}

fn read_array(data: &[u8], array: &data::Array) -> Result<Value, ReadError> {
    match array
        .values
        .iter()
        .map(|value| read_value(data, value))
        .collect()
    {
        Ok(values) => Ok(Value::Array(values)),
        Err(e) => Err(e),
    }
}

fn read_object(data: &[u8], object: &data::Object) -> Result<Value, ReadError> {
    match object
        .entries
        .iter()
        .map(|entry| {
            let key = read_key(data, &entry.key);
            let value = read_value(data, &entry.value);
            match (key, value) {
                (Ok(key), Ok(value)) => Ok((key, value)),
                (Err(key_error), _) => Err(key_error),
                (_, Err(value_error)) => Err(value_error),
            }
        })
        .collect()
    {
        Ok(entries) => Ok(Value::Object(entries)),
        Err(e) => Err(e),
    }
}
