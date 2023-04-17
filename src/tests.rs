#[cfg(test)]
mod tests {
	use std::fs::{create_dir, File};
	use std::io::Write;
	use std::path::Path;
	use json::JsonValue;
	use super::super::*;
	
	#[test]
	fn log() {
		let configDir = Path::new("./config");
		if(!configDir.exists())
		{
			create_dir(configDir).unwrap();
		}
		let testConfFile = Path::new("./config/test.json");
		let mut Rfile = File::create(testConfFile).unwrap();
		Rfile.write_all(b"{\
			\"name\":\"test is ok\"\
		}").unwrap();
		
		HConfigManager::HConfigManager::singleton().setConfPath("./config");
		let config = HConfigManager::HConfigManager::singleton().get_mut("test").unwrap();
		assert_eq!(config.get().get("name").unwrap(),"test is ok");
		config.update(|thisconf|
		{
			thisconf.set("name", |tmp| {
				*tmp = JsonValue::String("test is update".to_string());
			});
			thisconf.save().expect("Cannot save updated config");
		});
		assert_eq!(config.get().get("name").unwrap(),"test is update");
	}
}
