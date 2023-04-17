
use std::fs::File;
use std::io::Read;
use std::fmt;
use json::JsonValue;
use crate::Errors;

#[derive(Clone,Debug)]
pub struct HConfig
{
	name: String,
	path : String,
	datas : JsonValue
}

impl HConfig
{
	/// create new instance and autoload content
	pub fn new(name: String,path: String) -> Result<HConfig,Errors>
	{
		let mut file = File::open(&path)
			.map_err(|e|Errors::ConfigCannotLoadFile(name.clone(),path.clone(),e))?;
		let mut tmp = String::new();
		file.read_to_string(&mut tmp)
			.map_err(|e|Errors::ConfigCannotConvertFileToJson(name.clone(),path.clone(),e))?;
		
		//let tmp = json::parse(tmp.as_str())?;

		let mut tmp = HConfig
		{
			name,
			path : path.clone(),
			datas : JsonValue::new_object()
		};
		tmp.reload()?;
		return Ok(tmp);
	}
	
	/**
	 * reload content from file
	*/
	pub fn reload(&mut self) -> Result<(),Errors>
	{
		//println!("load config file path : {}",self.path);
		let mut file = File::open(self.path.clone())
			.map_err(|e|Errors::ConfigCannotLoadFile(self.name.clone(),self.path.clone(),e))?;
		let mut tmp = String::new();
		file.read_to_string(&mut tmp)
			.map_err(|e|Errors::ConfigCannotConvertFileToJson(self.name.clone(),self.path.clone(),e))?;
		self.datas = json::parse(tmp.as_str())
			.map_err(|e|Errors::ConfigCannotConvertFileToJsonValue(self.name.clone(),self.path.clone(),e))?;
		return Ok(());
	}
	
	
	/// save content into file
	pub fn save(&self) -> Result<bool,Errors>
	{
		//println!("save config file path : {}",self.path);
		let Rfile = File::create(&self.path);
		self.datas.write_pretty(&mut Rfile.unwrap(),4)
			.map_err(|e|Errors::ConfigCannotSaveFile(self.name.clone(),self.path.clone(),e))?;
		return Ok(true);
	}

	/// get content from a path (unsigned int for array)
	pub fn get(&self, path: &str) -> Option<JsonValue>
	{
		let splitedPath: Vec<&str> = path.split("/").collect();
		return get_recursive(splitedPath, 0, &self.datas);
	}

	/// set content string to a path (unsigned int for array)
	pub fn set(&mut self, path:&str, mut howToUpdate: impl FnMut(&mut JsonValue))
	{
		let splitedPath: Vec<&str> = path.split("/").collect();
		if let Some(oldvalue) = set_recursive(splitedPath, 0, &mut self.datas)
		{
			howToUpdate(oldvalue);
		}
	}
}

impl fmt::Display for HConfig {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
	{
		write!(f, "{}", self.datas)
	}
}

fn get_recursive(splintedPath:Vec<&str>, i:usize, parent:&JsonValue) -> Option<JsonValue>
{
	let thisdir = splintedPath[i];
	//println!("dir : {}",thisdir);
	//println!( "{}", parent);
	if(parent.is_array())
	{
		let tryingint: usize = thisdir.parse().unwrap();
		if(tryingint>=parent.len())
		{
			return None;
		}
		else if (i + 1 < splintedPath.len())
		{
			return get_recursive(splintedPath, i+1, &parent[tryingint]);
		}
		return Some(parent[tryingint].clone());
	}
	else
	{
		if (!parent.has_key(thisdir))
		{
			return None;
		}
		else if (i + 1 < splintedPath.len())
		{
			return get_recursive(splintedPath, i+1, &parent[thisdir]);
		}
		
		return Some(parent[thisdir].clone());
	}
}

fn set_recursive<'a>(splintedPath:Vec<&str>, i:usize, parent:&'a mut JsonValue) -> Option<&'a mut JsonValue>
{
	let thisdir = splintedPath[i];
	//println!("dir : {}",thisdir);
	//println!( "{}", parent);
	if(parent.is_array())
	{
		let tryingint: usize = thisdir.parse().unwrap();
		if(tryingint<parent.len())
		{
			return None;
		}
		else if (i + 1 < splintedPath.len())
		{
			return set_recursive(splintedPath, i+1, &mut parent[tryingint]);
		}
		else
		{
			return Some(&mut parent[tryingint]);
		}
	}
	else
	{
		if (!parent.has_key(thisdir))
		{
			return None;
		}
		else if (i + 1 < splintedPath.len())
		{
			return set_recursive(splintedPath, i+1, &mut parent[thisdir]);
		}
		
		return Some(&mut parent[thisdir]);
	}
}
