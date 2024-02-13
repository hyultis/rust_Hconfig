use std::sync::{Arc, OnceLock};
use arc_swap::ArcSwap;
use dashmap::DashMap;
use crate::HConfig::HConfig;
use crate::guard::Guard;


pub struct HConfigManager
{
	confpath: ArcSwap<String>,
	loadedConfs: DashMap<String, HConfig>,
}

static SINGLETON: OnceLock<HConfigManager> = OnceLock::new();

impl HConfigManager
{
	pub fn singleton() -> &'static HConfigManager
	{
		SINGLETON.get_or_init(|| {
			HConfigManager {
				confpath: ArcSwap::new(Arc::new("./".to_string())),
				loadedConfs: DashMap::new(),
			}
		})
	}
	
	pub fn setConfPath(&self, path: impl Into<String>)
	{
		let path = path.into();
		self.confpath.swap(Arc::new(path));
	}
	
	pub fn getConfPath(&self) -> String
	{
		return (&**self.confpath.load()).clone();
	}
	
	pub fn get(&self, name: impl Into<String>) -> Guard<'_>
	{
		let name = name.into();
		let containkey = {self.loadedConfs.contains_key(&name)};
		if !containkey
		{
			let mut basepath = self.getConfPath();
			basepath.push_str("/");
			basepath.push_str(&name);
			basepath.push_str(".json");
			let newvalue = HConfig::new(name.to_string(), basepath).expect(format!("Error HConfigManager on '{}'",&name).as_str());
			
			self.loadedConfs.insert(name.clone(), newvalue);
		}
		
		return Guard{
			context: self,
			guarded: self.loadedConfs.get_mut(&name).expect(format!("Error HConfigManager on '{}' : this error is not normally possible",&name).as_str()),
		};
	}
}
