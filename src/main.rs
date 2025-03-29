use std::thread;
use kv_store::server::{run_server};

fn main() {
    let server = thread::spawn(|| {
        run_server();
    });
    loop {
        
    }
}