#![allow(unused_parens)]
#![allow(non_snake_case)]

#[cfg(test)]
mod tests {
	use std::fs::{create_dir, File};
	use std::io::Write;
	use std::path::Path;
	use Hconfig::rusty_json::base::JsonValue;
	use Hconfig::HConfigManager::HConfigManager;
	
	#[test]
	fn log() {
		let configDir = Path::new("./config");
		if (!configDir.exists())
		{
			create_dir(configDir).expect(format!("Cannot create : {configDir:?}").as_str());
		}
		let testConfFile = Path::new("./config/test.json");
		let mut Rfile = File::create(testConfFile).expect(format!("Cannot create : {testConfFile:?}").as_str());
		Rfile.write_all(b"{\
			\"testget\":\"test is ok\",\
			\"testarray\":[\"ignore\",\"test is ok\",\"ignore\"]
		}").unwrap();
		
		HConfigManager::singleton().setConfPath("./config");
		let mut config = HConfigManager::singleton().get("test");
		assert_eq!(unwrap_or_not(config.get("testget")), "test is ok");
		assert_eq!(unwrap_or_not(config.get("testarray/1")), "test is ok");
		
		config.set("testset", "test is ok".to_string());
		config.save().expect("Cannot save updated config");
		assert_eq!(unwrap_or_not(config.get("testset")), "test is ok");
		
		if let Some(tmp) = config.get_mut("test/get/mut")
		{
			*tmp = JsonValue::String("test is ok".to_string());
		}
		config.save().expect("Cannot save updated config");
		assert_eq!(unwrap_or_not(config.get("test/get/mut")), "test is ok");
	}
	
	fn unwrap_or_not(base: Option<JsonValue>) -> String
	{
		return base.unwrap_or_else(||{JsonValue::String("not ok".to_string())}).parse().unwrap_or_else(|_|{"not ok".to_string()});
	}
}
