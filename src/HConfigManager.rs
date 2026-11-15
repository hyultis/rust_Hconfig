use std::collections::HashMap;
use std::sync::{Arc, OnceLock, PoisonError, RwLock};
use crate::Errors;
use crate::HConfig::HConfig;
use crate::IO::IOwrapper;

/// holds the configs directory and every loaded config, shared by the whole program through `singleton()`
pub struct HConfigManager
{
	confpath: RwLock<String>,
	loadedConfs: RwLock<HashMap<String, Arc<RwLock<HConfig>>>>,
}

static SINGLETON: OnceLock<HConfigManager> = OnceLock::new();

impl HConfigManager
{
	/// get the manager instance (the configs directory defaults to "./")
	pub fn singleton() -> &'static HConfigManager
	{
		SINGLETON.get_or_init(|| {
			HConfigManager {
				confpath: RwLock::new("./".to_string()),
				loadedConfs: RwLock::new(HashMap::new()),
			}
		})
	}

	/// set path where fetch and storing configs
	pub fn confPath_set(&self, path: impl Into<String>)
	{
		*self.confpath.write().unwrap_or_else(PoisonError::into_inner) = path.into();
	}

	/// retrieve configs path
	pub fn confPath_get(&self) -> String
	{
		return self.confpath.read().unwrap_or_else(PoisonError::into_inner).clone();
	}

	/// load (or create) the config "name" with the wrapper T.
	/// If a config with this name is already loaded, its content is replaced by a fresh load of the file :
	/// unsaved changes are lost, handles already returned by `get` see the new content.
	/// Do not call it while holding a lock on this config in the same thread (deadlock).
	pub fn create<T: IOwrapper + 'static>(&self, name: impl Into<String>) -> Result<(), Errors>
	{
		let name = name.into();
		let newvalue = HConfig::new::<T>(&name, &self.confPath_get())?;

		let mut loadedConfs = self.loadedConfs.write().unwrap_or_else(PoisonError::into_inner);
		let Some(existing) = loadedConfs.get(&name).cloned() else
		{
			loadedConfs.insert(name, Arc::new(RwLock::new(newvalue)));
			return Ok(());
		};
		drop(loadedConfs);

		*existing.write().unwrap_or_else(PoisonError::into_inner) = newvalue;
		return Ok(());
	}

	/// get a shared handle on a loaded config, use `read()` or `write()` on it to access the content
	pub fn get(&self, name: &str) -> Option<Arc<RwLock<HConfig>>>
	{
		return self.loadedConfs.read().unwrap_or_else(PoisonError::into_inner).get(name).cloned();
	}
}
