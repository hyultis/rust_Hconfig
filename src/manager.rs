
use once_cell::sync::OnceCell;
use std::str::FromStr;
use super::config::Config as config;
use std::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard};
use std::collections::HashMap;
use std::rc::Rc;
use std::borrow::Borrow;
use std::cell::Ref;
use std::ops::Deref;
use anyhow::{Result,anyhow};

pub struct Manager
{
	confpath : RwLock<Result<String>>,
	loadedConfs : RwLock<HashMap<String, config>>
}

static SINGLETON: OnceCell<Manager> = OnceCell::new();

impl Manager
{
	pub fn new() -> Manager {
		Manager{
			confpath: RwLock::new(Err(anyhow!("ConfPath not defined."))),
			loadedConfs: RwLock::new(HashMap::new())
		}
	}

	pub fn singleton() -> &'static Manager
	{
		SINGLETON.get_or_init(|| {
			//RwLock::new(Manager::new())
			Manager::new()
		});

		SINGLETON.get().unwrap()
	}

	pub fn setConfPath(&self, path : &str)
	{
		self.setConfPathString(String::from(path));
	}

	pub fn setConfPathString(&self, path : String)
	{
		let mut tmp= self.confpath.write().unwrap();
		*tmp = Ok(path);
	}

	pub fn getConfPath(&self) -> Result<String>
	{
		match self.confpath.read().unwrap().as_ref()
		{
			Ok(path) => Ok(path.clone()),
			Err(err) => Err(anyhow!(err.to_string()))
		}
	}

	pub fn get(&self, name: &str) -> Result<config>
	{
		if !self.loadedConfs.read().unwrap().contains_key(name)
		{
			let lock = &mut self.loadedConfs.write().unwrap();
			if !lock.contains_key(name)
			{
				let mut basepath = self.getConfPath()?;
				basepath.push_str("/");
				basepath.push_str(name);
				basepath.push_str(".json");
				let configC = config::new(basepath).unwrap();
				lock.insert(name.to_string(), configC);
			}
		}

		let tmp = self.loadedConfs.read().unwrap().get(name).unwrap().clone();
		return Ok(tmp);
	}
}