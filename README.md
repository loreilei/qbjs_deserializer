## About this library

This library is an attempt to deserialize files serialized in Qt5's internal binary JSON format.

I started this project because I wasn't aware of [QBinaryJson](https://doc.qt.io/qt-6/qbinaryjson.html) and thought that applications migrating from Qt5 to Qt6 that were using this JSON document encoding were stuck with the incapacity to read this file format. Noneless, this "happy" mistake let me work on this project and discover many aspects of Rust and how to use Rust crates from C++ code.

This library is an attempt to provide an alternative to [QBinaryJson](https://doc.qt.io/qt-6/qbinaryjson.html). Since Qt5's internal binary JSON format has been deprecated in Qt6, only the capacity of reading file encoded in this format is addressed by this library, not the capacity to encode files in this format.

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
That's what the `analysis` module does. It doesn't read any data, it just tries to find the data slice indexes and translates the JSON structure in its output along with the slice indexes.

Second, once the analysis is done, the `read` module simply attempts to convert the output of the analysis step to actual JSON data.

## File format transcode capacity

The Qt5's internal binary JSON format is deserialized as a [serde](https://crates.io/crates/serde) [Value](https://docs.rs/serde_json/latest/serde_json/value/enum.Value.html). This serde Value can be given to serde compatible serializer to transcode to another file format (CBOR, YAML, ect: [pick your preference](https://serde.rs/#data-formats)).

## Test data

Some basic JSON structures have been encoded to qbjs files thanks to the utilitary application registered as a submodule in `utils/json_to_qbjs_converter`.
The JSON files used to generated the qbjs files with this tool are located in the `tests/test_data/expected_json` folder.
These files are reused by tests: they are parsed with serde_json and the resulting JSON value is compared to the library output.

## C++ FFI
Qt is mainly used with C++ projects.

As mentionned in the "About this library" paragraph, [QBinaryJson](https://doc.qt.io/qt-6/qbinaryjson.html) is available for C++ application that migrated to Qt6, and __I recommend using this official implementation from Qt on production code__ (I believe the source code of Qt5 handling this binary JSON format was extracted to a dedicated compatibility module).

However if you feel adventurous and want to give a try to this Rust crate in your C++ codebase, I made this C++ FFI that is available in [this repository](https://gitlab.com/qbjs_deserializer/qbjs_deserializer_cxx).
It provides [conan](https://conan.io/) packages and CMake finders.
