## About this library

This library is an attempt to deserialize files serialized in Qt5's internal binary JSON format.

Applications migrating from Qt5 to Qt6 that were using this JSON document encoding are stuck with the incapacity to read this file format.
This library is an attempt to answer this matter. Since Qt5's internal binary JSON format has been deprecated in Qt6, only the capacity of reading file encoded in this format is addressed by this library, not the capacity to encoded files in this format.

Also, so far, only little endian encoded files are supported.

To deserialize a document, call
```Rust
pub fn deserialize_to_json(qbjs: &[u8]) -> Result<Value, DeserializeError> { ... }
```

The input parameter must be a `u8` slice containing the whole file content as binary (including the header containing the qbjs tag and version).
The output of the function is a `serde_json::Value` or an error, if any happened when deserializing the file.

## How the deserialization is done

The deserialization is done in 2 steps.

First, the library analyses the input slice to determine the type of JSON values that are encoded and which bytes indexes in the slice should be read to decode these values.
That's what the `analysis` module does. It doesnt' read any data, it just tries to find the data slice indexes and translates the JSON structure in its output along with the slice indexes.

Second, once the analysis is done, the `read` module simply attempts to convert the output of the analysis step to actual JSON data.

## Test data

Some basic JSON structures have been encoded to qbjs files thanks to the utilitary application registered as a submodule in `utils/json_to_qbjs_converter`.
The JSON files used to generated the qbjs files with this tool are located in the `tests/test_data/expected_json` folder.
These files are reused by tests: they are parsed with serde_json and the resulting JSON value is compared to the library output.

## C++ FFI
Qt being mainly used with C++ projects, a C++ FFI is available in [this repository](https://github.com/loreilei/qbjs_deserializer_cxx).
It provides [conan](https://conan.io/) packages and CMake finders.


