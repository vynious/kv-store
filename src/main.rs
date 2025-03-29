use std::{sync::Arc, thread};
use kv_store::server::Server;


fn main() {
    let server_handler = thread::spawn(|| {
        let svr = Arc::new(Server::new(6));
        svr.run_server();
    });
    loop {
        
    }
}