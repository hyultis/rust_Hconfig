
use once_cell::sync::OnceCell;
use std::sync::Arc;
use arc_swap::ArcSwapOption;
use dashmap::DashMap;
use crate::HConfig::HConfig;
use HArcMut::HArcMut;
use crate::Errors;


pub struct HConfigManager
{
	confpath : ArcSwapOption<String>,
	loadedConfs : DashMap<String, HArcMut<HConfig>>
}

static SINGLETON: OnceCell<HConfigManager> = OnceCell::new();

impl HConfigManager
{
	fn new() -> HConfigManager {
		HConfigManager {
			confpath: ArcSwapOption::new(None),
			loadedConfs: DashMap::new()
		}
	}

	pub fn singleton() -> &'static HConfigManager
	{
		SINGLETON.get_or_init(|| {
			HConfigManager::new()
		});

		SINGLETON.get().unwrap()
	}

	pub fn setConfPath(&self, path : &str)
	{
		self.confpath.swap(Some(Arc::new(path.to_string())));
	}


	pub fn getConfPath(&self) -> Option<String>
	{
		return self.confpath.load_full().map(|tmp|tmp.to_string());
	}

	pub fn get(&self, name: &str) -> Result<Arc<HConfig>,Errors>
	{
		return self.get_mut(name).map(|m|m.get());
	}
	
	pub fn get_mut(&self, name: &str) -> Result<HArcMut<HConfig>,Errors>
	{
		if( self.confpath.load().is_none())
		{
			return Err(Errors::ConfigNotSet);
		}
		
		if !self.loadedConfs.contains_key(name)
		{
			let mut basepath = self.getConfPath().unwrap();
			basepath.push_str("/");
			basepath.push_str(name);
			basepath.push_str(".json");
			let newvalue = HConfig::new(name.to_string(),basepath)?;
			let newvalue = HArcMut::new(newvalue);
			self.loadedConfs.insert(name.to_string(), newvalue.clone());
			return Ok(newvalue);
		}
		
		return Ok(self.loadedConfs.get(name).unwrap().clone());
	}
}
