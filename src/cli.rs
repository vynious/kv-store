use clap::{Arg, Parser, Subcommand};
use serde::{Deserialize, Serialize};

#[derive(Parser, Debug)]
pub struct Args {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug, Serialize, Deserialize)]
pub enum Commands {
    Get { key: String },
    Set { key: String, value: String },
    Delete { key: String },
}
