use serde_json::Value;

pub use crate::analysis::{self, analyze_document, data, header};
pub use crate::read;

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

    let document = analyze_document(qbjs).map_err(DeserializeError::AnalysisError)?;

    match document {
        data::Value::Array(_) | data::Value::Object(_) => {
            read::read_value(qbjs, &document).map_err(DeserializeError::ReadError)
        }
        _ => Err(DeserializeError::InvalidRootContainer),
    }
}
