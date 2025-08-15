use std::fs::{File, rename};
use std::io::Read;
use std::fmt;
use std::time::{SystemTime, UNIX_EPOCH};
use tinyjson::JsonValue;
use crate::Errors;
use crate::IO::IOwrapper;

#[derive(Debug)]
pub struct WrapperJson
{
	name: String,
	path: String,
	datas: JsonValue,
}

impl WrapperJson
{
	fn file_get(&self, suffix: String) -> Result<(File, String), Errors>
	{
		let tmp_path = format!("{}{}", self.path, suffix);
		let file;
		if let Ok(tmp) = File::open(&tmp_path)
		{
			file = tmp;
		} else {
			file = File::create(&tmp_path).map_err(|err| Errors::ConfigCannotCreateFile(self.name.clone(), tmp_path.clone(), err))?;
		}
		return Ok((file,tmp_path));
	}

	fn file_content_get(&self) -> Result<String, Errors>
	{

		let (mut file,_) = self.file_get("".to_string())?;

		let mut content = String::new();
		if (file.read_to_string(&mut content).is_err() || content.is_empty())
		{
			content = "{}".to_string();
		}

		return Ok(content);
	}
}

impl IOwrapper for WrapperJson
{
	/// create a new instance and autoload content
	fn init(name: &String, path: &String) -> Result<WrapperJson, Errors>
	{
		let path = format!("{}.{}", path, "json");
		let mut tmp = WrapperJson
		{
			name: name.clone(),
			path: path.clone(),
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
	
	
	/// save content into a file
	fn file_save(&self) -> Result<(), Errors>
	{
		let (mut tmp_file, path) = self.file_get(format!("_{}", SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().as_nanos().to_string()))?;

		self.datas.format_to(&mut tmp_file)
			.map_err(|e| Errors::ConfigCannotSaveFile(self.name.clone(), self.path.clone(), e))?;

		rename(&path, self.path.clone())
			.map_err(|e| Errors::ConfigCannotSaveFile(self.name.clone(), self.path.clone(), e))?;
		return Ok(());
	}

	/// Get config extension
	fn file_ext<'a>(&self) -> &'a str {
		return "json";
	}

	/// get file path
	fn file_path(&self) -> &String {
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
	fn value_get(&self, path: String) -> Option<JsonValue>
	{
		let splitedPath: Vec<String> = path.split("/").map(|s| s.to_string()).collect();
		return get_recursive(splitedPath, 0, &self.datas);
	}

	/// get content from a path (use an unsigned int for an array), or SET default and return it instead
	fn value_get_or_set(&mut self, path: String, default: JsonValue) -> JsonValue
	{
		let splitedPath: Vec<String> = path.split("/").map(|s| s.to_string()).collect();
		
		let Some(value) = get_recursive(splitedPath.clone(), 0, &self.datas) else {
			if let Some(defaultset) = set_recursive(splitedPath,0,&mut self.datas)
			{
				*defaultset = default.clone();
			}
			return default;
		};

		return value;
	}

	/// set content to a path (use an unsigned int for an array)
	fn value_get_mut<'a,'b>(&'b mut self, path: String) -> Option<&'a mut JsonValue>
	where 'b: 'a
	{
		let splitedPath: Vec<String> = path.split("/").map(|s| s.to_string()).collect();
		set_recursive(splitedPath, 0, &mut self.datas)
	}
	
	/// set content to a path (use an unsigned int for an array)
	fn value_set(&mut self, path: String, newval: JsonValue)
	{
		let splitedPath: Vec<String> = path.split("/").map(|s| s.to_string()).collect();
		if let Some(oldvalue) = set_recursive(splitedPath, 0, &mut self.datas)
		{
			*oldvalue = newval.into();
		}
	}
}

impl fmt::Display for WrapperJson {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
	{
		match &self.datas {
			JsonValue::Number(x) => write!(f, "{}", x),
			JsonValue::Boolean(x) => write!(f, "{}", x),
			JsonValue::String(x) => write!(f, "{}", x),
			JsonValue::Null => write!(f, "null"),
			JsonValue::Array(x) => write!(f, "{:#?}", x),
			JsonValue::Object(x) => write!(f, "{:#?}", x),
		}
	}
}

fn get_recursive(splintedPath: Vec<String>, i: usize, parent: &JsonValue) -> Option<JsonValue>
{
	let Some(thisdir) = splintedPath.get(i) else
	{
		return None;
	};

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

fn set_recursive(splintedPath: Vec<String>, i: usize, parent: &mut JsonValue) -> Option<&mut JsonValue>
{
	let Some(thisdir) = splintedPath.get(i) else
	{
		return None;
	};
	
	match parent {
		JsonValue::Object(parentAsObject) => {
			if (!parentAsObject.contains_key(thisdir))
			{
				parentAsObject.insert(thisdir.clone(),JsonValue::Object(Default::default()));
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
