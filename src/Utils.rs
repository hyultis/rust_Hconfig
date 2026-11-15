use std::collections::HashMap;
use tinyjson::JsonValue;

/// Convert a JsonValue (Object or Array) into Hashmap
/// Strings are kept as is, other values are converted to their json text.
/// If the JsonValue is not a Object or Array, Hashmap will be empty
pub fn jsonIntoHashMap(base: JsonValue) -> HashMap<String, String>
{
	let mut finalreturn = HashMap::new();

	if let JsonValue::Object(tmp) = &base
	{
		tmp.iter().for_each(|(key, value)| {
			finalreturn.insert(key.clone(), jsonToString(value));
		});
		return finalreturn;
	}

	if let JsonValue::Array(tmp) = &base
	{
		let mut i = 0;
		for x in tmp.iter()
		{
			finalreturn.insert(i.to_string(), jsonToString(x));
			i += 1;
		}
	}

	return finalreturn;
}

/// a string is returned as is, any other value as json text (empty if it cannot be generated)
fn jsonToString(value: &JsonValue) -> String
{
	if let JsonValue::String(text) = value
	{
		return text.clone();
	}
	return value.stringify().unwrap_or_default();
}
