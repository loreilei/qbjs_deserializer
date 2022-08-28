#[derive(Debug, Clone, Eq, PartialEq)]
pub enum DecodeError {
    InsufficientData,
    MalformedHeader,
    MalformedLatin1String,
    MalformedUtf16String,
    UnknownValueType,
    InvalidBoolValue,
    InvalidDoubleValue,
    InvalidObject,
    InvalidRootContainer,
}
