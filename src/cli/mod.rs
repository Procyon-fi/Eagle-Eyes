use clap::{Parser, Subcommand};
use graph::GraphCommand;

mod graph;

#[derive(Debug, Parser)]
#[command(name = "e2")]
#[command(about = "Rust-based tool for visualizing and auditing the call graph of a Rust-based smart contract project.", long_about = None)]
pub struct App {
    #[command(subcommand)]
    pub command: SubCommands,
}

#[derive(Debug, Subcommand)]
pub enum SubCommands {
    Graph(GraphCommand),
}
