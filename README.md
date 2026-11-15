# HConfig

A file configuration manager library that simplifies access to JSON (and other formats) configuration files from a directory.

The key feature of this library is that any variable can be accessed using a path.  
Saving is performed atomically, preventing partial reads from other applications/processes/etc. (saving to a temporary file synced on disk, then renaming it).

The `tinyjson` crate is re-exported via `hconfig::tinyjson`.

## Online Documentation

[Master branch](https://github.com/hyultis/rust_Hconfig)

## Installation

The crates.io package is named `Hconfig`, the library is imported as `hconfig` :

```toml
[dependencies]
Hconfig = "3"
```

## Example

```rust,no_run
use hconfig::HConfigManager::HConfigManager;
use hconfig::IO::json::WrapperJson;
use hconfig::tinyjson::JsonValue;

fn main() {
    // Configuration path: the directory must exist or be created before proceeding
    HConfigManager::singleton().confPath_set("./config");

    // Load (or create) the configuration "example" with WrapperJson; it is stored as "./config/example.json"
    HConfigManager::singleton().create::<WrapperJson>("example").expect("Cannot create HConfig");

    // Retrieve a shared handle on the configuration, then lock it for writing
    let config = HConfigManager::singleton().get("example").expect("Cannot get HConfig");
    let mut config = config.write().expect("HConfig lock poisoned");

    // Retrieve a variable, and parse it as a string (parsing is done via tinyjson)
    let my_var: Option<JsonValue> = config.value_get("testget");
    let my_string: Option<String> = my_var.and_then(|value| value.try_into().ok());

    // Set "path/to/save" to "test is updated"
    config.value_set("path/to/save", "test is updated".to_string());

    // Save configuration changes
    config.file_save().expect("Cannot save HConfig");
}
```

You can also check the tests.

## Paths

A path is a list of keys separated by `/`; an unsigned integer targets an array index (`servers/0/host`). Empty segments are ignored.

- Reading (`value_get`, `value_get_mut`, `value_remove`) never creates anything: a missing path returns `None` / `false`.
- Writing (`value_set`, `value_get_or_set`) forces the path: missing keys are created, an out of bounds array index grows the array (filled with `null`), and a scalar on the path is replaced by an object.

## Upgrading from 2.x

See the `3.0.0` section of [CHANGELOD.md](CHANGELOD.md).

## License

Licensed under either of

* Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
