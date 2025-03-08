use std::fmt;
use tinyjson::JsonValue;
use crate::Errors;
use crate::IO::IOwrapper;

#[derive(Debug)]
pub struct HConfig
{
	name: String,
	constructed_fullpath: String,
	wrapper: Box<dyn IOwrapper + 'static>,
}

impl HConfig
{
	/// create new instance and autoload content
	pub fn new<T: IOwrapper + 'static>(name: String, mut path: String) -> Result<HConfig, Errors>
	{
		if(path.ends_with("/"))
		{
			path.pop();
		}

		let fullpath = format!("{}/{}", path, name);
		let wrapper = T::init(&name, &fullpath)?;

		return Ok(HConfig{
			name,
			constructed_fullpath: fullpath,
			wrapper: Box::new(wrapper),
		});
	}

	/// reload content from file
	pub fn file_load(&mut self) -> Result<(), Errors>
	{
		return self.wrapper.file_load();
	}

	/// save content into file
	pub fn file_save(&self) -> Result<(), Errors>
	{
		return self.wrapper.file_save();
	}

	/// Get config extension
	pub fn file_ext<'a>(&self) -> &'a str
	{
		return self.wrapper.file_ext();
	}

	/// get config file path
	pub fn file_path(&self) -> &String {
		return &self.wrapper.file_path();
	}

	/// get root node
	pub fn root_get(&self) -> &JsonValue
	{
		return self.wrapper.root_get();
	}

	/// get content from a path (unsigned int for array)
	pub fn value_get(&self, path: &str) -> Option<JsonValue>
	{
		return self.wrapper.value_get(path.into());
	}

	/// get content from a path (unsigned int for array), or SET default and return it instead
	pub fn value_get_or_set(&mut self, path: &str, default: impl Into<JsonValue>) -> JsonValue
	{
		return self.wrapper.value_get_or_set(path.into(),default.into());
	}

	/// set content to a path (unsigned int for array)
	pub fn value_get_mut<'a,'b>(&'b mut self, path: impl Into<String>) -> Option<&'a mut JsonValue>
	where 'b: 'a
	{
		return self.wrapper.value_get_mut(path.into());
	}

	/// set content to a path (unsigned int for array)
	pub fn value_set(&mut self, path: &str, newval: impl Into<JsonValue>)
	{
		return self.wrapper.value_set(path.into(),newval.into());
	}
}

impl fmt::Display for HConfig {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
	{
		return write!(f, "{}", self.wrapper);
	}
}
