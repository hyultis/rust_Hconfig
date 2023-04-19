# HConfig

A file configuration manager library that simplify access of json configuration files from a dir.

if a config file is not existing, it will be created. Any var is accessed by a path.
Saving use a atomic method disallowing partial load from other app/process/etc

## Online Documentation

[Master branch](https://github.com/hyultis/rust_Hconfig)

## Example

```
fn main()
{
    // configuration, the directory need to be existing or created before continuing
    HConfigManager::HConfigManager::singleton().setConfPath("./config");
    
    // get a "config", the name "test" mean "./config/test.json"
    let config = HConfigManager::HConfigManager::singleton().get("test").unwrap();
    
    // exemple of getting a var
    let myvar: Option<JsonValue> = config.get("name");
    let myvar: Option<JsonValue> = config.get("path/to/myvar");
    let myvar: Option<JsonValue> = config.get("array/0/myvar");
    
    // updating a var with set() (note : update() if from HArcMut lib)
    let config = HConfigManager::HConfigManager::singleton().get_mut("test").unwrap();
    config.update(|thisconf|
    {
        thisconf.set("name", |tmp| {
            *tmp = JsonValue::String("test is update".to_string());
        });
        
        // save change into file
        thisconf.save().expect("Cannot save updated config");
    });
}
```

## License

Licensed under either of

* Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
