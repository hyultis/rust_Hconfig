use std::collections::HashMap;
use json::JsonValue;

pub struct Utils
{

}


impl Utils
{
	pub fn jsonIntoHashMap(base: JsonValue) -> HashMap<String,String>
	{
		let mut finalreturn = HashMap::new();
		
		if(!base.is_object())
		{
			return finalreturn;
		}
		
		for tmp in base.entries()
		{
			let data = tmp.1;
			finalreturn.insert(tmp.0.to_string(),data.to_string());
		}
		
		return finalreturn;
	}
}
