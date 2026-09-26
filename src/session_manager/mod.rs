use crate::session::{ Session, SessionError };
use serde_json::{ Map, Value, json};
use serde::Deserialize;
use core::str;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpStream;
use tokio::sync::Mutex;
use uuid::Uuid;

use std::fmt;

#[derive(Deserialize, Debug)]
#[serde(untagged)]
enum Payload {
    List(Vec<String>),
    Dict(Map<String, Value>),
    None,
}

pub enum RequestError {
    SessionError,
    ValueError,
    InvalidCommand,
}

#[derive(Deserialize, Debug)]
struct Request {
    cmd: Option<String>,
    payload: Payload,
}


impl fmt::Display for Request {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.cmd.as_ref() {
            Some(cmd) => write!(f, "{}", cmd),
            None => write!(f, "")
        }
    }
}

pub struct SessionManager {
    pub sessions: Arc<Mutex<HashMap<String, Session>>>,
    // Tasks are not owned by the session manager, but by the sessions themselves
}

impl SessionManager {
    pub fn new() -> SessionManager {
        SessionManager {
            sessions: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub async fn add_socket(&self, stream: TcpStream) {
        let session_id = Uuid::new_v4().to_string();
        let session = Session {
            id: session_id.clone(),
            stream: Some(stream),
            data: Some(Map::new()),
        };

        self.sessions
            .lock()
            .await
            .insert(session_id.clone(), session);

        let sessions_clone = Arc::clone(&self.sessions);

        tokio::spawn(async move {
            Self::run_session(sessions_clone, session_id).await;
        });
    }

    async fn run_session(sessions: Arc<Mutex<HashMap<String, Session>>>, session_id: String) {
        let stream = {
            let mut guard = sessions.lock().await;
            if let Some(session) = guard.get_mut(&session_id) {
                session.stream.take()
            } else {
                None
            }
        };
        let Some(stream) = stream else { return };

        let (reader, mut writer) = tokio::io::split(stream);
        let mut buf_reader = BufReader::new(reader);
        let mut line = String::new(); // Bytes

        // Processing loop
        loop {
            line.clear();
            match buf_reader.read_line(&mut line).await {
                Ok(0) => break,
                Ok(_) => {
                    match Self::parse_request(line.trim_end().to_owned()).await {
                        Ok(request) => {
                            let response: String = {
                                let mut guard = sessions.lock().await;

                                match guard.get_mut(&session_id) {
                                    Some(session) => Self::handle_request(session, request).await,
                                    None => break
                                }
                            };
                    
                            if writer.write_all(format!("{response}\n").as_bytes()).await.is_err() { break }
                        },
                        Err(e) => {
                            eprintln!("parse error: {e}; input: {line:?}");
                            break;
                        }
                    };
                }
                Err(_) => break,
                }
            }
            sessions.lock().await.remove(&session_id);
        }
        

    async fn parse_request(raw: String) -> Result<Request, serde_json::Error> {
        serde_json::from_str::<Request>(&raw)
    }


    async fn handle_request(session: &mut Session, request: Request) -> String {
        
        let Some(command) = request.cmd.as_deref() else {
            return json!({"error": "Command cannot be None!"}).to_string();
        };

        println!("Session {} received command {}", &session.id, &command);

        let payload: &Payload = &request.payload;

        let response: Map<String, Value> = match command.trim() {
            
            // CMD = get
            // May return an error if:
            //   - session data wasn't previously initialized
            "get" => {

                let result: Result<Map<String, Value>, SessionError> = session.get_data();

                match result {
                    Ok(data) => data,
                    Err(e) => {
                        Map::from_iter([
                            ("error".to_string(), json!(format!("{:?}", e)))
                        ])
                    }
                }
            },

            // CMD = get_partial -> Requires payload: Vec<String>
            // May return an error if:
            //   - payload cannot be parsed to type Vec<String>
            //   - session data wasn't previously initialized
            "get_partial" => {
                if let Payload::List(keys) = payload {
                    let result = session.get_partial_data(keys);

                    match result {
                        Ok(data) => data,
                        Err(e) => {
                            Map::from_iter([
                                ("error".to_string(), json!(format!("{:?}", e)))
                            ])
                        }
                    }
                } else {
                    Map::from_iter([
                        ("error".to_string(), json!("get_partial requires a list of keys"))
                    ])
                }
            },

            // CMD = set -> Requires data: Map<String, Value>
            // May return an error if:
            //   - payload cannot be parsed to type Map<String, Value>
            "set" => {
                if let Payload::Dict(data) = payload {
                    session.set_data(Some(data.clone()));
                    
                    Map::from_iter([
                        ("result".to_string(), json!("ok"))
                    ])
                    
                } else {
                    Map::from_iter([
                        ("error".to_string(), json!("set requires an object"))
                    ])
                }
            },

            // CMD = set_partial -> Requires data: Map<String, Value>
            // May return an error if:
            //   - payload cannot be parsed to type Map<String, Value>
            "set_partial" => {
                if let Payload::Dict(data) = payload {
                    let result = session.set_partial_data(Some(data.clone()));

                    match result {
                        Ok(_) => {
                            Map::from_iter([
                            ("result".to_string(), json!("ok"))
                            ])
                        },
                        Err(e) => {
                            Map::from_iter([
                                ("error".to_string(), json!(format!("{:?}", e)))
                            ])
                        }
                    }
                } else {
                    Map::from_iter([
                        ("error".to_string(), json!("set_partial requires an object"))
                    ])
                }
            },
            "wipeout" => {
                session.wipeout();

                Map::from_iter([
                    ("result".to_string(), json!("ok"))
                    ])
            }
            cmd => {
                Map::from_iter([
                        ("error".to_string(), json!(format!("invalid command: {}", cmd)))
                    ])
            }
        };

        serde_json::to_string(&response).unwrap()
    }
}
