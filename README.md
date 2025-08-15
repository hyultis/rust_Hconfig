# HConfig

A file configuration manager library that simplifies access to JSON (and other formats) configuration files from a directory.

The key feature of this library is that any variable can be accessed using a path.  
Saving is performed atomically, preventing partial reads from other applications/processes/etc. (saving to a temporary file before renaming).

The `tinyjson` crate is re-exported via `HConfig::tinyjson`.

## Online Documentation

[Master branch](https://github.com/hyultis/rust_Hconfig)

## Example

```rust
fn main() {
    // Configuration path: the directory must exist or be created before proceeding
    HConfigManager::singleton().setConfPath("./config");
    
    // Initialize a new configuration "example" with WrapperJson; it will be stored as "./config/example.json"
    HConfigManager::singleton().create::<WrapperJson>("example").expect("Cannot create HConfig");
    
    // Retrieve a configuration: the name "example" corresponds to "./config/example.json"
    let mut config = HConfigManager::singleton().get("example").expect("Cannot get HConfig");
    
    // Example of retrieving a variable and parsing it as a string (parsing is done via tinyjson)
    let my_var: Option<JsonValue> = config.value_get("testget");
    let my_string: String = config.value_get("testget").unwrap().try_into().unwrap();

	// Set "path/to/save" to "test is updated"
    config.value_set("path/to/save", "test is updated".to_string());
    
    // Save configuration changes
    config.save();
}
```

You can also check the tests.

## License

Licensed under either of

* Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
