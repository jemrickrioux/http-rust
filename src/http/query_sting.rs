use std::collections::HashMap;


pub struct QueryString<'buf> {
    data: HashMap<&'buf str, Value<'buf>>

}

pub enum Value<'buf> {
    Single(&'buf str),
    Multiple(Vec<&'buf str>),
}

impl <'buf> QueryString<'buf> {
    pub fn get(&self, key: &str) -> Option<&Value> {
        self.data.get(key)
    }
}

//a=1&b=3&c=454&d=&e===&d=7&d=abc

impl<'buf> From<&'buf str> for QueryString<'buf> {
    fn from(s: &'buf str) -> Self {
        let mut data = HashMap::new();
        for sub_str in s.split('&') {
            let mut key = sub_str;
            let mut val = "";
            if let Some(i) = sub_str.find('=') {
                key = &sub_str[..i];
                val = &sub_str[i+1..];
            }

            // if key exist in Hashmap, we add the 2 parameteers in an array
            // If key exit in Hashmap and it is a vec, we add it to the vector
            data.entry(key)
                .and_modify(|existing:&mut Value | match existing {
                    Value::Single(prev_val)=> {
                       *existing = Value::Multiple(vec![prev_val, val]);
                    },
                    Value::Multiple(vec) => vec.push(val)
                }).or_insert(Value::Single(val));
        }
        QueryString {data}
    }
}