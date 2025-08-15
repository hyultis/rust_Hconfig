use std::fmt::{Debug, Display};
use tinyjson::JsonValue;
use crate::Errors;
use crate::IO::json::WrapperJson;

pub mod json;

pub trait IOwrapper: Display + Debug + Sync + Send {

	/// init instance and autoload content
	fn init(name: &String, path: &String) -> Result<WrapperJson, Errors> where Self: Sized;

	/// reload content from a file (called one time by init if file exist)
	fn file_load(&mut self) -> Result<(), Errors>;

	/// save content into a file
	fn file_save(&self) -> Result<(), Errors>;

	/// Get config extension
	fn file_ext<'a>(&self) -> &'a str;

	/// get file path
	fn file_path(&self) -> &String;


	/// get root node
	fn root_get(&self) -> &JsonValue;

	/// get root node
	fn root_get_mut (&mut self) -> &mut JsonValue;


	/// get content from a path (use an unsigned int for an array)
	fn value_get(&self, path: String) -> Option<JsonValue>;

	/// get content from a path (use an unsigned int for an array), or SET the default value and return it instead
	fn value_get_or_set(&mut self, path: String, default: JsonValue) -> JsonValue;

	/// set content to a path (use an unsigned int for an array)
	fn value_get_mut<'a,'b>(&'b mut self, path: String) -> Option<&'a mut JsonValue>
		where 'b: 'a;

	/// set content to a path (use an unsigned int for an array)
	fn value_set(&mut self, path: String, newval: JsonValue);
}