use std::fs::{File, rename};
use std::io::Read;
use std::fmt;
use std::time::{SystemTime, UNIX_EPOCH};
use json::JsonValue;
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
			datas: json::parse(tmp.as_str()).unwrap(),
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
		self.datas = json::parse(tmp.as_str())
			.map_err(|e| Errors::ConfigCannotConvertFileToJsonValue(self.name.clone(), self.path.clone(), e))?;
		return Ok(());
	}
	
	
	/// save content into file
	pub fn save(&self) -> Result<bool, Errors>
	{
		let tmppath = format!("{}_{}", self.path, SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().as_nanos().to_string());
		let Rfile = File::create(&tmppath);
		self.datas.write_pretty(&mut Rfile.unwrap(), 4)
			.map_err(|e| Errors::ConfigCannotSaveFile(self.name.clone(), self.path.clone(), e))?;
		
		rename(tmppath, self.path.clone())
			.map_err(|e| Errors::ConfigCannotSaveFile(self.name.clone(), self.path.clone(), e))?;
		return Ok(true);
	}
	
	/// get content from a path (unsigned int for array)
	pub fn get(&self, path: &str) -> Option<JsonValue>
	{
		let splitedPath: Vec<&str> = path.split("/").collect();
		return get_recursive(splitedPath, 0, &self.datas);
	}
	
	/// set content to a path (unsigned int for array)
	pub fn set(&mut self, path: &str, mut howToUpdate: impl FnMut(&mut JsonValue))
	{
		let splitedPath: Vec<&str> = path.split("/").collect();
		if let Some(oldvalue) = set_recursive(splitedPath, 0, &mut self.datas)
		{
			howToUpdate(oldvalue);
		}
	}
	
	/// set content to a path (unsigned int for array)
	pub fn get_mut<'a,'b>(&'b mut self, path: &str) -> Option<&'a mut JsonValue>
		where 'b: 'a
	{
		let splitedPath: Vec<&str> = path.split("/").collect();
		set_recursive(splitedPath, 0, &mut self.datas)
	}
}

impl fmt::Display for HConfig {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
	{
		write!(f, "{}", self.datas)
	}
}

fn get_recursive(splintedPath: Vec<&str>, i: usize, parent: &JsonValue) -> Option<JsonValue>
{
	let thisdir = splintedPath[i];
	if (parent.is_array())
	{
		let tryingint: usize = thisdir.parse::<usize>().unwrap();
		if (tryingint >= parent.len())
		{
			return None;
		} else if (i + 1 < splintedPath.len())
		{
			return get_recursive(splintedPath, i + 1, &parent[tryingint]);
		}
		return Some(parent[tryingint].clone());
	} else {
		if (!parent.has_key(thisdir))
		{
			return None;
		} else if (i + 1 < splintedPath.len())
		{
			return get_recursive(splintedPath, i + 1, &parent[thisdir]);
		}
		
		return Some(parent[thisdir].clone());
	}
}

fn set_recursive<'a>(splintedPath: Vec<&str>, i: usize, parent: &'a mut JsonValue) -> Option<&'a mut JsonValue>
{
	let thisdir = splintedPath[i];
	//println!("dir : {}",thisdir);
	//println!( "{}", parent);
	if (parent.is_array())
	{
		let tryingint: usize = thisdir.parse::<usize>().unwrap();
		if (tryingint < parent.len())
		{
			return None;
		} else if (i + 1 < splintedPath.len())
		{
			return set_recursive(splintedPath, i + 1, &mut parent[tryingint]);
		}
		
		return Some(&mut parent[tryingint]);
	} else {
		if (!parent.has_key(thisdir))
		{
			parent[thisdir] = JsonValue::new_object();
		}
		
		if (i + 1 < splintedPath.len())
		{
			return set_recursive(splintedPath, i + 1, &mut parent[thisdir]);
		}
		return Some(&mut parent[thisdir]);
	}
}
