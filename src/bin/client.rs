use bincode;
use clap::Parser;
use kv_store::cli::{Args, Commands};
use std::{
    io::{Read, Write},
    net::TcpStream,
    time::Duration,
};

fn main() {
    let args = Args::parse();
    match args.command {
        Commands::Set { key, value } => {
            let mut client = Client::new();
            println!(
                "sending SET command: {} = {}",
                key.to_string(),
                value.to_string()
            );
            let cmd = Commands::Set {
                key: key.to_string(),
                value: value.to_string(),
            };
            let bytes = bincode::serialize(&cmd).expect("failed to serialize command");
            client.send_message(&bytes)
        }
        Commands::Delete { key } => {
            let mut client: Client = Client::new();
            println!("sending DELETE command: {}", key.to_string());
            let cmd = Commands::Delete {
                key: key.to_string(),
            };
            let bytes = bincode::serialize(&cmd).expect("failed to serialize command");
            client.send_message(&bytes)
        }
        Commands::Get { key } => {
            let mut client = Client::new();
            println!("sending GET command: {}", key.to_string());
            let cmd = Commands::Get {
                key: key.to_string(),
            };
            let bytes = bincode::serialize(&cmd).expect("failed to serialize command");
            client.send_message(&bytes)
        }
    };
}

pub struct Client {
    tcp_stream: TcpStream,
}

impl Client {
    pub fn new() -> Self {
        let stream = TcpStream::connect("127.0.0.1:6378").unwrap();
        stream
            .set_read_timeout(Some(Duration::from_secs(1)))
            .unwrap();
        Client { tcp_stream: stream }
    }

    pub fn send_message(&mut self, msg: &Vec<u8>) {
        self.tcp_stream
            .write_all(msg)
            .expect("failed to send message");
        let mut buffer = Vec::new();
        let n = self
            .tcp_stream
            .read_to_end(&mut buffer)
            .expect("failed to read into buffer");
        let response = String::from_utf8_lossy(&buffer[..n]);
        println!("response: {}", response);
    }
}
