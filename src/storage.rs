use anyhow::{bail, Ok, Result};
use std::{
    collections::HashMap,
    ops::Add,
    sync::{Arc, Mutex},
};

use chrono::{DateTime, Duration, Utc};

use crate::command_parser::{Command, Payload};

#[allow(unused)]
#[derive(Default)]
pub struct Value {
    content: Option<String>,
    expiry: Option<DateTime<Utc>>,
}

#[derive(Default)]
pub struct KVStore {
    pub storage: Arc<Mutex<HashMap<String, Value>>>,
}

impl KVStore {
    pub fn new() -> Self {
        KVStore::default()
    }
    pub fn build_response(&self, cmd: Payload) -> Result<String> {
        match cmd.action {
            Command::Ping => Ok(String::from("+PONG\r\n")),
            Command::Echo => {
                let cloned_payload = cmd.payload.clone();
                let mut joined_resp = cloned_payload.unwrap().join("\r\n");
                joined_resp.push_str("\r\n");

                Ok(joined_resp)
            }
            Command::Get => {
                let storage = self.storage.lock().unwrap();
                let arg = cmd.payload.unwrap();
                if arg.len() < 3 {
                    bail!("wrong GET Payload");
                }
                let key = &arg[1];

                println!("GET - {:?}", arg);

                if let Some(val) = storage.get(key) {
                    if let Some(exp) = val.expiry {
                        let current_date_time = Utc::now();
                        if current_date_time.gt(&exp) {
                            return Ok(String::from("_\r\n"));
                        }
                    }

                    if let Some(val) = &val.content {
                        return Ok(val.clone());
                    }
                }

                Ok(String::from("_\r\n"))
            }
            Command::Set => {
                let mut storage = self.storage.lock().unwrap();
                let arg = cmd.payload.unwrap();
                if arg.len() < 4 {
                    bail!("wrong SET payload");
                }

                println!("SET - {:?}", arg);

                let expiry_arg = if arg.len() == 8 {
                    let cloned_dur_str = arg[7].clone();
                    let mut duration_chars = cloned_dur_str.as_str().chars();
                    duration_chars.next();

                    let duration_int = duration_chars.as_str().parse::<i64>()?;
                    let current_date_time = Utc::now();
                    match arg[5].to_uppercase().as_str() {
                        "PX" => Some(current_date_time.add(Duration::milliseconds(duration_int))),
                        "EX" => Some(current_date_time.add(Duration::seconds(duration_int))),
                        _ => None,
                    }
                } else {
                    None
                };

                let key = &arg[1];
                let mut serialized_value = arg[2..=3].join("\r\n");
                serialized_value.push_str("\r\n");
                let val = Value {
                    content: Some(serialized_value),
                    expiry: expiry_arg,
                };

                storage.insert(key.clone(), val);

                Ok(String::from("+OK\r\n"))
            }
        }
    }
}
