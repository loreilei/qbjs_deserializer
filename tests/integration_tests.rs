use qbjs_reader;

use std::fs;

use serde_json;

fn read_test_data_content(path: &str) -> Vec<u8> {
    let test_data_folder = "tests/test_data";

    let test_data_path = format!("{}/{}", test_data_folder, path);

    fs::read(test_data_path).expect(format!("Couldn't read file: {}", path).as_str())
}

#[test]
fn null_document() {
    let test_data_content = read_test_data_content("00_null_document.qbjs");

    let decoded_content = qbjs_reader::decode(&test_data_content);

    assert_eq!(
        decoded_content,
        serde_json::json!({
            "nullObj": serde_json::Value::Null,
        })
    );
}

#[test]
fn bool_true_document() {
    let test_data_content = read_test_data_content("01_bool_true_document.qbjs");

    let decoded_content = qbjs_reader::decode(&test_data_content);

    assert_eq!(
        decoded_content,
        serde_json::json!({
            "aBool": true,
        })
    );
}

#[test]
fn bool_false_document() {
    let test_data_content = read_test_data_content("02_bool_false_document.qbjs");

    let decoded_content = qbjs_reader::decode(&test_data_content);

    assert_eq!(
        decoded_content,
        serde_json::json!({
            "aBool": false,
        })
    );
}

#[test]
fn double_document() {
    let test_data_content = read_test_data_content("03_double_document.qbjs");

    let decoded_content = qbjs_reader::decode(&test_data_content);

    assert_eq!(
        decoded_content,
        serde_json::json!({
            "pi_approx": 3.14159,
        })
    );
}

#[test]
fn int_document() {
    let test_data_content = read_test_data_content("04_int_document.qbjs");

    let decoded_content = qbjs_reader::decode(&test_data_content);

    assert_eq!(
        decoded_content,
        serde_json::json!({
            "pi_approx": 3,
        })
    );
}

#[test]
fn string_document() {
    let test_data_content = read_test_data_content("05_string_document.qbjs");

    let decoded_content = qbjs_reader::decode(&test_data_content);

    assert_eq!(
        decoded_content,
        serde_json::json!({
            "foo": "bar",
        })
    );
}

#[test]
fn null_array_document() {
    let test_data_content = read_test_data_content("06_null_array_document.qbjs");

    let decoded_content = qbjs_reader::decode(&test_data_content);

    assert_eq!(
        decoded_content,
        serde_json::json!({
            "null array": [serde_json::Value::Null,serde_json::Value::Null,serde_json::Value::Null],
        })
    );
}

#[test]
fn bool_array_document() {
    let test_data_content = read_test_data_content("07_bool_array_document.qbjs");

    let decoded_content = qbjs_reader::decode(&test_data_content);

    assert_eq!(
        decoded_content,
        serde_json::json!({
            "bool array": [true, false, true],
        })
    );
}

#[test]
fn double_array_document() {
    let test_data_content = read_test_data_content("08_double_array_document.qbjs");

    let decoded_content = qbjs_reader::decode(&test_data_content);

    assert_eq!(
        decoded_content,
        serde_json::json!({
            "double array": [-0.5, 0.0, 0.5],
        })
    );
}

#[test]
fn int_array_document() {
    let test_data_content = read_test_data_content("09_int_array_document.qbjs");

    let decoded_content = qbjs_reader::decode(&test_data_content);

    assert_eq!(
        decoded_content,
        serde_json::json!({
            "int array": [-5, 0, 5],
        })
    );
}

#[test]
fn string_array_document() {
    let test_data_content = read_test_data_content("10_string_array_document.qbjs");

    let decoded_content = qbjs_reader::decode(&test_data_content);

    assert_eq!(
        decoded_content,
        serde_json::json!({
            "string array": ["foo", "bar", "baz"]
        })
    );
}

#[test]
fn object_document() {
    let test_data_content = read_test_data_content("11_object_document.qbjs");

    let decoded_content = qbjs_reader::decode(&test_data_content);

    assert_eq!(
        decoded_content,
        serde_json::json!({
            "everything": {
            "nullObj": serde_json::Value::Null,
            "aBool": true,
            "anotherBool": false,
            "pi_approx_d": 3.14159,
            "pi_approx_i": 3,
            "foo": "bar",
            "null array": [serde_json::Value::Null,serde_json::Value::Null,serde_json::Value::Null],
            "bool array": [true, false, true],
            "double array": [-0.5, 0.0, 0.5],
            "int array": [-5, 0, 5],
            "string array": ["foo", "bar", "baz"]
            }
        })
    );
}

#[test]
fn object_in_object_document() {
    let test_data_content = read_test_data_content("12_object_in_object_document.qbjs");

    let decoded_content = qbjs_reader::decode(&test_data_content);

    assert_eq!(
        decoded_content,
        serde_json::json!({
            "root" : {
                "child" : {
                    "aBool": false,
                    "anotherBool": true,
                }
            }
        })
    );
}

#[test]
fn empty_document() {
    let test_data_content = read_test_data_content("13_empty_document.qbjs");

    let decoded_content = qbjs_reader::decode(&test_data_content);

    assert_eq!(decoded_content, serde_json::json!({}));
}
