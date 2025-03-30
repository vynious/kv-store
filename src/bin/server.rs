use clap::Command;
use kv_store::threadpool::ThreadPool;
use kv_store::{
    cli::Commands,
    storage::{self, KvStore, SharedKvStore},
};
use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    sync::{Arc, Mutex},
    thread::{self, JoinHandle},
    time::Duration,
};

fn main() {
    let server_handler = thread::spawn(|| {
        let svr = Arc::new(Server::new(6));
        svr.run_server();
    });
    loop {}
}

pub struct Server {
    tcp_listener: TcpListener,
    store: SharedKvStore,
    pool: ThreadPool,
}

impl Server {
    pub fn new(thread_count: usize) -> Self {
        Server {
            tcp_listener: TcpListener::bind("127.0.0.1:6378").expect("failed to open port"),
            store: Arc::new(Mutex::new(KvStore::new())),
            pool: ThreadPool::new(thread_count).unwrap(),
        }
    }

    pub fn run_server(self: Arc<Server>) {
        for stream in self.tcp_listener.incoming() {
            match stream {
                Ok(stream) => {
                    let server = Arc::clone(&self);
                    self.pool.execute(move || {
                        server.handle_incoming_cmd(stream);
                    });
                }
                Err(e) => {
                    eprintln!("error: {}", e)
                }
            }
        }
    }

    // todo: got issues
    fn handle_incoming_cmd(&self, mut stream: TcpStream) {
        if let Err(e) = stream.set_read_timeout(Some(Duration::from_secs(5))) {
                eprintln!("failed to set timeout: {}", e);
                return;
        }
        let mut buffer = [0; 1024]; 
        if let Err(e) = stream.read(&mut buffer) { 
            eprintln!("failed to read into buffer: {}",e);
            return;
        }
        let cmd = match bincode::deserialize(&buffer) {
            Ok(cmd) => {
                cmd
            },
            Err(e) => {
                eprintln!("failed to deserialize command: {}", e);
                return;
            }
        };

        let mut response = String::new();
    
        match cmd {
            Commands::Set { key, value } => {
                println!("setting key={}, value={}", key, value);
                self.store.lock().unwrap().set(key, value);
                response.push_str("OK");
            }
            Commands::Delete { key } => {
                println!("deleting key={}", key);
                match self.store.lock().unwrap().remove(&key) {
                    Some(resp) => {
                        response.push_str(&resp);
                    }
                    None => {
                        response.push_str("key not found");
                    }
                }
            }
            Commands::Get { key } => {
                println!("getting key={}", key);
                match self.store.lock().unwrap().get(&key) {
                    Some(resp) => {
                        response.push_str(&resp);
                    }
                    None => {
                        response.push_str("key not found");
                    }
                }
            }
        }
        response.push_str("\r\n");
        println!("response: {}", response);
        if let Err(e) = stream.write_all(response.as_bytes()) {
            eprintln!("failed to write to stream: {}", e);
            return;
        }
        
        if let Err(e) = stream.flush() {
            eprintln!("failed to flush stream: {}", e);
            return;
        }
    }
}

fn mock_client() {
    let mut stream = TcpStream::connect("127.0.0.1:6378").expect("failed to connect to server");
    stream
        .set_read_timeout(Some(Duration::from_secs(1)))
        .expect("failed to set read timeout");
    stream.write_all(b"+PING\r\n").expect("failed to send ping");
    let mut buffer = [0; 128];
    let n = stream
        .read(&mut buffer)
        .expect("failed to read into buffer");
    let response = String::from_utf8_lossy(&buffer[..n]);
    assert_eq!(response, "+PONG\r\n");
}

#[test]
fn test_spam_pings() {
    let server = thread::spawn(|| {
        let server = Arc::new(Server::new(5));
        server.run_server();
    });

    thread::sleep(Duration::from_millis(200));

    let num_of_pings = 10;
    let mut handles: Vec<JoinHandle<_>> = Vec::with_capacity(num_of_pings);
    for _ in 0..num_of_pings {
        handles.push(thread::spawn(|| {
            mock_client();
        }));
    }
    for handle in handles {
        handle.join().expect("client thread panicked")
    }

    server.thread().unpark();
}
