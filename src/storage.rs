use anyhow::{bail, Ok, Result};
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};
// use tokio::sync::Mutex;

use crate::command_parser::{Command, Payload};

#[allow(unused)]
#[derive(Default)]
pub struct Value {
    content: Option<String>,
    expiry: Option<String>,
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

                if let Some(val) = storage.get(key) {
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
                let key = &arg[1];
                let val = Value {
                    content: Some(arg[3].clone()),
                    ..Default::default()
                };

                storage.insert(key.clone(), val);

                Ok(String::from("+OK\r\n"))
            }
        }
    }
}
