use std::fs::{File, remove_file, rename};
use std::io::{ErrorKind, Read, Write};
use std::fmt;
use std::time::{SystemTime, UNIX_EPOCH};
use tinyjson::JsonValue;
use crate::Errors;
use crate::IO::IOwrapper;

/// json format : the config is stored as `<name>.json` in the configs directory
#[derive(Debug)]
pub struct WrapperJson
{
	name: String,
	path: String,
	datas: JsonValue,
}

impl WrapperJson
{
	/// read the config file : a missing file is created empty, an empty file is "{}",
	/// an unreadable file is an error (never replaced by "{}", to not overwrite it on the next save)
	fn file_content_get(&self) -> Result<String, Errors>
	{
		let loadError = |err| Errors::ConfigCannotLoadFile(self.name.clone(), self.path.clone(), err);

		let mut content = String::new();
		match File::open(&self.path)
		{
			Ok(mut file) => {
				file.read_to_string(&mut content).map_err(loadError)?;
			}
			Err(err) if err.kind() == ErrorKind::NotFound => {
				File::create(&self.path).map_err(|err| Errors::ConfigCannotCreateFile(self.name.clone(), self.path.clone(), err))?;
			}
			Err(err) => return Err(loadError(err)),
		}

		if (content.trim().is_empty())
		{
			content = "{}".to_string();
		}
		return Ok(content);
	}
}

impl IOwrapper for WrapperJson
{
	/// create a new instance and autoload content
	fn init(name: &str, path: &str) -> Result<WrapperJson, Errors>
	{
		let mut tmp = WrapperJson
		{
			name: name.to_string(),
			path: format!("{}.{}", path, "json"),
			datas: JsonValue::Object(Default::default()),
		};
		tmp.file_load()?;
		return Ok(tmp);
	}

	/// reload content from a file
	fn file_load(&mut self) -> Result<(), Errors>
	{
		self.datas = self.file_content_get()?.parse()
			.map_err(|e| Errors::ConfigCannotConvertFileToJsonValue(self.name.clone(), self.path.clone(), e))?;

		return Ok(());
	}

	/// save content into a temporary file synced on disk, then rename it over the config file
	fn file_save(&self) -> Result<(), Errors>
	{
		let saveError = |err| Errors::ConfigCannotSaveFile(self.name.clone(), self.path.clone(), err);

		let content = self.datas.format().map_err(|e| saveError(std::io::Error::other(e)))?;
		let tmpPath = format!("{}_{}", self.path, SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().as_nanos());

		let written = File::create(&tmpPath)
			.and_then(|mut file| {
				file.write_all(content.as_bytes())?;
				file.sync_all()
			})
			.and_then(|_| rename(&tmpPath, &self.path));

		if let Err(err) = written
		{
			let _ = remove_file(&tmpPath);
			return Err(saveError(err));
		}
		return Ok(());
	}

	/// Get config extension
	fn file_ext(&self) -> &'static str {
		return "json";
	}

	/// get file path
	fn file_path(&self) -> &str {
		&self.path
	}

	/// get root node
	fn root_get(&self) -> &JsonValue
	{
		return &self.datas;
	}

	/// get mutable root node
	fn root_get_mut(&mut self) -> &mut JsonValue
	{
		return &mut self.datas;
	}

	/// get content from a path (use an unsigned int for an array)
	fn value_get(&self, path: &str) -> Option<JsonValue>
	{
		let segments = path_split(path);
		if (segments.is_empty())
		{
			return None;
		}
		return node_get(&self.datas, &segments).cloned();
	}

	/// get content from a path (use an unsigned int for an array), or SET default and return it instead
	fn value_get_or_set(&mut self, path: &str, default: JsonValue) -> JsonValue
	{
		let segments = path_split(path);
		if let Some(value) = node_get(&self.datas, &segments).filter(|_| !segments.is_empty())
		{
			return value.clone();
		}

		if let Some(node) = node_force(&mut self.datas, &segments)
		{
			*node = default.clone();
		}
		return default;
	}

	/// get mutable content from a path (use an unsigned int for an array), None if the path does not exist
	fn value_get_mut(&mut self, path: &str) -> Option<&mut JsonValue>
	{
		let segments = path_split(path);
		if (segments.is_empty())
		{
			return None;
		}
		return node_get_mut(&mut self.datas, &segments);
	}

	/// remove content from a path (use an unsigned int for an array)
	fn value_remove(&mut self, path: &str) -> bool
	{
		let segments = path_split(path);
		let Some((last, parentPath)) = segments.split_last() else {return false;};
		let Some(parent) = node_get_mut(&mut self.datas, parentPath) else {return false;};

		match parent {
			JsonValue::Array(childs) => {
				if let Ok(last) = last.parse::<usize>()
				{
					if(last >= childs.len()) {return false;}
					childs.remove(last);
					return true;
				}
			}
			JsonValue::Object(childs) => {
				return (childs.remove(*last).is_some());
			}
			_ => {},
		}

		return false;
	}

	/// set content to a path (use an unsigned int for an array)
	fn value_set(&mut self, path: &str, newval: JsonValue)
	{
		if let Some(oldvalue) = node_force(&mut self.datas, &path_split(path))
		{
			*oldvalue = newval;
		}
	}
}

impl fmt::Display for WrapperJson {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
	{
		let content = self.datas.format().map_err(|_| fmt::Error)?;
		return write!(f, "{}", content);
	}
}

/// split a path on "/", ignoring empty segments
fn path_split(path: &str) -> Vec<&str>
{
	return path.split('/').filter(|segment| !segment.is_empty()).collect();
}

/// return the node at the path, without creating anything
fn node_get<'a>(root: &'a JsonValue, segments: &[&str]) -> Option<&'a JsonValue>
{
	let mut node = root;
	for segment in segments
	{
		node = match node
		{
			JsonValue::Object(object) => object.get(*segment)?,
			JsonValue::Array(array) => array.get(segment.parse::<usize>().ok()?)?,
			_ => return None,
		};
	}
	return Some(node);
}

/// return the mutable node at the path, without creating anything
fn node_get_mut<'a>(root: &'a mut JsonValue, segments: &[&str]) -> Option<&'a mut JsonValue>
{
	let mut node = root;
	for segment in segments
	{
		node = match node
		{
			JsonValue::Object(object) => object.get_mut(*segment)?,
			JsonValue::Array(array) => array.get_mut(segment.parse::<usize>().ok()?)?,
			_ => return None,
		};
	}
	return Some(node);
}

/// return the mutable node at the path, forcing the path to exist :
/// a missing key is created, an out of bounds array index grows the array (filled with null),
/// a scalar on the path is replaced by an object.
/// None for an empty path or a non numeric segment on an array.
fn node_force<'a>(root: &'a mut JsonValue, segments: &[&str]) -> Option<&'a mut JsonValue>
{
	if (segments.is_empty())
	{
		return None;
	}

	let mut node = root;
	for segment in segments
	{
		if (!matches!(node, JsonValue::Object(_) | JsonValue::Array(_)))
		{
			*node = JsonValue::Object(Default::default());
		}

		node = match (node, segment.parse::<usize>())
		{
			(JsonValue::Array(array), Ok(index)) => {
				if (index >= array.len())
				{
					array.resize(index + 1, JsonValue::Null);
				}
				&mut array[index]
			}
			(JsonValue::Object(object), _) => object.entry(segment.to_string()).or_insert(JsonValue::Null),
			_ => return None,
		};
	}
	return Some(node);
}
