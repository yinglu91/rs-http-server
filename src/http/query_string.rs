use std::collections::HashMap;

#[derive(Debug)]
pub struct QueryString<'buf> {
    data: HashMap<&'buf str, Value<'buf>>,
}

#[derive(Debug)]
pub enum Value<'buf> {
    Single(&'buf str),
    Multiple(Vec<&'buf str>),
}

impl<'buf> QueryString<'buf> {
    pub fn get(&self, key: &str) -> Option<&Value> {
        return self.data.get(key);
    }
}

// can't fail, so it From, not TryFrom
impl<'buf> From<&'buf str> for QueryString<'buf> {
    // a=1&a=2&c&b=3
    fn from(s: &'buf str) -> Self {
        let mut data = HashMap::new();

        for sub_str in s.split('&') {
            let mut key = sub_str;
            let mut val = "";

            if let Some(i) = sub_str.find('=') {
                key = &sub_str[..i];
                val = &sub_str[i+1..];
            }

            data.entry(key)
                .and_modify(|existing: &mut Value| 
                    match existing {
                        Value::Single(prev_val) => {
                            *existing = Value::Multiple(vec![prev_val, val]);
                        }
                        Value::Multiple(vec) => vec.push(val),
                    })
                .or_insert(Value::Single(val));
        }

        QueryString {data} 
    }
}