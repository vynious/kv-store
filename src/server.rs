use std::{io::{Read, Write}, net::{TcpListener, TcpStream}, sync::{Arc, Mutex}, thread::{self, JoinHandle}, time::Duration};
use crate::{storage::{KvStore, SharedKvShare}, threadpool::{PoolCreationError, ThreadPool}};


pub struct Server {
    tcp_listener: TcpListener,
    store: SharedKvShare,
    pool: ThreadPool,
     
}

impl Server {
    pub fn new(thread_count: usize) -> Self {
        Server { 
            tcp_listener: TcpListener::bind("127.0.0.1:6378").expect("failed to open port"),
            store: Arc::new(Mutex::new(KvStore::new())), 
            pool: ThreadPool::new(thread_count).unwrap() 
        }
    }

    pub fn run_server(self: Arc<Server>) {
        for stream in self.tcp_listener.incoming() {
            match stream {
                Ok(stream) => {
                    let server = Arc::clone(&self);
                    self.pool.execute(move || { 
                        server.handle_client(stream);
                    });
                },
                Err(e) => {
                    eprintln!("error: {}", e)
                }
            }
        }
    }

    fn handle_client(&self, mut stream: TcpStream) {
        let mut buffer = [0;128];
        match stream.read(&mut buffer) {
            Ok(n) => {

                let response = String::from_utf8_lossy(&buffer[..n]);
                println!("received: {}", response);
                if let Err(e) = stream.write_all(b"+PONG\r\n") {
                        eprintln!("failed to write to stream: {}" , e)
                    }
            }
            Err(e) => {
                eprint!("error reading from stream into buffer: {}", e)
            }
        }
    }

}





fn mock_client() {
    let mut stream = TcpStream::connect("127.0.0.1:6378").expect("failed to connect to server");
    stream
        .set_read_timeout(Some(Duration::from_secs(1)))
        .expect("failed to set read timeout");
    stream.write_all(b"+PING\r\n")
        .expect("failed to send ping");
    let mut buffer = [0;128]; 
    let n = stream.read(&mut buffer).expect("failed to read into buffer");
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