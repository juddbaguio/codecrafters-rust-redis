// use anyhow::Result;

use anyhow::{bail, Ok, Result};

#[derive(Debug)]
pub enum Command {
    Ping,
    Echo,
    Get,
    Set,
}

#[derive(Debug)]
pub struct Payload {
    pub action: Command,
    pub payload: Option<Vec<String>>,
}

#[allow(unused)]

impl Command {
    pub fn parse(string_payload: String) -> Result<Payload> {
        let mut splitted_payload = string_payload
            .split("\r\n")
            .map(String::from)
            .collect::<Vec<String>>();
        splitted_payload.pop();

        let mut command: Option<Command> = None;

        while !splitted_payload.is_empty() {
            match splitted_payload.remove(0).as_str().to_uppercase().as_str() {
                "ECHO" => {
                    command = Some(Command::Echo);
                    break;
                }
                "PING" => {
                    command = Some(Command::Ping);
                    break;
                }
                "GET" => {
                    command = Some(Command::Get);
                    break;
                }
                "SET" => {
                    command = Some(Command::Set);
                    break;
                }
                _ => {}
            }
        }

        if let Some(cmd) = command {
            return Ok(Payload {
                action: cmd,
                payload: Some(splitted_payload),
            });
        }

        bail!("Command {} - invalid", string_payload)
    }
}
