use qbjs_deserializer;

use std::fs;

use serde_json;

macro_rules! create_test {
    // This macro takes an argument of designator `ident` and creates a test `$test_name`.
    // It uses the test_name (minus the first charcter) to look for file to test and file to read to know what json value to expect
    ($test_name:ident) => {
        #[test]
        fn $test_name() {
            let test_name = stringify!($test_name);
            let file_name = &test_name[1..];
            let expected_json_test_data_folder = "tests/test_data/expected_json";
            let expected_json_file_path =
                format!("{}/{}.json", expected_json_test_data_folder, file_name);

            let qbjs_test_data_folder = "tests/test_data/qbjs_data";
            let qbjs_file_path = format!("{}/{}.qbjs", qbjs_test_data_folder, file_name);

            let read_file = |file_path| {
                fs::read(file_path).expect(format!("Couldn't read file: {}", file_path).as_str())
            };

            let expected_json_content = read_file(&expected_json_file_path);
            let expected_json = match &expected_json_content.len() {
                0 => serde_json::json!({}),
                _ => serde_json::from_slice::<serde_json::Value>(&expected_json_content).unwrap(),
            };

            let qbjs_content = read_file(&qbjs_file_path);
            let deserialized_content = qbjs_deserializer::qbjs::deserialize_to_json(&qbjs_content);

            assert_eq!(deserialized_content.unwrap(), expected_json);
        }
    };
}

macro_rules! create_tests {
    ($test_name:ident) => {
        create_test!($test_name);
    };
    ($test_name:ident, $($test_names:ident),+) => {
        create_test!($test_name);
        create_tests!($($test_names),+);
    };
}

macro_rules! create_error_check_test {
    // This macro takes an argument of designator `ident` and creates a test `$test_name`.
    // It uses the test_name (minus the first charcter) to look for file to test and file to read to know what json value to expect
    ($test_name:ident, $expected_error:expr) => {
        #[test]
        fn $test_name() {
            let test_name = stringify!($test_name);
            let file_name = &test_name[1..];

            let qbjs_test_data_folder = "tests/test_data/qbjs_data";
            let qbjs_file_path = format!("{}/{}.qbjs", qbjs_test_data_folder, file_name);

            let read_file = |file_path| {
                fs::read(file_path).expect(format!("Couldn't read file: {}", file_path).as_str())
            };

            let qbjs_content = read_file(&qbjs_file_path);
            let deserialized_content = qbjs_deserializer::qbjs::deserialize_to_json(&qbjs_content);

            match deserialized_content {
                Ok(_) => {
                    assert!(false)
                }
                Err(e) => {
                    assert_eq!($expected_error, e)
                }
            }
        }
    };
}
// Test code ranges:
// 0 -> 99 basic document with object value as root
// 100 -> 199 basic document with array value as root
// 200 -> 299 documents composed of objects and arrays mixed
// 300 -> 399 documents supposed to trigger error codes from API
// 400 -> 499 "real" json documents
create_tests!(
    _000_null_object_document,
    _001_bool_true_object_document,
    _002_bool_false_object_document,
    _003_double_object_document,
    _004_double_zero_object_document,
    _005_negative_double_object_document,
    _006_int_object_document,
    // Disabled because Qt always encode 0 as double apparently
    // _007_int_zero_object_document,
    _008_negative_int_object_document,
    _009_string_object_document,
    _010_strings_object_document,
    _011_japanese_string_object_document,
    _012_various_values_object_document,
    _100_null_array_document,
    _101_bool_array_document,
    _102_double_array_document,
    _103_int_array_document,
    _104_string_array_document,
    _105_various_values_array_document,
    _200_object_object_document,
    _201_array_object_document,
    _202_tree_object_document,
    _203_tree_array_document,
    _204_array_in_array_document,
    _205_tree_array_in_array_document,
    _206_objects_in_array_document,
    _207_tree_empty_arrays_in_object_document,
    _208_tree_empty_objects_in_object_document,
    _300_empty_document,
    _400_example_from_qbjs_source_document
);

// FIXME: Try to find the right macro to declare test name and expected error code as a list of tuple
create_error_check_test!(
    _301_insufficient_data_document,
    qbjs_deserializer::error::DecodeError::InsufficientData
);
create_error_check_test!(
    _302_invalid_qbjs_tag_document,
    qbjs_deserializer::error::DecodeError::MalformedHeader
);
create_error_check_test!(
    _303_invalid_qbjs_version_document,
    qbjs_deserializer::error::DecodeError::MalformedHeader
);
