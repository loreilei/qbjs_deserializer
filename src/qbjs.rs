use serde_json::Value;

use crate::analysis::{self, analyze_document, data, header};
use crate::read;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum DeserializeError {
    AnalysisError(analysis::AnalysisError),
    InsufficientData,
    InvalidRootContainer,
    ReadError(read::ReadError),
}

pub fn deserialize_to_json(qbjs: &[u8]) -> Result<Value, DeserializeError> {
    if qbjs.is_empty() {
        return Ok(serde_json::json!({}));
    }

    if qbjs.len() < header::HEADER_LENGTH {
        return Err(DeserializeError::InsufficientData);
    }

    match analyze_document(&qbjs) {
        Ok(value) => match value {
            data::Value::Array(_) | data::Value::Object(_) => match read::read_value(&qbjs, &value)
            {
                Ok(value) => Ok(value),
                Err(e) => Err(DeserializeError::ReadError(e)),
            },
            _ => Err(DeserializeError::InvalidRootContainer),
        },
        Err(e) => Err(DeserializeError::AnalysisError(e)),
    }
}
