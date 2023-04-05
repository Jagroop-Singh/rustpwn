use std::io::{self, prelude::*};
use clap::Parser;
mod util;

#[derive(Parser)]
struct Cli{
    // Process
    process: String
}

#[tokio::main]
async fn main() {
    util::commands::menu().await;
}

