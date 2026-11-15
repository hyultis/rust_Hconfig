#![allow(unused_parens)]
#![allow(non_snake_case)]

#[cfg(test)]
mod tests {
	use std::fs::{create_dir, File};
	use std::io::{ErrorKind, Write};
	use std::path::Path;
	use tinyjson::JsonValue;
	use Hconfig::HConfigManager::HConfigManager;
	use Hconfig::IO::json::WrapperJson;

	#[test]
	fn simpleRead()
	{
		setupTestsJson();
		
		let mut config = HConfigManager::singleton().get("test").expect("Cannot get config");
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
		setupTestsJson();

		let mut config = HConfigManager::singleton().get("test").expect("Cannot get config");
		config.value_set("testswrite", "test is ok".to_string());
		config.file_save().expect("Cannot save updated config");
		config.file_load().expect("Cannot load updated config");
		assert_eq!(unwrap_or_not(config.value_get("testswrite")), "test is ok");
	}


	#[test]
	fn simpleRemove()
	{
		setupTestsJson();

		let mut config = HConfigManager::singleton().get("test").expect("Cannot get config");

		// object test
		config.value_set("testswrite", "test is ko".to_string());
		config.value_remove("testswrite");
		assert_eq!(config.value_get("testswrite"), None);

		// array test
		config.value_remove("testarray/1");
		let mut len = 0;
		if let Some(JsonValue::Array(values)) = config.value_get("testarray")
		{
			len=values.len();
		}
		assert_eq!(len, 2);
	}
	
	
	#[test]
	fn mutWriteAndSave()
	{
		setupTestsJson();

		let mut config = HConfigManager::singleton().get("test").expect("Cannot get config");
		//println!("file path {}",config.file_path());
		if let Some(tmp) = config.value_get_mut("test/get/mut")
		{
			*tmp = JsonValue::String("testmut is ok".to_string());
		}
		config.file_save().expect("Cannot save updated config");
		assert_eq!(unwrap_or_not(config.value_get("test/get/mut")), "testmut is ok");
	}
	
	#[test]
	fn arrayIndex()
	{
		setupTestsJson();
		HConfigManager::singleton().create::<WrapperJson>("test_array").expect("Cannot create test_array HConfig");
		let mut config = HConfigManager::singleton().get("test_array").expect("Cannot get config");
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
	
	fn setupTestsJson()
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
		
		let testConfFile = Path::new("./config/test.json");
		let mut Rfile = File::options().create(true).write(true).truncate(true).open(testConfFile).expect(format!("Cannot create default test file : {testConfFile:?}").as_str());
		Rfile.write_all(r#"{
			"testget":"testget is ok",
			"testarray":["ignore","testarray is ok","ignore"]
		}"#.as_bytes()).unwrap();
		
		HConfigManager::singleton().confPath_set(configDir.to_str().unwrap().to_string());
		HConfigManager::singleton().create::<WrapperJson>("test").expect("Cannot create test HConfig");
	}
}
