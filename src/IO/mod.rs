use std::fmt::{Debug, Display};
use tinyjson::JsonValue;
use crate::Errors;

/// json format (`WrapperJson`, default format)
pub mod json;

/// A configuration file format. Paths are "/" separated keys, an unsigned int targets an array index,
/// empty segments are ignored.
/// Reading (`value_get`, `value_get_mut`, `value_remove`) never creates anything.
/// Writing (`value_set`, `value_get_or_set`) forces the path : missing keys are created,
/// an out of bounds array index grows the array (filled with null), a scalar on the path is replaced by an object.
pub trait IOwrapper: Display + Debug + Sync + Send {

	/// init instance and autoload content
	fn init(name: &str, path: &str) -> Result<Self, Errors> where Self: Sized;

	/// reload content from a file (called one time by init if file exist)
	fn file_load(&mut self) -> Result<(), Errors>;

	/// save content into a file
	fn file_save(&self) -> Result<(), Errors>;

	/// Get config extension
	fn file_ext(&self) -> &'static str;

	/// get file path
	fn file_path(&self) -> &str;


	/// get root node
	fn root_get(&self) -> &JsonValue;

	/// get mutable root node
	fn root_get_mut(&mut self) -> &mut JsonValue;


	/// get content from a path, None if the path does not exist
	fn value_get(&self, path: &str) -> Option<JsonValue>;

	/// get content from a path, or SET the default value (forcing the path) and return it instead
	fn value_get_or_set(&mut self, path: &str, default: JsonValue) -> JsonValue;

	/// get mutable content from a path, None if the path does not exist
	fn value_get_mut(&mut self, path: &str) -> Option<&mut JsonValue>;

	/// remove content from a path, false if the path does not exist
	fn value_remove(&mut self, path: &str) -> bool;

	/// set content to a path, forcing the path to exist
	fn value_set(&mut self, path: &str, newval: JsonValue);
}
