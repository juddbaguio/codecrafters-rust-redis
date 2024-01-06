// use anyhow::Result;

use anyhow::{bail, ensure, Ok, Result};

#[allow(unused)]
pub enum Command {
    Ping,
    Echo,
    Get,
    Set,
}

#[allow(unused)]
pub struct Payload {
    action: Command,
    payload: Option<Vec<String>>,
}

#[allow(unused)]

impl Command {
    pub fn parse(string_payload: String) -> Result<Payload> {
        let mut splitted_payload = string_payload
            .split("\r\n")
            .map(String::from)
            .collect::<Vec<String>>();

        let mut command: Option<Command> = None;

        while !splitted_payload.is_empty() {
            match splitted_payload.remove(0).as_str() {
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

impl Payload {
    pub fn build_response(&mut self) -> Result<String> {
        match self.action {
            Command::Ping => return Ok(String::from("+PONG\r\n")),
            Command::Echo => {
                let cloned_payload = self.payload.clone();
                let mut joined_resp = cloned_payload.unwrap().join("\r\n");
                joined_resp.push_str("\r\n");

                return Ok(joined_resp);
            }
            Command::Get => {
                // return Ok(String::from(""));
            }
            Command::Set => {
                // return Ok(String::from(""));
            }
        }

        bail!("Wrong command")
    }
}
