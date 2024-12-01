use syn::{visit::Visit, ItemFn};

use crate::ast::{Function, Ident};

pub struct RustVisitor {
    functions: Vec<Function>,
}

impl RustVisitor {
    pub fn new() -> Self {
        Self {
            functions: Vec::new(),
        }
    }
}

impl<'ast> Visit<'ast> for RustVisitor {
    fn visit_item_fn(&mut self, node: &'ast ItemFn) {
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

        self.functions.push(Function {
            name: Ident::from(&node.sig.ident)
        });

        syn::visit::visit_item_fn(self, node);
    }
}
