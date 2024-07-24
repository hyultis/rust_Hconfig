use std::fs::{File, rename};
use std::io::{Read, Write};
use std::fmt;
use std::time::{SystemTime, UNIX_EPOCH};
use rusty_json::base::{JsonObject, JsonValue};
use rusty_json::extra::{JsonFormatter, JsonParser};
use crate::Errors;

#[derive(Clone, Debug)]
pub struct HConfig
{
	name: String,
	path: String,
	datas: JsonValue,
}

impl HConfig
{
	/// create new instance and autoload content
	pub fn new(name: String, path: String) -> Result<HConfig, Errors>
	{
		let mut file;
		if let Ok(tmp) = File::open(&path)
		{
			file = tmp;
		} else {
			file = File::create(&path).map_err(|err| Errors::ConfigCannotCreateFile(name.clone(), path.clone(), err))?;
		}
		
		let mut tmp = String::new();
		if (file.read_to_string(&mut tmp).is_err() || tmp.is_empty())
		{
			tmp = "{}".to_string();
		}
		
		let mut tmp = HConfig
		{
			name,
			path: path.clone(),
			datas: JsonParser::parse(tmp.as_str()).unwrap_or(JsonValue::Object(JsonObject::new())),
		};
		tmp.reload()?;
		return Ok(tmp);
	}
	
	/**
	 * reload content from file
	 */
	pub fn reload(&mut self) -> Result<(), Errors>
	{
		//println!("load config file path : {}",self.path);
		let mut file = File::open(self.path.clone())
			.map_err(|e| Errors::ConfigCannotCreateFile(self.name.clone(), self.path.clone(), e))?;
		let mut tmp = String::new();
		if (file.read_to_string(&mut tmp).is_err() || tmp.is_empty())
		{
			tmp = "{}".to_string();
		}
		self.datas = JsonParser::parse(tmp.as_str())
			.map_err(|e| Errors::ConfigCannotConvertFileToJsonValue(self.name.clone(), self.path.clone(), e))?;
		return Ok(());
	}
	
	
	/// save content into file
	pub fn save(&self) -> Result<String, Errors>
	{
		let tmppath = format!("{}_{}", self.path, SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().as_nanos().to_string());
		let mut Rfile = File::create(&tmppath)
			.map_err(|e| Errors::ConfigCannotSaveFile(self.name.clone(), self.path.clone(), e))?;
		
		let formatter = JsonFormatter::default();
		Rfile.write_all(formatter.format(&self.datas).as_bytes())
			.map_err(|e| Errors::ConfigCannotSaveFile(self.name.clone(), self.path.clone(), e))?;
		
		rename(&tmppath, self.path.clone())
			.map_err(|e| Errors::ConfigCannotSaveFile(self.name.clone(), self.path.clone(), e))?;
		return Ok(tmppath);
	}
	
	/// get content from a path (unsigned int for array)
	pub fn get(&self, path: impl Into<String>) -> Option<JsonValue>
	{
		let path = path.into();
		let splitedPath: Vec<String> = path.split("/").map(|s| s.to_string()).collect();
		return get_recursive(splitedPath, 0, &self.datas);
	}
	
	/// get content from a path (unsigned int for array), or SET default and return it instead
	pub fn getOrSetDefault(&mut self, path: impl Into<String>, default: JsonValue) -> JsonValue
	{
		let path = path.into();
		let splitedPath: Vec<String> = path.split("/").map(|s| s.to_string()).collect();
		
		return match get_recursive(splitedPath.clone(), 0, &self.datas) {
			None => {
				if let Some(defaultset) = set_recursive(splitedPath,0,&mut self.datas)
				{
					*defaultset = default.clone();
				}
				default
			}
			Some(value) => value
		};
	}
	
	/// get root node
	pub fn getRoot(&self) -> &JsonValue
	{
		return &self.datas;
	}
	
	/// set content to a path (unsigned int for array)
	pub fn set<T>(&mut self, path: impl Into<String>, newval: T)
		where T: Into<JsonValue>
	{
		let path = path.into();
		let splitedPath: Vec<String> = path.split("/").map(|s| s.to_string()).collect();
		if let Some(oldvalue) = set_recursive(splitedPath, 0, &mut self.datas)
		{
			*oldvalue = newval.into();
		}
	}
	
	/// set content to a path (unsigned int for array)
	pub fn get_mut<'a,'b>(&'b mut self, path: impl Into<String>) -> Option<&'a mut JsonValue>
		where 'b: 'a
	{
		let path = path.into();
		let splitedPath: Vec<String> = path.split("/").map(|s| s.to_string()).collect();
		set_recursive(splitedPath, 0, &mut self.datas)
	}
}

impl fmt::Display for HConfig {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
	{
		write!(f, "{}", self.datas)
	}
}

fn get_recursive(splintedPath: Vec<String>, i: usize, parent: &JsonValue) -> Option<JsonValue>
{
	let thisdir = splintedPath.get(i);
	if(thisdir.is_none())
	{
		return None;
	}
	let thisdir = thisdir.unwrap();
	if let JsonValue::Array(parentAsArray) = &parent
	{
		if let Ok(tryingint) = thisdir.parse::<usize>()
		{
			if (tryingint >= parentAsArray.len())
			{
				return None;
			}
			else if (i + 1 < splintedPath.len())
			{
				return get_recursive(splintedPath.clone(), i + 1, parentAsArray.get(tryingint).unwrap());
			}
			return Some(parentAsArray[tryingint].clone());
		}
		return None;
	}
	else if let JsonValue::Object(parentAsObject) = &parent
	{
		if (!parentAsObject.contains_key(thisdir))
		{
			return None;
		}
		else if (i + 1 < splintedPath.len())
		{
			return get_recursive(splintedPath.clone(), i + 1, parentAsObject.get(thisdir).unwrap());
		}
		
		return Some(parentAsObject[thisdir].clone());
	}
	else
	{
		return None;
	}
}

fn set_recursive<'a>(splintedPath: Vec<String>, i: usize, parent: &'a mut JsonValue) -> Option<&'a mut JsonValue>
{
	let thisdir = splintedPath.get(i);
	if(thisdir.is_none())
	{
		return None;
	}
	let thisdir = thisdir.unwrap();
	
	match parent {
		JsonValue::Object(parentAsObject) => {
			if (!parentAsObject.contains_key(thisdir))
			{
				parentAsObject.set(thisdir,JsonValue::Object(JsonObject::new()));
			}
			
			if (i + 1 < splintedPath.len())
			{
				return set_recursive(splintedPath.clone(), i + 1, parentAsObject.get_mut(thisdir).unwrap());
			}
			return parentAsObject.get_mut(thisdir);
		}
		JsonValue::Array(parentAsArray) => {
			if let Ok(tryingint) = thisdir.parse::<usize>()
			{
				if (tryingint < parentAsArray.len())
				{
					return None;
				}
				else if (i + 1 < splintedPath.len())
				{
					return set_recursive(splintedPath.clone(), i + 1, parentAsArray.get_mut(tryingint).unwrap());
				}
				
				return parentAsArray.get_mut(tryingint);
			}
			return None;
		}
		_ => {}
	}
	return None;
}
