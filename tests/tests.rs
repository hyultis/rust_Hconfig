#![allow(unused_parens)]
#![allow(non_snake_case)]

#[cfg(test)]
mod tests {
	use std::collections::HashMap;
	use std::fmt;
	use std::fs::{create_dir, read_dir, File};
	use std::io::{ErrorKind, Write};
	use std::path::Path;
	use tinyjson::JsonValue;
	use Hconfig::Errors;
	use Hconfig::HConfigManager::HConfigManager;
	use Hconfig::IO::IOwrapper;
	use Hconfig::IO::json::WrapperJson;
	use Hconfig::Utils::jsonIntoHashMap;

	const DEFAULT_CONTENT: &str = r#"{
		"testget":"testget is ok",
		"testarray":["ignore","testarray is ok","ignore"],
		"test":{"get":{"mut":"original"}}
	}"#;

	#[test]
	fn simpleRead()
	{
		setupTestsJson("test_read", DEFAULT_CONTENT);

		let config = HConfigManager::singleton().get("test_read").expect("Cannot get config");
		let mut config = config.write().expect("Config lock poisoned");
		assert_eq!(unwrap_or_not(config.value_get("testget")), "testget is ok");
		assert_eq!(unwrap_or_not(config.value_get("testarray/1")), "testarray is ok");

		if let Some(tmp) = config.value_get_mut("test/get/mut")
		{
			*tmp = JsonValue::String("test is ok".to_string());
		}
		config.file_save().expect("Cannot save updated config");
		assert_eq!(unwrap_or_not(config.value_get("test/get/mut")), "test is ok");
	}

	#[test]
	fn simpleWriteAndSave()
	{
		setupTestsJson("test_write", DEFAULT_CONTENT);

		let config = HConfigManager::singleton().get("test_write").expect("Cannot get config");
		let mut config = config.write().expect("Config lock poisoned");
		config.value_set("testswrite", "test is ok".to_string());
		config.file_save().expect("Cannot save updated config");
		config.file_load().expect("Cannot load updated config");
		assert_eq!(unwrap_or_not(config.value_get("testswrite")), "test is ok");

		// no temporary file left after a save
		let leftovers = read_dir("./config").expect("Cannot read config dir")
			.filter_map(|entry| entry.ok())
			.filter(|entry| entry.file_name().to_string_lossy().starts_with("test_write.json_"))
			.count();
		assert_eq!(leftovers, 0);
	}

	#[test]
	fn simpleRemove()
	{
		setupTestsJson("test_remove", DEFAULT_CONTENT);

		let config = HConfigManager::singleton().get("test_remove").expect("Cannot get config");
		let mut config = config.write().expect("Config lock poisoned");

		// object test
		config.value_set("testswrite", "test is ko".to_string());
		assert!(config.value_remove("testswrite"));
		assert_eq!(config.value_get("testswrite"), None);

		// array test
		assert!(config.value_remove("testarray/1"));
		assert_eq!(arrayLen(config.value_get("testarray")), 2);

		// removing a missing path creates nothing
		assert!(!config.value_remove("missing/sub/key"));
		assert_eq!(config.value_get("missing"), None);
	}

	#[test]
	fn mutWriteAndSave()
	{
		setupTestsJson("test_mut", DEFAULT_CONTENT);

		let config = HConfigManager::singleton().get("test_mut").expect("Cannot get config");
		let mut config = config.write().expect("Config lock poisoned");
		if let Some(tmp) = config.value_get_mut("test/get/mut")
		{
			*tmp = JsonValue::String("testmut is ok".to_string());
		}
		config.file_save().expect("Cannot save updated config");
		assert_eq!(unwrap_or_not(config.value_get("test/get/mut")), "testmut is ok");

		// get_mut on a missing path creates nothing
		assert!(config.value_get_mut("missing/sub").is_none());
		assert_eq!(config.value_get("missing"), None);

		// root_get_mut gives a real mutable access
		if let JsonValue::Object(root) = config.root_get_mut()
		{
			root.insert("fromroot".to_string(), JsonValue::Boolean(true));
		}
		assert_eq!(config.value_get("fromroot"), Some(JsonValue::Boolean(true)));
	}

	#[test]
	fn getOrSet()
	{
		setupTestsJson("test_getorset", DEFAULT_CONTENT);

		let config = HConfigManager::singleton().get("test_getorset").expect("Cannot get config");
		let mut config = config.write().expect("Config lock poisoned");
		assert_eq!(unwrap_or_not(Some(config.value_get_or_set("testget", "default".to_string()))), "testget is ok");
		assert_eq!(unwrap_or_not(Some(config.value_get_or_set("new/key", "default".to_string()))), "default");
		assert_eq!(unwrap_or_not(config.value_get("new/key")), "default");
	}

	#[test]
	fn pathEdgeCases()
	{
		setupTestsJson("test_paths", DEFAULT_CONTENT);

		let config = HConfigManager::singleton().get("test_paths").expect("Cannot get config");
		let mut config = config.write().expect("Config lock poisoned");

		// writing through a scalar replaces it by an object
		config.value_set("testget/sub", "sub is ok".to_string());
		assert_eq!(unwrap_or_not(config.value_get("testget/sub")), "sub is ok");

		// empty segments are ignored
		config.value_set("slash//double/", "double is ok".to_string());
		assert_eq!(unwrap_or_not(config.value_get("slash/double")), "double is ok");

		// an empty path does nothing
		config.value_set("", "empty".to_string());
		config.value_set("/", "empty".to_string());
		assert_eq!(config.value_get(""), None);
		assert!(config.value_get_mut("").is_none());
		assert!(!config.value_remove(""));
		if let JsonValue::Object(root) = config.root_get()
		{
			assert!(!root.contains_key(""));
		}

		// a non numeric segment on an array is ignored
		config.value_set("testarray/key", "ko".to_string());
		assert_eq!(arrayLen(config.value_get("testarray")), 3);
	}

	#[test]
	fn arrayIndex()
	{
		setupTestsJson("test_array", "{}");
		let config = HConfigManager::singleton().get("test_array").expect("Cannot get config");
		let mut config = config.write().expect("Config lock poisoned");
		config.value_set("array", JsonValue::Array(vec!["first".to_string().into(), "second".to_string().into()]));

		// get out of bounds
		assert_eq!(config.value_get("array/5"), None);
		assert_eq!(config.value_get("array/5/cmd"), None);

		// get_mut out of bounds : no panic, no creation
		assert!(config.value_get_mut("array/5/cmd").is_none());
		assert!(config.value_get_mut("array/5").is_none());
		assert_eq!(arrayLen(config.value_get("array")), 2);

		// set on an existing index
		config.value_set("array/1", "second is updated".to_string());
		assert_eq!(unwrap_or_not(config.value_get("array/1")), "second is updated");
		assert_eq!(arrayLen(config.value_get("array")), 2);

		// get_mut on an existing index
		if let Some(tmp) = config.value_get_mut("array/0")
		{
			*tmp = JsonValue::String("first is updated".to_string());
		}
		assert_eq!(unwrap_or_not(config.value_get("array/0")), "first is updated");

		// set out of bounds : the array grows, filled with null
		config.value_set("array/4", "fifth".to_string());
		assert_eq!(arrayLen(config.value_get("array")), 5);
		assert_eq!(config.value_get("array/3"), Some(JsonValue::Null));
		assert_eq!(unwrap_or_not(config.value_get("array/4")), "fifth");

		// set out of bounds with a deeper path
		config.value_set("array/6/cmd", "cmd is ok".to_string());
		assert_eq!(arrayLen(config.value_get("array")), 7);
		assert_eq!(unwrap_or_not(config.value_get("array/6/cmd")), "cmd is ok");

		// get_or_set out of bounds
		assert_eq!(unwrap_or_not(Some(config.value_get_or_set("array/7", "default".to_string()))), "default");
		assert_eq!(unwrap_or_not(config.value_get("array/7")), "default");

		// remove out of bounds
		assert!(!config.value_remove("array/20"));
		assert!(!config.value_remove("array/20/cmd"));
		assert_eq!(arrayLen(config.value_get("array")), 8);
	}

	#[test]
	fn displayIsJson()
	{
		setupTestsJson("test_display", r#"{"key":"value"}"#);

		let config = HConfigManager::singleton().get("test_display").expect("Cannot get config");
		let config = config.read().expect("Config lock poisoned");
		let displayed: JsonValue = config.to_string().parse().expect("Display is not json");
		assert_eq!(&displayed, config.root_get());
	}

	#[test]
	fn invalidJsonFile()
	{
		setupConfigDir();
		writeConfigFile("test_invalid", "{not json");
		let result = HConfigManager::singleton().create::<WrapperJson>("test_invalid");
		assert!(matches!(result, Err(Errors::ConfigCannotConvertFileToJsonValue(..))));
	}

	#[test]
	fn unreadableFileIsNotOverwritten()
	{
		setupConfigDir();
		let content: &[u8] = b"{\"key\":\"\xff\"}";
		std::fs::write("./config/test_unreadable.json", content).expect("Cannot write test file");

		let result = HConfigManager::singleton().create::<WrapperJson>("test_unreadable");
		assert!(matches!(result, Err(Errors::ConfigCannotLoadFile(..))));
		assert_eq!(std::fs::read("./config/test_unreadable.json").expect("Cannot read test file"), content);
	}

	#[test]
	fn missingFileIsCreated()
	{
		setupConfigDir();
		let _ = std::fs::remove_file("./config/test_missing.json");

		HConfigManager::singleton().create::<WrapperJson>("test_missing").expect("Cannot create HConfig on a missing file");
		assert!(Path::new("./config/test_missing.json").exists());
		let config = HConfigManager::singleton().get("test_missing").expect("Cannot get config");
		let config = config.read().expect("Config lock poisoned");
		assert_eq!(config.root_get(), &JsonValue::Object(Default::default()));
	}

	#[test]
	fn createReloadsInPlace()
	{
		setupTestsJson("test_reload", r#"{"key":"before"}"#);
		let handle = HConfigManager::singleton().get("test_reload").expect("Cannot get config");

		writeConfigFile("test_reload", r#"{"key":"after"}"#);
		HConfigManager::singleton().create::<WrapperJson>("test_reload").expect("Cannot reload HConfig");

		// a handle obtained before the reload sees the new content
		assert_eq!(unwrap_or_not(handle.read().expect("Config lock poisoned").value_get("key")), "after");
	}

	#[test]
	fn customWrapper()
	{
		setupTestsJson("test_custom", DEFAULT_CONTENT);
		HConfigManager::singleton().create::<WrapperCustom>("test_custom").expect("Cannot create custom HConfig");

		let config = HConfigManager::singleton().get("test_custom").expect("Cannot get config");
		let config = config.read().expect("Config lock poisoned");
		assert_eq!(config.name_get(), "test_custom");
		assert_eq!(config.file_path(), "./config/test_custom.json");
		assert_eq!(config.file_ext(), "custom");
		assert_eq!(unwrap_or_not(config.value_get("testget")), "testget is ok");
	}

	#[test]
	fn utilsJsonIntoHashMap()
	{
		let value: JsonValue = r#"{"text":"a","number":1,"flag":true,"sub":{"k":"v"}}"#.parse().expect("invalid test json");
		let map = jsonIntoHashMap(value);
		assert_eq!(map.get("text").map(String::as_str), Some("a"));
		assert_eq!(map.get("number").map(String::as_str), Some("1"));
		assert_eq!(map.get("flag").map(String::as_str), Some("true"));
		assert_eq!(map.get("sub").map(String::as_str), Some(r#"{"k":"v"}"#));

		let value: JsonValue = r#"["a",2]"#.parse().expect("invalid test json");
		let map = jsonIntoHashMap(value);
		assert_eq!(map, HashMap::from([("0".to_string(), "a".to_string()), ("1".to_string(), "2".to_string())]));

		assert!(jsonIntoHashMap(JsonValue::Null).is_empty());
	}

	/// a wrapper using WrapperJson storage, only to check that `create::<T>` really uses T
	#[derive(Debug)]
	struct WrapperCustom
	{
		inner: WrapperJson,
	}

	impl fmt::Display for WrapperCustom
	{
		fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
		{
			return write!(f, "{}", self.inner);
		}
	}

	impl IOwrapper for WrapperCustom
	{
		fn init(name: &str, path: &str) -> Result<Self, Errors>
		{
			return Ok(WrapperCustom { inner: WrapperJson::init(name, path)? });
		}
		fn file_load(&mut self) -> Result<(), Errors> { return self.inner.file_load(); }
		fn file_save(&self) -> Result<(), Errors> { return self.inner.file_save(); }
		fn file_ext(&self) -> &'static str { return "custom"; }
		fn file_path(&self) -> &str { return self.inner.file_path(); }
		fn root_get(&self) -> &JsonValue { return self.inner.root_get(); }
		fn root_get_mut(&mut self) -> &mut JsonValue { return self.inner.root_get_mut(); }
		fn value_get(&self, path: &str) -> Option<JsonValue> { return self.inner.value_get(path); }
		fn value_get_or_set(&mut self, path: &str, default: JsonValue) -> JsonValue { return self.inner.value_get_or_set(path, default); }
		fn value_get_mut(&mut self, path: &str) -> Option<&mut JsonValue> { return self.inner.value_get_mut(path); }
		fn value_remove(&mut self, path: &str) -> bool { return self.inner.value_remove(path); }
		fn value_set(&mut self, path: &str, newval: JsonValue) { self.inner.value_set(path, newval); }
	}

	fn arrayLen(value: Option<JsonValue>) -> usize
	{
		if let Some(JsonValue::Array(values)) = value
		{
			return values.len();
		}
		return 0;
	}

	// simply return a "not ok" is something is wrong
	fn unwrap_or_not(base: Option<JsonValue>) -> String
	{
		if let Some(jsonval) =  base
		{
			if let Ok(finalval) = jsonval.try_into()
			{
				return finalval;
			}
		}
		return "not ok".to_string();
	}

	fn setupConfigDir()
	{
		let configDir = Path::new("./config");
		if (!configDir.exists())
		{
			match create_dir(configDir) {
				Ok(_) => {},
				Err(ref _e) if _e.kind() == ErrorKind::AlreadyExists   => {},
				Err(e) => {panic!("Cannot create \"{configDir:?}\": {e}")}
			}
		}
		HConfigManager::singleton().confPath_set(configDir.to_str().unwrap().to_string());
	}

	fn writeConfigFile(name: &str, content: &str)
	{
		let testConfFile = format!("./config/{name}.json");
		let mut Rfile = File::options().create(true).write(true).truncate(true).open(&testConfFile).expect(format!("Cannot create default test file : {testConfFile:?}").as_str());
		Rfile.write_all(content.as_bytes()).unwrap();
	}

	/// each test uses its own config name : tests run in parallel and share the singleton
	fn setupTestsJson(name: &str, content: &str)
	{
		setupConfigDir();
		writeConfigFile(name, content);
		HConfigManager::singleton().create::<WrapperJson>(name).expect("Cannot create test HConfig");
	}
}
