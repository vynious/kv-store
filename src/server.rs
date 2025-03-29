use std::{io::{Read, Write}, net::{TcpListener, TcpStream}, thread::{self, JoinHandle}, time::Duration};
use crate::threadpool::{ThreadPool, PoolCreationError};

// reads into buffer
fn handle_client(mut stream: TcpStream) {
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

pub fn run_server() {
    let pool = match ThreadPool::new(5) {
        Ok(pool) => pool,
        Err(e) => {
            eprintln!("failed to create thread pool: {}", e);
            return
        }
    };
    let listener = TcpListener::bind("127.0.0.1:6378").expect("failed to open port");
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                pool.execute(|| {
                    handle_client(stream);
                });
            },
            Err(e) => {
                println!("error: {}", e)
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
        run_server();
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