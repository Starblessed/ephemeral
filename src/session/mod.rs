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
    pub fn new() -> Session {
        Session { id:String::from("TODO"), data:None }
    }

    pub fn set_data<T>(&mut self, data: T)
    where
        T: Into<Option<Map<String, Value>>>,
    {
        self.data = data.into();
    }

    pub fn set_partial_data<T>(&mut self, data: T) -> Result<(), SessionError>
    where
        T: Into <Option<Map<String, Value>>>,
    {
        let incoming = data.into();

        let target = self
            .data
            .as_mut()
            .ok_or(SessionError::NoDataPresent)?;
        
        if let Some(new_data) = incoming {
            target.extend(new_data);
        }

        Ok(())
    }
}