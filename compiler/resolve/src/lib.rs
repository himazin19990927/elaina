use std::collections::HashMap;

use ast::*;
use hir::def_id::{DefId, DefIdGen};
use span::{
    span::Span,
    symbol::{Ident, Symbol},
};

pub fn resolve_items(items: &[Item]) -> HashMap<Span, DefId> {
    let mut resolver = ASTNameResolver::new();
    resolver.resolve_items_decl(items);
    resolver.resolve_items(items);

    resolver.finish()
}

pub struct ASTNameResolver {
    def_gen: DefIdGen,
    resolution: HashMap<Span, DefId>,
    scopes: Vec<HashMap<Symbol, DefId>>,
}

impl ASTNameResolver {
    pub fn new() -> ASTNameResolver {
        ASTNameResolver {
            def_gen: DefIdGen::new(),
            resolution: HashMap::new(),
            scopes: Vec::new(),
        }
    }

    pub fn finish(self) -> HashMap<Span, DefId> {
        self.resolution
    }

    pub fn new_decl(&mut self, name: Symbol, span: Span) {
        let id = self.def_gen.new_id();
        self.scopes.last_mut().unwrap().insert(name, id);
        self.resolution.insert(span, id);
    }

    pub fn new_use(&mut self, name: Symbol, span: Span) {
        let id = self.lookup(&name).expect("Undefined ident given.");
        self.resolution.insert(span, id);
    }

    fn lookup(&self, name: &Symbol) -> Option<DefId> {
        for scope in self.scopes.iter().rev() {
            if let Some(def) = scope.get(name) {
                return Some(*def);
            }
        }

        None
    }

    pub fn with_new_scope<F>(&mut self, f: F)
    where
        F: FnOnce(&mut ASTNameResolver),
    {
        self.scopes.push(HashMap::new());
        f(self);
        self.scopes.pop();
    }
}

impl ASTNameResolver {
    pub fn resolve_ident(&mut self, ident: &Ident) {
        self.new_use(ident.name, ident.span);
    }

    pub fn resolve_expr(&mut self, expr: &Expr) {
        match expr {
            Expr::Binary { lhs, rhs, .. } => {
                self.resolve_expr(lhs);
                self.resolve_expr(rhs);
            }
            Expr::Unary { expr, .. } => self.resolve_expr(expr),
            Expr::If {
                cond,
                then,
                else_opt,
            } => {
                self.resolve_expr(cond);
                self.resolve_block(then);

                if let Some(else_expr) = else_opt {
                    self.resolve_expr(else_expr);
                }
            }
            Expr::Loop { block } => self.resolve_block(block),
            Expr::Break { expr } | Expr::Continue { expr } => {
                if let Some(expr) = expr {
                    self.resolve_expr(expr)
                }
            }
            Expr::Block { block } => self.resolve_block(block),
            Expr::Assign { lhs, rhs } => {
                self.resolve_expr(rhs);
                self.resolve_expr(lhs);
            }
            Expr::Lit { .. } => {}
            Expr::Path(path) => self.resolve_ident(&path.ident),
        }
    }

    pub fn resolve_block(&mut self, block: &Block) {
        self.with_new_scope(|this| {
            for stmt in &block.stmts {
                this.resolve_stmt(stmt);
            }
        });
    }

    pub fn resolve_stmt(&mut self, stmt: &Stmt) {
        match stmt {
            Stmt::Local { ident, init, .. } => {
                self.resolve_expr(init);
                self.new_decl(ident.name, ident.span);
            }
            Stmt::Expr(expr) | Stmt::Semi(expr) | Stmt::Println(expr) => self.resolve_expr(expr),
        }
    }

    pub fn resolve_items_decl(&mut self, items: &[Item]) {
        self.with_new_scope(|this| {
            for item in items {
                let ident = &item.ident;
                this.new_decl(ident.name, ident.span);
            }
        })
    }

    pub fn resolve_items(&mut self, items: &[Item]) {
        self.with_new_scope(|this| {
            for item in items {
                match &item.kind {
                    ItemKind::Fn(fun) => this.resolve_item_fn(fun.as_ref()),
                }
            }
        })
    }

    pub fn resolve_item_fn(&mut self, fun: &Fn) {
        self.with_new_scope(|this| {
            for param in &fun.inputs {
                let ident = &param.ident;
                this.new_decl(ident.name, ident.span);
            }

            this.resolve_block(&fun.body);
        })
    }
}
