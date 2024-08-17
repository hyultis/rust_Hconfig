use std::collections::HashMap;
use serde_json::Value as JsonValue;

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
				let value: String = value.as_str().unwrap_or("").to_string();
				finalreturn.insert(key.clone(),value);
			});
			return finalreturn;
		}
		
		
		if let JsonValue::Array(tmp) = &base
		{
			let mut i = 0;
			for x in tmp.iter()
			{
				let result = x.as_str().unwrap_or("").to_string();
				finalreturn.insert(i.to_string(),result);
				i+=1;
			}
		}
		
		return finalreturn;
	}
}
