use serde_json::{ Map, Value };

#[derive(Debug)]
pub enum SessionError {
    NoDataPresent,
    KeyNotFound(String),
}

struct Session {
    pub id: String,
    pub data: Option<Map<String, Value>>,

}

impl Session {
    fn new() -> Session {
        Session { id:String::from("TODO"), data:None}
    }

    fn set_data<T>(&mut self, data: T) -> ()
    where
        T: Into<Option<Map<String, Value>>>,
    {
        self.data = data.into();
    }
}