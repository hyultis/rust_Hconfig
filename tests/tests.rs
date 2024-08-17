#![allow(unused_parens)]
#![allow(non_snake_case)]

#[cfg(test)]
mod tests {
	use std::fs::{create_dir, File};
	use std::io::{ErrorKind, Write};
	use std::path::Path;
	use Hconfig::rusty_json::base::JsonValue;
	use Hconfig::HConfigManager::HConfigManager;
	
	#[test]
	fn simpleRead()
	{
		exampleFileWithData();
		
		let mut config = HConfigManager::singleton().get("test");
		assert_eq!(unwrap_or_not(config.get("testget")), "testget is ok");
		assert_eq!(unwrap_or_not(config.get("testarray/1")), "testarray is ok");
		
		if let Some(tmp) = config.get_mut("test/get/mut")
		{
			*tmp = JsonValue::String("test is ok".to_string());
		}
		config.save().expect("Cannot save updated config");
		assert_eq!(unwrap_or_not(config.get("test/get/mut")), "test is ok");
	}
	
	#[test]
	fn simpleWriteAndSave()
	{
		exampleFileWithData();
		
		let mut config = HConfigManager::singleton().get("test");
		config.set("testswrite", "test is ok".to_string());
		config.save().expect("Cannot save updated config");
		assert_eq!(unwrap_or_not(config.get("testswrite")), "test is ok");
	}
	
	
	#[test]
	fn mutWriteAndSave()
	{
		exampleFileWithData();
		
		let mut config = HConfigManager::singleton().get("test");
		if let Some(tmp) = config.get_mut("test/get/mut")
		{
			*tmp = JsonValue::String("testmut is ok".to_string());
		}
		config.save().expect("Cannot save updated config");
		assert_eq!(unwrap_or_not(config.get("test/get/mut")), "testmut is ok");
	}
	
	// simply return a "not ok" is something is wrong
	fn unwrap_or_not(base: Option<JsonValue>) -> String
	{
		if let Some(jsonval) =  base
		{
			if let Ok(finalval) = jsonval.parse::<String>()
			{
				return finalval;
			}
		}
		return "not ok".to_string();
	}
	
	fn exampleFileWithData()
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
		let mut Rfile = File::options().create(true).write(true).truncate(true).open(testConfFile).expect(format!("Cannot create : {testConfFile:?}").as_str());
		Rfile.write_all(b"{\
			\"testget\":\"testget is ok\",\
			\"testarray\":[\"ignore\",\"testarray is ok\",\"ignore\"]
		}").unwrap();
		
		HConfigManager::singleton().setConfPath("./config");
	}
}
