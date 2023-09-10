// Storing query parameters

use std::collections::HashMap;

pub struct QueryString<'buf> {
    // &str'buf str won't be complaining about lifetime reference
    // until this module is know by the compiler
    data: HashMap<&'buf str, Value<'buf>>,
}

// Query parameter type
// Eg: ?key=value&key2=value2 => key and key2 are Single
// Eg: ?key=value&key=value2 => key is Multiple
pub enum Value<'buf> {
    Single(&'buf str),
    Multiple(Vec<&'buf str>),
}

impl<'buf> QueryString<'buf> {
    pub fn get(&self, key: &str) -> Option<&Value> {
        self.data.get(key)
    }
}

// Why not use TryFrom?
// Because this should not fail
// Borrowed input &str
impl<'buf> From<&'buf str> for QueryString<'buf> {
    fn from(s: &'buf str) -> QueryString<'buf> {
        // Create empty HashMap
        let mut data = HashMap::new();

        for parameter_string in s.split('&') {
            // We will assign parameter_string to key
            // incase there is no '=' in the parameter_string
            let mut key = parameter_string;
            let mut value = "";

            if let Some(i) = parameter_string.find('=') {
                key = &parameter_string[..i];
                value = &parameter_string[i + 1..];
            }

            // If key already exist in HashMap
            data.entry(key)
                // we will append the value to the existing key
                .and_modify(|existing| match existing {
                    Value::Single(prev_value) => {
                        let vec = vec![value, prev_value];
                        // Swap memory address in existing to new Value::Multiple
                        *existing = Value::Multiple(vec);
                    }
                    Value::Multiple(vec) => vec.push(value),
                })
                // else we will create a new key with the value
                .or_insert(Value::Single(value));
        }

        QueryString { data: data }
    }
}
