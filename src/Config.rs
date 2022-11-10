
use std::fs::File;
use std::io::{Read, Write, stdout};
use std::fmt;
use std::sync::{RwLock, Arc, RwLockReadGuard};
use json::JsonValue;
use anyhow::{Result, anyhow};
use std::borrow::{Borrow, BorrowMut};
use std::ops::{Deref, IndexMut};
use std::ptr::null;
use std::str::Split;
use json::number::Number;

#[derive(Clone,Debug)]
pub struct Config
{
	path : String,
	datas : Option<Arc<RwLock<JsonValue>>>
}

impl Config
{
	/// create new instance and autoload content
	pub fn new(path: String) -> Result<Config>
	{
		let mut file = File::open(&path)?;
		let mut tmp = String::new();
		file.read_to_string(&mut tmp)?;
		
		//let tmp = json::parse(tmp.as_str())?;

		let mut tmp = Config
		{
			path : path.clone(),
			datas : None
		};
		tmp.reload()?;
		return anyhow::Ok(tmp);
	}
	
	/**
	 * reload content from file
	*/
	pub fn reload(&mut self) -> Result<()>
	{
		println!("load config file path : {}",self.path);
		let mut file = File::open(self.path.clone())?;
		let mut tmp = String::new();
		file.read_to_string(&mut tmp)?;
		let tmp = json::parse(tmp.as_str())?;
		self.datas = Some(Arc::new(RwLock::new(tmp)));
		return anyhow::Ok(());
	}
	
	
	/// save content into file
	pub fn save(&self) -> Result<bool>
	{
		println!("save config file path : {}",self.path);
		
		let tmp = "./config/test2.json";
		println!("save config file path : {}",tmp);
		let Rfile = File::create(tmp);
		let contentToSAve = self.datas.as_ref().unwrap().read().unwrap();
		contentToSAve.write_pretty(&mut Rfile.unwrap(),4)?;
		
		return anyhow::Ok(true);
	}
	
	/// draw content into STDOUT
	pub fn drawContent(&self)
	{
		let tmp = self.datas.as_ref().unwrap().read().unwrap();
		println!("Json content : ");
		//tmp.write_pretty(stdout().borrow_mut(), 4).unwrap();
		
		println!( "{}", tmp);
		println!();
	}

	/// get content from a path (unsigned int for array)
	pub fn get(&self, path: &str) -> Option<JsonValue>
	{
		let tmp = self.datas.as_ref().unwrap().read();
		if(tmp.is_err())
		{
			return None;
		}
		
		let splintedPath : Vec<&str> = path.split("/").collect();
		
		return self.get_recursive(splintedPath, 0, &tmp.unwrap());
	}
	
	fn get_recursive(&self, splintedPath:Vec<&str>, i:usize, parent:&JsonValue) -> Option<JsonValue>
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
				return self.get_recursive(splintedPath, i+1, &parent[tryingint]);
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
				return self.get_recursive(splintedPath, i+1, &parent[thisdir]);
			}
			
			return Some(parent[thisdir].clone());
		}
	}

	/// set content string to a path (unsigned int for array)
	pub fn set(&self, path:&str, newvalue:String)
	{
		let tmp = self.datas.as_ref().unwrap().write();
		if (tmp.is_err())
		{
			return;
		}
		let splintedPath: Vec<&str> = path.split("/").collect();
		match self.set_recursive(splintedPath, 0, &mut tmp.unwrap())
		{
			Some(oldvalue) => *oldvalue = JsonValue::String(newvalue),
			None => {
			}
		}
	}
	
	/// set content number (integer/unsigned/etc) to a path (unsigned int for array)
	pub fn setInt<T>(&self, path:&str, newvalue:T) where T: Into<Number>
	{
		let newvalueC: Number = newvalue.into();
		let tmp = self.datas.as_ref().unwrap().write();
		if (tmp.is_err())
		{
			return;
		}
		let splintedPath: Vec<&str> = path.split("/").collect();
		match self.set_recursive(splintedPath, 0, &mut tmp.unwrap())
		{
			Some(oldvalue) => *oldvalue = JsonValue::Number(newvalueC),
			None => {
			}
		}
	}
	
	
	/// set content JsonValue to a path (unsigned int for array)
	pub fn setJson(&self, path:&str, newvalue:JsonValue)
	{
		let tmp = self.datas.as_ref().unwrap().write();
		if (tmp.is_err())
		{
			return;
		}
		let splintedPath: Vec<&str> = path.split("/").collect();
		match self.set_recursive(splintedPath, 0, &mut tmp.unwrap())
		{
			Some(oldvalue) => *oldvalue = newvalue,
			None => {
			}
		}
	}
	
	fn set_recursive<'a>(&self, splintedPath:Vec<&str>, i:usize, parent:&'a mut JsonValue) -> Option<&'a mut JsonValue>
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
				return self.set_recursive(splintedPath, i+1, &mut parent[tryingint]);
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
				return self.set_recursive(splintedPath, i+1, &mut parent[thisdir]);
			}
			
			return Some(&mut parent[thisdir]);
		}
	}
}

impl fmt::Display for Config {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
	{
		write!(f, "{}", self.datas.as_ref().unwrap().read().unwrap())
	}
}
