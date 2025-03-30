use bincode;
use clap::Parser;
use kv_store::cli::{Args, Commands};
use std::{
    io::{Error, Read, Write}, net::TcpStream, time::Duration
};

fn main() {
    let mut client = Client::new();
    let args = Args::parse();
    let cmd: Commands = match args.command {
        Commands::Set { key, value } => {
            println!(
                "sending SET command: {} = {}",
                key.to_string(),
                value.to_string()
            );
            Commands::Set {
                key: key.to_string(),
                value: value.to_string(),
            }
        }
        Commands::Delete { key } => {
            println!("sending DELETE command: {}", key.to_string());
            Commands::Delete {
                key: key.to_string(),
            }
        }
        Commands::Get { key } => {
            println!("sending GET command: {}", key.to_string());
            Commands::Get {
                key: key.to_string(),
            }
        }
    };
    let bytes = bincode::serialize(&cmd).expect("failed to serialize command");
    match client.send_message(&bytes) {
        Ok(resp) => println!("response: {}", resp),
        Err(e) => eprintln!("error: {}", e),
    }
}


pub struct Client {
    tcp_stream: TcpStream,
}

impl Client {
    pub fn new() -> Self {
        let stream = TcpStream::connect("127.0.0.1:6378").unwrap();
        stream
            .set_read_timeout(Some(Duration::from_secs(2)))
            .unwrap();
        Client { tcp_stream: stream }
    }

    pub fn send_message(&mut self, msg: &[u8]) -> Result<String, Error>{
        self.tcp_stream
            .write_all(msg)?;
        self.tcp_stream.flush()?;

        let mut buffer = [0; 1024];
        let mut response = String::new();
        
        loop {
            match self.tcp_stream.read(&mut buffer) {
                Ok(0) => {
                    if response.is_empty() {
                        return Err(Error::new(std::io::ErrorKind::ConnectionAborted,"server closed connection without sending data"));
                    }
                },
                Ok(n) => {
                    response.push_str(&String::from_utf8_lossy(&buffer[..n]));
                    if response.ends_with("\r\n") {
                        break
                    }
                },
                Err(e) => {
                    if e.kind() == std::io::ErrorKind::WouldBlock || e.kind() == std::io::ErrorKind::TimedOut {
                        if !response.is_empty() {
                            break;
                        }
                        return Err(Error::new(std::io::ErrorKind::TimedOut,"timeout waiting for server response"));
                    }
                    return Err(e)
                }
            }
        }
        Ok(response)
    }
}
