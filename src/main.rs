use syn::{ visit::Visit, File, ItemFn };
use std::fs;
use std::collections::HashMap;
use std::path::Path;
use walkdir::WalkDir;
use petgraph::graph::DiGraph;
use plotters::prelude::*;
use plotters::style::WHITE;

// Visitor to collect function names and relationships
struct FunctionVisitor {
    // TODO: use a Function structure instead of a string
    functions: Vec<String>,
    calls: HashMap<String, Vec<String>>,
}

impl FunctionVisitor {
    pub fn new() -> Self {
        Self {
            functions: Vec::new(),
            calls: HashMap::new()
        }
    }
}

impl<'ast> Visit<'ast> for FunctionVisitor {
    fn visit_item_fn(&mut self, node: &'ast ItemFn) {
        let fn_name = node.sig.ident.to_string();
        let mut called_functions = Vec::new();

        for stmt in &node.block.stmts {
            if let syn::Stmt::Expr(expr, _) = stmt {
                if let syn::Expr::Call(call_expr) = expr {
                    if let syn::Expr::Path(path) = *call_expr.func.clone() {
                        if let Some(segment) = path.path.segments.first() {
                            called_functions.push(segment.ident.to_string());
                        }
                    }
                }
            }
        }

        self.calls.insert(fn_name.clone(), called_functions);
        self.functions.push(fn_name);

        syn::visit::visit_item_fn(self, node);
    }
}

// Recursively parse all Rust files in the directory
fn parse_rust_code_recursive<P: AsRef<Path>>(dir: P) -> HashMap<String, Vec<String>> {
    let mut calls = HashMap::new();

    for entry in WalkDir::new(dir)
        .into_iter()
        .filter_map(Result::ok)
        .filter(
            |e|
                e
                    .path()
                    .extension()
                    .and_then(|ext| ext.to_str()) == Some("rs")
        ) {
        let code = fs::read_to_string(entry.path()).expect("Could not read the file");
        let syntax_tree: File = syn::parse_file(&code).expect("Failed to parse the file");

        let mut visitor = FunctionVisitor::new();
        visitor.visit_file(&syntax_tree);

        for (key, value) in visitor.calls {
            calls.entry(key).or_insert_with(Vec::new).extend(value);
        }
    }

    calls
}

// Build the call graph using petgraph
fn build_call_graph(function_calls: HashMap<String, Vec<String>>) -> DiGraph<String, ()> {
    let mut graph = DiGraph::new();
    let mut node_map = HashMap::new();

    for function in function_calls.keys() {
        let node = graph.add_node(function.clone());
        node_map.insert(function, node);
    }

    for (function, calls) in &function_calls {
        if let Some(&caller_node) = node_map.get(function) {
            for callee in calls {
                if let Some(&callee_node) = node_map.get(callee) {
                    graph.add_edge(caller_node, callee_node, ());
                }
            }
        }
    }

    graph
}

// Render the call graph to an SVG file using plotters
fn render_graph_to_svg(graph: &DiGraph<String, ()>, file_name: &str) {
    let root = SVGBackend::new(file_name, (800, 600)).into_drawing_area();
    root.fill(&WHITE).unwrap();

    let mut chart = ChartBuilder::on(&root)
        .caption("Call Graph", ("sans-serif", 30).into_font())
        .build_cartesian_2d(0..10, 0..10)
        .unwrap();

    for (index, node) in graph.node_indices().enumerate() {
        let label = graph[node].clone();
        chart
            .draw_series(
                PointSeries::of_element(
                    vec![(index as i32, 10 - (index as i32))],
                    5,
                    &BLACK,
                    &(|coord, size, style| {
                        EmptyElement::at(coord) +
                            Circle::new((0, 0), size, style.filled()) +
                            Text::new(label.clone(), (10, 0), ("sans-serif", 15))
                    })
                )
            )
            .unwrap();
    }

    root.present().unwrap();
}

fn main() {
    // Parse the Rust project in the current directory
    let function_calls = parse_rust_code_recursive(".");

    // Build the call graph
    let graph = build_call_graph(function_calls);

    // Render the graph as an SVG image
    render_graph_to_svg(&graph, "call_graph.svg");

    println!("Graph generated as 'call_graph.svg'.");
}
