use std::fmt;
use tinyjson::JsonValue;
use crate::Errors;
use crate::IO::IOwrapper;

#[derive(Debug)]
pub struct HConfig
{
	name: String,
	wrapper: Box<dyn IOwrapper + 'static>,
}

impl HConfig
{
	/// create a new instance and autoload content
	pub fn new<T: IOwrapper + 'static>(name: &str, path: &str) -> Result<HConfig, Errors>
	{
		let path = path.strip_suffix("/").unwrap_or(path);
		let fullpath = format!("{}/{}", path, name);
		let wrapper = T::init(name, &fullpath)?;

		return Ok(HConfig{
			name: name.to_string(),
			wrapper: Box::new(wrapper),
		});
	}

	/// get the config name (as given to `HConfigManager::create`)
	pub fn name_get(&self) -> &str
	{
		return &self.name;
	}

	/// reload content from the file
	pub fn file_load(&mut self) -> Result<(), Errors>
	{
		return self.wrapper.file_load();
	}

	/// save content into the file
	pub fn file_save(&self) -> Result<(), Errors>
	{
		return self.wrapper.file_save();
	}

	/// Get config extension
	pub fn file_ext(&self) -> &'static str
	{
		return self.wrapper.file_ext();
	}

	/// get file path
	pub fn file_path(&self) -> &str {
		return self.wrapper.file_path();
	}

	/// get root node
	pub fn root_get(&self) -> &JsonValue
	{
		return self.wrapper.root_get();
	}

	/// get mutable root node
	pub fn root_get_mut(&mut self) -> &mut JsonValue
	{
		return self.wrapper.root_get_mut();
	}

	/// get content from a path (use an unsigned int for an array), None if the path does not exist
	pub fn value_get(&self, path: &str) -> Option<JsonValue>
	{
		return self.wrapper.value_get(path);
	}

	/// get content from a path (use an unsigned int for an array), or SET default (forcing the path) and return it instead
	pub fn value_get_or_set(&mut self, path: &str, default: impl Into<JsonValue>) -> JsonValue
	{
		return self.wrapper.value_get_or_set(path, default.into());
	}

	/// get mutable content from a path (use an unsigned int for an array), None if the path does not exist
	pub fn value_get_mut(&mut self, path: &str) -> Option<&mut JsonValue>
	{
		return self.wrapper.value_get_mut(path);
	}

	/// remove content from a path (use an unsigned int for an array), false if the path does not exist
	pub fn value_remove(&mut self, path: &str) -> bool
	{
		return self.wrapper.value_remove(path);
	}

	/// set content to a path (use an unsigned int for an array), forcing the path to exist
	pub fn value_set(&mut self, path: &str, newval: impl Into<JsonValue>)
	{
		return self.wrapper.value_set(path, newval.into());
	}
}

impl fmt::Display for HConfig {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
	{
		return write!(f, "{}", self.wrapper);
	}
}
