use clap::Args;

use std::path::PathBuf;

#[derive(Debug, Args)]
pub struct GraphCommand {
    /// Directory to audit
    pub directory: PathBuf,
    
    /// Path to the resulting output file (by default it will place in the path target/e2-graph) 
    #[arg(short)]
    pub output: Option<PathBuf>
}
