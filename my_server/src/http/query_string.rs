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
