use serde_json::Value;

use crate::analysis::{analyze_document, data, header, Error};
use crate::error::DecodeError;
use crate::read;

pub fn decode_to_json(qbjs: &[u8]) -> Result<Value, DecodeError> {
    if qbjs.is_empty() {
        return Ok(serde_json::json!({}));
    }

    if qbjs.len() < header::HEADER_LENGTH {
        return Err(DecodeError::InsufficientData);
    }

    match analyze_document(&qbjs) {
        Ok(value) => match value {
            data::Value::Array(_) | data::Value::Object(_) => Ok(read::read_value(&qbjs, &value)),
            _ => Err(DecodeError::InvalidRootContainer),
        },
        Err(e) => match e {
            Error::Header(header_error) => match header_error {
                header::Error::InvalidLength => Err(DecodeError::InsufficientData),
                header::Error::InvalidTag => Err(DecodeError::MalformedHeader),
                header::Error::InvalidVersion => Err(DecodeError::MalformedHeader),
            },
        },
    }
}
