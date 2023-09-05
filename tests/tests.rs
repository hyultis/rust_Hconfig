#![allow(unused_parens)]
#![allow(non_snake_case)]

#[cfg(test)]
mod tests {
	use std::fs::{create_dir, File};
	use std::io::Write;
	use std::path::Path;
	use json::JsonValue;
	use Hconfig::HConfigManager::HConfigManager;
	
	#[test]
	fn log() {
		let configDir = Path::new("./config");
		if (!configDir.exists())
		{
			create_dir(configDir).unwrap();
		}
		let testConfFile = Path::new("./config/test.json");
		let mut Rfile = File::create(testConfFile).unwrap();
		Rfile.write_all(b"{\
			\"testget\":\"test is ok\",\
			\"testarray\":[\"ignore\",\"test is ok\",\"ignore\"]
		}").unwrap();
		
		HConfigManager::singleton().setConfPath("./config");
		let mut config = HConfigManager::singleton().get("test");
		assert_eq!(config.get("testget").unwrap(), "test is ok");
		assert_eq!(config.get("testarray/1").unwrap(), "test is ok");
		
		config.set("testset", "test is ok".to_string());
		config.save().expect("Cannot save updated config");
		assert_eq!(config.get("testset").unwrap(), "test is ok");
		
		if let Some(tmp) = config.get_mut("test/get/mut")
		{
			*tmp = JsonValue::String("test is ok".to_string());
		}
		config.save().expect("Cannot save updated config");
		assert_eq!(config.get("test/get/mut").unwrap(), "test is ok");
	}
}
