use std::process::exit;

use clap::Parser;
use e2::cli::{App, SubCommands};
use syn::visit::Visit;
use walkdir::WalkDir;

fn main() {
    let app = App::parse();
    match app.command {
        SubCommands::Graph(graph) => {
            if !graph.directory.exists() {
                eprintln!(
                    "error: The directory '{}' does not exist, please give a correct directory.",
                    graph.directory.display()
                );
                exit(1)
            }

            for entry in WalkDir::new(graph.directory)
                .into_iter()
                .filter_map(Result::ok)
                .filter(|e| e.path().extension().and_then(|ext| ext.to_str()) == Some("rs"))
            {
                let code = std::fs::read_to_string(entry.path()).expect("Could not read the file");
                let syntax_tree: syn::File =
                    syn::parse_file(&code).expect("Failed to parse the file");

                let mut visitor = e2::visitors::RustVisitor::new();
                visitor.visit_file(&syntax_tree);
            }
        }
    }
}
