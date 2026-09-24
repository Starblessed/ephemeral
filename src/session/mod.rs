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

    fn get_data_ref(&self) -> Result<&Map<String, Value>, SessionError> {
        self.data
            .as_ref()
            .ok_or(SessionError::NoDataPresent)
    }

    pub fn get_data(&self) -> Result<Map<String, Value>, SessionError> {
        Ok(self.get_data_ref()?.clone())
    }

    pub fn get_partial_data(&mut self, keys: &[String]) -> Result<Map<String, Value>, SessionError> {
        let data: &Map<String, Value> = self.get_data_ref()?;

        let missing_keys: Vec<String> = keys
            .iter()
            .filter(|k| data.contains_key(*k))
            .cloned()
            .collect();

        if !missing_keys.is_empty() {
            return Err(SessionError::KeyNotFound(missing_keys))
        }

        let mut result: Map<String, Value> = Map::new();
        for key in keys {
            if let Some(value) = data.get(key) {
                result.insert(key.clone(), value.clone());
            }
        }

        Ok(result)
    }

    pub fn set_data(&mut self, data: Option<Map<String, Value>>) {
        self.data = data;
    }

    pub fn set_partial_data(&mut self, data: Option<Map<String, Value>>) -> Result<(), SessionError> {

        let target = self.data.as_mut().ok_or(SessionError::NoDataPresent)?;
        
        if let Some(new_data) = data {
            target.extend(new_data);
        }

        Ok(())
    }

    pub fn wipeout(&mut self) {
        self.data = None;
    }
  
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::{ json, Map, Value };

    #[test]
    fn test_get_set_data_roundtrip() {
        let mut session = Session::new();

        let data: Map<String, Value> = Map::from_iter([
            ("abc".to_string(), json!(123)),
            ("xyz".to_string(), json!(456)),
        ]);

        session.set_data(Some(data.clone()));

        assert_eq!(session.get_data().unwrap(), data);

    }
}
