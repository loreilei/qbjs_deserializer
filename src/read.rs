use encoding::all::{ISO_8859_1, UTF_16LE};
use encoding::{DecoderTrap, Encoding};

use serde_json::{Map, Value};

use crate::analysis::{data, metadata};
use crate::type_conversions::{as_i27, as_u32, as_u64};

fn read_latin1_string(data: &[u8], bytefield: &data::ByteField) -> String {
    match ISO_8859_1.decode(
        &data[bytefield.range.start..bytefield.range.end],
        DecoderTrap::Strict,
    ) {
        Ok(s) => s,
        Err(_) => "".to_owned(),
    }
}

fn read_utf16_string(data: &[u8], bytefield: &data::ByteField) -> String {
    match UTF_16LE.decode(
        &data[bytefield.range.start..bytefield.range.end],
        DecoderTrap::Strict,
    ) {
        Ok(s) => s,
        Err(_) => "".to_owned(),
    }
}

fn read_key(data: &[u8], key: &data::Key) -> String {
    match key {
        data::Key::Latin1String(bytefield) => read_latin1_string(&data, &bytefield),
        data::Key::Utf16String(bytefield) => read_utf16_string(&data, &bytefield),
    }
}

pub fn read_value(data: &[u8], value: &data::Value) -> Value {
    match value {
        data::Value::Null(_) => Value::Null,
        data::Value::Bool(position) => read_bool(&data, *position),
        data::Value::SelfContainedNumber(position) => read_self_contained_number(&data, *position),
        data::Value::Number(bytefield) => read_number(&data, bytefield),
        data::Value::Latin1String(bytefield) => read_latin1_string_value(&data, bytefield),
        data::Value::Utf16String(bytefield) => read_utf16_string_value(&data, bytefield),
        data::Value::Array(array) => read_array(&data, array),
        data::Value::Object(object) => read_object(&data, object),
    }
}

fn read_bool(data: &[u8], position: usize) -> Value {
    Value::Bool((data[position] & 0b100000) != 0)
}

fn read_self_contained_number(data: &[u8], position: usize) -> Value {
    let raw_value = as_u32(&data[position..(position + metadata::VALUE_HEADER_BYTE_SIZE)]);

    Value::Number(serde_json::Number::from(as_i27(raw_value)))
}

fn read_number(data: &[u8], bytefield: &data::ByteField) -> Value {
    let value = f64::from_bits(as_u64(&data[bytefield.range.start..bytefield.range.end]));
    Value::Number(serde_json::Number::from_f64(value).unwrap())
}

fn read_latin1_string_value(data: &[u8], bytefield: &data::ByteField) -> Value {
    let latin1_string = read_latin1_string(&data, &bytefield);
    Value::String(latin1_string)
}

fn read_utf16_string_value(data: &[u8], bytefield: &data::ByteField) -> Value {
    let utf16_string = read_utf16_string(&data, &bytefield);
    Value::String(utf16_string)
}

fn read_array(data: &[u8], array: &data::Array) -> Value {
    Value::Array(
        array
            .values
            .iter()
            .map(|value| read_value(&data, &value))
            .collect::<Vec<Value>>(),
    )
}

fn read_object(data: &[u8], object: &data::Object) -> Value {
    Value::Object(
        object
            .entries
            .iter()
            .map(|entry| {
                let key = read_key(&data, &entry.key);
                let value = read_value(&data, &entry.value);
                (key, value)
            })
            .collect::<Map<String, Value>>(),
    )
}
