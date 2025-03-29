
# Rust Key-Value Store (KV Store)

A simple, in-memory key-value store in Rust:

- **Server** listens on `127.0.0.1:6378`
- **Client** sends serialized commands (`GET`, `SET`, `DELETE`) via `bincode`
- **Thread Pool** handles concurrency
- Basic **HashMap**-based storage


## Quick Start


**2. Run Server:**
```bash
cargo run --bin server
```

**3. Run Client:**
```bash
# Set
cargo run --bin client -- set mykey myvalue
# Get
cargo run --bin client -- get mykey
# Delete
cargo run --bin client -- delete mykey
```


## Overview

- **Commands**  
  ```rust
  #[derive(Subcommand, Serialize, Deserialize)]
  pub enum Commands {
      Get { key: String },
      Set { key: String, value: String },
      Delete { key: String },
  }
  ```
  The client serializes these commands and sends them to the server. The server deserializes and executes them on the `KvStore`.

- **KvStore**  
  A `HashMap<String, String>` with `get`, `set`, and `remove` operations. Access is synchronized via `Arc<Mutex<KvStore>>`.

- **ThreadPool**  
  A custom thread pool that uses a worker model to handle incoming client connections in parallel.
