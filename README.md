# Embedded KV Store in Rust

A lightweight key-value (KV) store using TCP networking, thread pools, and Rust's concurrency primitives.


## Features

- **Client-server architecture** over TCP.
- **Thread pool** for efficient concurrent request handling.
- **Binary serialization** (`bincode`) for fast command transmission.
- **Clap-based CLI** for user-friendly interaction.

## Commands

- `SET key value` — Sets a value for a key.
- `GET key` — Retrieves a value by key.
- `DELETE key` — Deletes a key-value pair.

## Architecture

### Client-Server Interaction

```
Client                   Server
   | Connect TCP (6378)    |
   |---------------------->| 
   | Serialized Command    |
   |---------------------->| Deserialize
   |                       | Process command
   |        Response       |
   |<----------------------|
```

### Thread Pool Mechanism

```
TCP Listener
      |
      v
  ThreadPool
      |
      +-------------------+---------------+
      |                   |               |
 Worker1              Worker2          WorkerN
      |                   |               |
      +-------------------+---------------+
                  |
                  v
              KV Store
```
## Usage

### Starting the Server

```shell
cargo run --bin server
```

### Running Client Commands

```shell
cargo run --bin client -- set key value
cargo run --bin client -- get key
cargo run --bin client -- delete key
```
