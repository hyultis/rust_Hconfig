use std::sync::{Arc, OnceLock};
use arc_swap::ArcSwap;
use dashmap::DashMap;
use dashmap::mapref::one::RefMut;
use crate::Errors;
use crate::HConfig::HConfig;
use crate::IO::IOwrapper;
use crate::IO::json::WrapperJson;

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

	/// set path where fetch and storing configs
	pub fn confPath_set(&self, path: impl Into<String>)
	{
		let path = path.into();
		self.confpath.swap(Arc::new(path));
	}

	/// retrieve configs path
	pub fn confPath_get(&self) -> String
	{
		return (&**self.confpath.load()).clone();
	}

	/// init
	pub fn create<T: IOwrapper>(&self, name: impl Into<String>) -> Result<(), Errors>
	{
		let name = name.into();
		let newvalue = HConfig::new::<WrapperJson>(name.to_string(), self.confPath_get())
			.expect(format!("Error HConfigManager on '{}'",&name).as_str());

		self.loadedConfs.insert(name.clone(), newvalue);
		return Ok(());
	}

	/// get config
	pub fn get(&self, name: impl Into<String>) -> Option<RefMut<String, HConfig>>
	{
		let name = name.into();
		return self.loadedConfs.get_mut(&name);
	}
}
