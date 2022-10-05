use crate::type_conversions::as_u32;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum AnalysisError {
    HeaderAnalysisError(header::Error),
    MetadataAnalysisError(metadata::Error),
    DataAnalysisError(data::Error),
}
pub mod data {
    use std::ops::Range;

    #[derive(Debug, Clone, Eq, PartialEq)]
    pub enum Error {
        UnknownQtValue,
        InvalidArrayContainer, // Means we tried to deserialize and array but the container base's object flag is set
        InvalidObjectContainer, // Means we tried to deserialize and array but the container base's object flag isn't set
        InvalidValueLength,
    }

    #[derive(Debug)]
    pub struct ByteField {
        pub range: Range<usize>,
    }

    #[derive(Debug)]
    pub struct Array {
        pub values: Vec<Value>,
    }

    #[derive(Debug)]
    pub enum Key {
        Latin1String(ByteField),
        Utf16String(ByteField),
    }

    #[derive(Debug)]
    pub struct Entry {
        pub key: Key,
        pub value: Value,
    }

    #[derive(Debug)]
    pub struct Object {
        pub entries: Vec<Entry>,
    }

    #[derive(Debug)]
    pub enum Value {
        Null(usize),
        Bool(usize),
        SelfContainedNumber(usize),
        Number(ByteField),
        Latin1String(ByteField),
        Utf16String(ByteField),
        Array(Array),
        Object(Object),
    }
}

pub mod header {
    use crate::type_conversions::as_u32;
    use std::ops::Range;

    #[derive(Debug, Clone, Eq, PartialEq)]
    pub enum Error {
        InvalidLength,
        InvalidTag,
        InvalidVersion,
    }

    pub const HEADER_LENGTH: usize = 8;
    const TAG_RANGE: Range<usize> = 0..4;
    const VERSION_RANGE: Range<usize> = 4..8;
    const VALID_TAG: &str = "qbjs";
    const VALID_VERSION: u32 = 1;

    pub struct QbjsHeader {
        pub tag: String,
        pub version: u32, // Must be one, so far only little endian supported
    }

    impl QbjsHeader {
        fn new(tag: String, version: u32) -> Self {
            QbjsHeader { tag, version }
        }

        pub fn from_data(data: &[u8]) -> Result<Self, Error> {
            let tag = match data.get(TAG_RANGE) {
                Some(data) => data.iter().map(|d| *d as char).collect::<String>(),
                None => {
                    return Err(Error::InvalidLength);
                }
            };

            let version = match data.get(VERSION_RANGE) {
                Some(data) => as_u32(data),
                None => {
                    return Err(Error::InvalidLength);
                }
            };

            if tag != VALID_TAG {
                return Err(Error::InvalidTag); // Assuming we're dealing with little endian documents
            }

            if version != VALID_VERSION {
                return Err(Error::InvalidVersion); // Assuming we're dealing with little endian documents
            }

            Ok(QbjsHeader::new(tag, version))
        }
    }
}

pub mod metadata {
    use crate::type_conversions::{as_u27, as_u32};
    use std::ops::Range;

    #[derive(Debug, Clone, Eq, PartialEq)]
    pub enum Error {
        InvalidContainerBaseLength,
        InvalidValueHeaderSize,
    }

    pub const CONTAINER_BASE_LENGTH: usize = 12;
    const SIZE_FIELD_RANGE: Range<usize> = 0..4;
    const OBJECT_FLAG_AND_LENGTH_RANGE: Range<usize> = 4..8;
    const TABLE_OFFSET_RANGE: Range<usize> = 8..12;
    #[derive(Debug)]
    pub struct ContainerBase {
        pub size: u32,
        pub is_object: bool,
        pub length: u32,
        pub table_offset: u32,
    }

    impl ContainerBase {
        pub fn from_data(data: &[u8]) -> Result<Self, Error> {
            let size = match data.get(SIZE_FIELD_RANGE) {
                Some(data) => as_u32(data),
                None => return Err(Error::InvalidContainerBaseLength),
            };

            let object_flag_and_length = match data.get(OBJECT_FLAG_AND_LENGTH_RANGE) {
                Some(data) => as_u32(data),
                None => return Err(Error::InvalidContainerBaseLength),
            };

            let is_object = (object_flag_and_length & 0b1) != 0;
            let length = (object_flag_and_length & !0b1_u32) >> 1;

            let table_offset = match data.get(TABLE_OFFSET_RANGE) {
                Some(data) => as_u32(data),
                None => return Err(Error::InvalidContainerBaseLength),
            };

            Ok(ContainerBase {
                size,
                is_object,
                length,
                table_offset,
            })
        }
    }

    #[derive(Debug)]
    pub struct ValueHeader {
        pub qt_value_type: u8,
        pub latin_or_int_value_flag: bool,
        pub latin_key_flag: bool,
        pub value_bit_field: u32,
        pub position: usize,
    }

    pub const VALUE_HEADER_BYTE_SIZE: usize = 4;
    const QT_VALUE_TYPE_MASK: u8 = 0b111;
    const LATIN_OR_INT_VALUE_FLAG_MASK: u8 = 0b1 << 3;
    const LATIN_KEY_FLAG_MASK: u8 = 0b1 << 4;
    const VALUE_BIT_FIELD_RANGE: Range<usize> = 0..4;

    impl ValueHeader {
        pub fn from_data(data: &[u8], position: usize) -> Result<Self, Error> {
            let header = match data.first() {
                Some(header) => header,
                None => {
                    return Err(Error::InvalidValueHeaderSize);
                }
            };
            let qt_value_type = header & QT_VALUE_TYPE_MASK;
            let latin_or_int_value_flag = (header & LATIN_OR_INT_VALUE_FLAG_MASK) != 0;
            let latin_key_flag = (header & LATIN_KEY_FLAG_MASK) != 0;
            let value_bit_field = match data.get(VALUE_BIT_FIELD_RANGE) {
                Some(data) => as_u27(as_u32(data)),
                None => {
                    return Err(Error::InvalidValueHeaderSize);
                }
            };

            Ok(ValueHeader {
                qt_value_type,
                latin_or_int_value_flag,
                latin_key_flag,
                value_bit_field,
                position,
            })
        }
    }

    pub const LATIN1_SIZE_FIELD_LENGTH: usize = 2;
    pub const LATIN1_CHAR_LENGTH: usize = 1;
    pub const UTF16_SIZE_FIELD_LENGTH: usize = 4;
    pub const UTF16_CHAR_LENGTH: usize = 2;
}

pub fn analyze_document(data: &[u8]) -> Result<data::Value, AnalysisError> {
    let header = match data.get(0..header::HEADER_LENGTH) {
        Some(data) => header::QbjsHeader::from_data(data),
        None => Err(header::Error::InvalidLength),
    };

    match header {
        Ok(_) => {}
        Err(e) => {
            return Err(AnalysisError::HeaderAnalysisError(e));
        }
    }

    let container_base_range =
        header::HEADER_LENGTH..(header::HEADER_LENGTH + metadata::CONTAINER_BASE_LENGTH);
    let container_base = match data.get(container_base_range) {
        Some(data) => metadata::ContainerBase::from_data(data),
        None => Err(metadata::Error::InvalidContainerBaseLength),
    };

    match container_base {
        Ok(container_base) => {
            if container_base.is_object {
                match analyze_object(data, header::HEADER_LENGTH) {
                    Ok(analyzed_object) => Ok(analyzed_object.0),
                    Err(e) => Err(e),
                }
            } else {
                match analyze_array(data, header::HEADER_LENGTH) {
                    Ok(analyzed_array) => Ok(analyzed_array.0),
                    Err(e) => Err(e),
                }
            }
        }
        Err(e) => Err(AnalysisError::MetadataAnalysisError(e)),
    }
}

fn analyze_array(data: &[u8], base_start: usize) -> Result<(data::Value, usize), AnalysisError> {
    let base_end = base_start + metadata::CONTAINER_BASE_LENGTH;
    let base_range = base_start..base_end;

    let array_info = match data.get(base_range) {
        Some(data) => metadata::ContainerBase::from_data(data),
        None => Err(metadata::Error::InvalidContainerBaseLength),
    };

    match array_info {
        Ok(array_info) => {
            if array_info.is_object {
                return Err(AnalysisError::DataAnalysisError(
                    data::Error::InvalidArrayContainer,
                ));
            }

            let nb_values = array_info.length as usize;

            let mut values = Vec::<data::Value>::new();
            values.reserve_exact(nb_values);

            let mut offset = base_start + array_info.table_offset as usize;
            for _i in 0..nb_values {
                let header_start = offset;
                let header_end = header_start + metadata::VALUE_HEADER_BYTE_SIZE;

                let header = match data.get(header_start..header_end) {
                    Some(data) => metadata::ValueHeader::from_data(data, header_start),
                    None => {
                        return Err(AnalysisError::MetadataAnalysisError(
                            metadata::Error::InvalidValueHeaderSize,
                        ));
                    }
                };

                match header {
                    Ok(header) => match analyze_value(data, &header, base_start) {
                        Ok((value, _)) => {
                            values.push(value);

                            offset = header_end;
                        }
                        Err(e) => {
                            return Err(e);
                        }
                    },
                    Err(e) => {
                        return Err(AnalysisError::MetadataAnalysisError(e));
                    }
                }
            }

            Ok((
                data::Value::Array(data::Array { values }),
                base_start + array_info.size as usize,
            ))
        }
        Err(e) => Err(AnalysisError::MetadataAnalysisError(e)),
    }
}

fn analyze_object(data: &[u8], base_start: usize) -> Result<(data::Value, usize), AnalysisError> {
    let base_end = base_start + metadata::CONTAINER_BASE_LENGTH;
    let base_range = base_start..base_end;

    let object_info = match data.get(base_range) {
        Some(data) => metadata::ContainerBase::from_data(data),
        None => Err(metadata::Error::InvalidContainerBaseLength),
    };

    match object_info {
        Ok(object_info) => {
            if !object_info.is_object {
                return Err(AnalysisError::DataAnalysisError(
                    data::Error::InvalidObjectContainer,
                ));
            }

            let nb_entries = object_info.length as usize;

            let mut entries = Vec::<data::Entry>::new();
            entries.reserve_exact(nb_entries);

            let mut offset = base_end;
            for _i in 0..nb_entries {
                match analyze_entry(data, offset, base_start) {
                    Ok((entry, entry_end)) => {
                        entries.push(entry);

                        offset = entry_end;
                    }
                    Err(e) => {
                        return Err(e);
                    }
                }
            }

            Ok((
                data::Value::Object(data::Object { entries }),
                base_start + object_info.size as usize,
            ))
        }
        Err(e) => Err(AnalysisError::MetadataAnalysisError(e)),
    }
}

fn analyze_entry(
    data: &[u8],
    entry_start: usize,
    object_start: usize,
) -> Result<(data::Entry, usize), AnalysisError> {
    let header_end = entry_start + metadata::VALUE_HEADER_BYTE_SIZE;
    let header_range = entry_start..header_end;

    let header_data = match data.get(header_range) {
        Some(data) => data,
        None => {
            return Err(AnalysisError::MetadataAnalysisError(
                metadata::Error::InvalidValueHeaderSize,
            ));
        }
    };

    match metadata::ValueHeader::from_data(header_data, entry_start) {
        Ok(header) => {
            let key_analysis_result = if header.latin_key_flag {
                analyze_latin1_key(data, header_end)
            } else {
                analyze_utf16_key(data, header_end)
            };

            let (key, key_end) = match key_analysis_result {
                Ok(key_info) => key_info,
                Err(e) => {
                    return Err(e);
                }
            };

            match analyze_value(data, &header, object_start) {
                Ok((value, value_end)) => {
                    let entry_end = match value {
                        data::Value::Null(_)
                        | data::Value::Bool(_)
                        | data::Value::SelfContainedNumber(_) => key_end,
                        _ => value_end,
                    };

                    Ok((data::Entry { key, value }, entry_end))
                }
                Err(e) => Err(e),
            }
        }
        Err(e) => Err(AnalysisError::MetadataAnalysisError(e)),
    }
}

fn analyze_latin1_string(data: &[u8], string_field_start: usize) -> (data::ByteField, usize) {
    let string_field_length = as_u32(data) as usize;
    let string_data_start = string_field_start + metadata::LATIN1_SIZE_FIELD_LENGTH;
    let string_data_end = string_data_start + string_field_length * metadata::LATIN1_CHAR_LENGTH;

    // Strings are filled with 0 to be aligned to 4 bytes
    let zero_alignment = match string_data_end % 4 {
        0 => 0,
        n => 4 - n,
    };
    let aligned_string_data_end = string_data_end + zero_alignment;
    (
        data::ByteField {
            range: string_data_start..string_data_end,
        },
        aligned_string_data_end,
    )
}

fn analyze_utf16_string(data: &[u8], string_field_start: usize) -> (data::ByteField, usize) {
    let string_field_length = as_u32(data) as usize;
    let string_data_start = string_field_start + metadata::UTF16_SIZE_FIELD_LENGTH;
    let string_data_end = string_data_start + string_field_length * metadata::UTF16_CHAR_LENGTH;

    // Strings are filled with 0 to be aligned to 4 bytes
    let zero_alignment = match string_data_end % 4 {
        0 => 0,
        n => 4 - n,
    };
    let aligned_string_data_end = string_data_end + zero_alignment;
    (
        data::ByteField {
            range: string_data_start..string_data_end,
        },
        aligned_string_data_end,
    )
}

fn analyze_latin1_key(data: &[u8], key_start: usize) -> Result<(data::Key, usize), AnalysisError> {
    let size_field_range = key_start..(key_start + metadata::LATIN1_SIZE_FIELD_LENGTH);
    match data.get(size_field_range) {
        Some(data) => {
            let (bytefield, key_end) = analyze_latin1_string(data, key_start);
            Ok((data::Key::Latin1String(bytefield), key_end))
        }
        None => Err(AnalysisError::DataAnalysisError(
            data::Error::InvalidValueLength,
        )),
    }
}

fn analyze_utf16_key(data: &[u8], key_start: usize) -> Result<(data::Key, usize), AnalysisError> {
    let size_field_range = key_start..(key_start + metadata::UTF16_SIZE_FIELD_LENGTH);
    match data.get(size_field_range) {
        Some(data) => {
            let (bytefield, key_end) = analyze_utf16_string(data, key_start);
            Ok((data::Key::Utf16String(bytefield), key_end))
        }
        None => Err(AnalysisError::DataAnalysisError(
            data::Error::InvalidValueLength,
        )),
    }
}

const QT_NULL_VALUE: u8 = 0;
const QT_BOOL_VALUE: u8 = 1;
const QT_NUMBER_VALUE: u8 = 2;
const QT_STRING_VALUE: u8 = 3;
const QT_ARRAY_VALUE: u8 = 4;
const QT_OBJECT_VALUE: u8 = 5;

fn analyze_value(
    data: &[u8],
    header: &metadata::ValueHeader,
    container_start: usize,
) -> Result<(data::Value, usize), AnalysisError> {
    let header_end = header.position + metadata::VALUE_HEADER_BYTE_SIZE;
    match header.qt_value_type {
        QT_NULL_VALUE => Ok((data::Value::Null(header.position), header_end)),
        QT_BOOL_VALUE => Ok((data::Value::Bool(header.position), header_end)),
        QT_NUMBER_VALUE => {
            if header.latin_or_int_value_flag {
                Ok((
                    data::Value::SelfContainedNumber(header.position),
                    header_end,
                ))
            } else {
                Ok(analyze_double_value(header, container_start))
            }
        }
        QT_STRING_VALUE => {
            let value_range_start = container_start + header.value_bit_field as usize;
            if header.latin_or_int_value_flag {
                analyze_latin1_string_value(data, value_range_start)
            } else {
                analyze_utf16_string_value(data, value_range_start)
            }
        }
        QT_ARRAY_VALUE => {
            let value_range_start = container_start + header.value_bit_field as usize;
            analyze_array(data, value_range_start)
        }
        QT_OBJECT_VALUE => {
            let value_range_start = container_start + header.value_bit_field as usize;
            analyze_object(data, value_range_start)
        }
        _ => Err(AnalysisError::DataAnalysisError(
            data::Error::UnknownQtValue,
        )),
    }
}

const DOUBLE_VALUE_BYTE_SIZE: usize = 8;

fn analyze_double_value(
    header: &metadata::ValueHeader,
    container_start: usize,
) -> (data::Value, usize) {
    let data_start = container_start + header.value_bit_field as usize;
    let data_end = data_start + DOUBLE_VALUE_BYTE_SIZE;
    (
        data::Value::Number(data::ByteField {
            range: data_start..data_end,
        }),
        data_end,
    )
}

fn analyze_latin1_string_value(
    data: &[u8],
    value_start: usize,
) -> Result<(data::Value, usize), AnalysisError> {
    let size_field_range = value_start..(value_start + metadata::LATIN1_SIZE_FIELD_LENGTH);
    match data.get(size_field_range) {
        Some(data) => {
            let (bytefield, value_end) = analyze_latin1_string(data, value_start);
            Ok((data::Value::Latin1String(bytefield), value_end))
        }
        None => Err(AnalysisError::DataAnalysisError(
            data::Error::InvalidValueLength,
        )),
    }
}

fn analyze_utf16_string_value(
    data: &[u8],
    value_start: usize,
) -> Result<(data::Value, usize), AnalysisError> {
    let size_field_range = value_start..(value_start + metadata::UTF16_SIZE_FIELD_LENGTH);
    match data.get(size_field_range) {
        Some(data) => {
            let (bytefield, value_end) = analyze_utf16_string(data, value_start);
            Ok((data::Value::Utf16String(bytefield), value_end))
        }
        None => Err(AnalysisError::DataAnalysisError(
            data::Error::InvalidValueLength,
        )),
    }
}
