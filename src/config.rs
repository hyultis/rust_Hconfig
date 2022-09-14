
use std::fs::File;
use std::io::{Read, Write, stdout};
use std::fmt;
use std::sync::{RwLock, Arc};
use json::JsonValue;
use anyhow::{Result, anyhow};
use std::borrow::BorrowMut;

#[derive(Clone,Debug)]
pub struct Config
{
	path : String,
	datas : Arc<RwLock<JsonValue>>
}

impl Config
{
	/// create new instance and autoload content
	pub fn new(path: String) -> Result<Config>
	{
		println!("config file path : {}",&path);
		let mut file = File::open(&path)?;
		let mut tmp = String::new();
		file.read_to_string(&mut tmp)?;

		let tmp = json::parse(tmp.as_str())?;

		println!("Json content : ");
		tmp.write_pretty(stdout().borrow_mut(), 4)?;
		println!("");

		Ok(Config
		{
			path : path.clone(),
			datas : Arc::new(RwLock::new(tmp))
		})
	}

	pub fn get(&self, path: &str) -> Option<JsonValue>
	{
		let tmp = self.datas.read().unwrap();
		tmp[path].write_pretty(stdout().borrow_mut(), 4);
		println!("");
		Some(tmp[path].clone())
	}

	pub fn set(&self, datas:String)
	{
		let mut tmp= self.datas.write().unwrap();
		*tmp = JsonValue::String(datas);
	}
}

impl fmt::Display for Config {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}", self.datas.read().unwrap())
	}
}