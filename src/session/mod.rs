use serde_json::{ Map, Value };

#[derive(Debug)]
pub enum SessionError {
    NoDataPresent,
    KeyNotFound(Vec<String>),
}

struct Session {
    pub id: String,
    pub data: Option<Map<String, Value>>,
}

impl Session {
    pub fn new() -> Session {
        Session { id: String::from("TODO"), data: None }
    }

    fn _get_data(&mut self) -> Result<&mut Map<String, Value>, SessionError> {
        self.data
            .as_mut()
            .ok_or(SessionError::NoDataPresent)
    }

    pub fn get_data(&mut self) -> Result<Map<String, Value>, SessionError> {
        match self._get_data() {
            Err(err) => {
                Err(err)
            },
            Ok(data) => {
                Ok(data.clone())
            }
        }
    }

    pub fn get_partial_data(&mut self, keys: &[String]) -> Result<Map<String, Value>, SessionError> {
        let data: &mut Map<String, Value> = self._get_data()?;

        let mut result: Map<String, Value> = Map::new();

        let orphan_keys: Vec<String> = keys
            .iter()
            .filter(|k| data.contains_key(*k))
            .cloned()
            .collect();

        if !orphan_keys.is_empty() {
            return Err(SessionError::KeyNotFound(orphan_keys))
        }

        for key in keys {
            if let Some(value) = data.get(key) {
                result.insert(key.clone(), value.clone());
            }
        }

        Ok(result)
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

        let target = self._get_data()?;
        
        if let Some(new_data) = incoming {
            target.extend(new_data);
        }

        Ok(())
    }

    pub fn wipeout(&mut self) {
        self.data = None;
    }
  
}
