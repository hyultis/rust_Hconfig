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
	fn mutWriteAndSave()
	{
		setupTestsJson();

		let mut config = HConfigManager::singleton().get("test").expect("Cannot get config");
		println!("file path {}",config.file_path());
		if let Some(tmp) = config.value_get_mut("test/get/mut")
		{
			*tmp = JsonValue::String("testmut is ok".to_string());
		}
		config.file_save().expect("Cannot save updated config");
		assert_eq!(unwrap_or_not(config.value_get("test/get/mut")), "testmut is ok");
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
