# HConfig

A file configuration manager library that simplify access of json configuration files from a dir.

if a config file is not existing, it will be created. Any var is accessed by a path.
Saving use atomic method disallowing partial load from other app/process/etc.

the used serde_json crate is re-exported via "Hconfig::serde_json"

## Online Documentation

[Master branch](https://github.com/hyultis/rust_Hconfig)

## Example

```
fn main()
{
    // configuration path, the directory need to be existing or created before continuing
	HConfigManager::singleton().setConfPath("./config");
	
	// get a "config", the name "example" mean "./config/example.json"
	let mut config = HConfigManager::singleton().get("example");
	
	// exemple of getting a var and getting a string (parse is from serde_json)
	let myVar: Option<Value> = config.get("name");
	let myString: String = config.get("path/to/myvar").unwrap().as_str().unwrap().to_string();
	
	config.set("path/to/save",JsonValue::String("test is update".to_string()));
	
	// save config modification.
	config.save();
}
```

you can also check tests.

## License

Licensed under either of

* Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
