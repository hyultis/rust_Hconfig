use std::collections::HashMap;
use rusty_json::base::{JsonValue};

pub struct Utils
{

}


impl Utils
{
	pub fn jsonIntoHashMap(base: JsonValue) -> HashMap<String,String>
	{
		let mut finalreturn = HashMap::new();
		
		if let JsonValue::Object(tmp) = &base
		{
			tmp.iter().for_each(|(key,value)|{
				let value: String = value.parse().unwrap_or("".to_string());
				finalreturn.insert(key.clone(),value);
			});
			return finalreturn;
		}
		
		
		if let JsonValue::Array(tmp) = &base
		{
			let mut i = 0;
			for x in tmp.iter()
			{
				let mut result = "".to_string();
				if let Ok(tmp) = x.parse()
				{
					result = tmp;
				}
				finalreturn.insert(i.to_string(),result);
				i+=1;
			}
		}
		
		return finalreturn;
	}
}
