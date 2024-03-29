use ast::op::{BinOp, UnOp};
use hir;
use span::*;
use thir::*;
use ty::{res::*, *};

use std::collections::HashMap;

pub struct HIRLoweringCtx<'ast, 'tcx> {
    tcx: TyCtx<'ast, 'tcx>,
    fn_headers: HashMap<DefId, FnHeader<'tcx>>,
}

impl<'ast, 'tcx> HIRLoweringCtx<'ast, 'tcx> {
    pub fn new(tcx: TyCtx<'ast, 'tcx>) -> HIRLoweringCtx<'ast, 'tcx> {
        HIRLoweringCtx {
            tcx,
            fn_headers: HashMap::new(),
        }
    }

    pub fn finish(self) -> TyCtx<'ast, 'tcx> {
        self.tcx
    }

    fn insert_ty(&mut self, def: DefId, ty: Ty<'tcx>) {
        self.tcx.def_map.insert(def, ty);
    }

    fn get_ty(&self, def: DefId) -> Ty<'tcx> {
        self.tcx
            .def_map
            .get(&def)
            .unwrap_or_else(|| {
                panic!(
                    "cannot found type of {:?}.\n defined types are:\n{:?}",
                    def, self.tcx.def_map
                )
            })
            .clone()
    }
}

impl<'ast, 'tcx> HIRLoweringCtx<'ast, 'tcx> {
    pub fn lower_ty(&self, ty: &ast::Ty) -> Ty<'tcx> {
        match &ty.kind {
            ast::TyKind::Path(path) => self.tcx.common_type_from_name(path.ident.name),
        }
    }

    pub fn lower_fun_header(
        &mut self,
        fn_def: DefId,
        fn_name: Symbol,
        hir_inputs: &Vec<hir::Param>,
        hir_output: &Option<ast::ty::Ty>,
    ) {
        // insert function header.
        let mut inputs = Vec::new();
        for input in hir_inputs {
            let ty = self.lower_ty(&input.ty);

            let param = Param {
                res: input.res,
                name: input.name,
                ty,
            };

            inputs.push(param);
        }

        let output = match hir_output {
            Some(ty) => self.lower_ty(ty),
            None => self.tcx.common_types.unit,
        };

        let header = FnHeader {
            def: fn_def,
            name: fn_name,
            inputs,
            output,
        };

        self.fn_headers.insert(fn_def, header);

        // insert function definition type.
        let fn_def_ty = self.tcx.intern(TyKind::FnDef(fn_def));
        self.tcx.def_map.insert(fn_def, fn_def_ty);

        // insert function parameter type.
        for param in hir_inputs {
            let ty = self.lower_ty(&param.ty);
            self.tcx.def_map.insert(param.res.def, ty);
        }
    }
}

impl<'ast, 'tcx> HIRLoweringCtx<'ast, 'tcx> {
    pub fn lower_lit(&self, lit: &hir::Lit) -> Expr<'tcx> {
        let ty = match &lit {
            hir::Lit::Bool { .. } => self.tcx.common_types.bool,
            hir::Lit::Int(_) => self.tcx.common_types.i32,
        };

        let lit = match lit {
            hir::Lit::Bool { value } => Lit::Bool { value: *value },
            hir::Lit::Int(lit) => Lit::Int(LitInt { value: lit.value }),
        };

        Expr::Lit { lit, ty }
    }

    pub fn lower_pat(&self, pat: &hir::Pat, ty: ty::Ty<'tcx>) -> Pat<'tcx> {
        let kind = match &pat.kind {
            hir::PatKind::Binding { res, name } => PatKind::Binding {
                res: *res,
                name: *name,
                ty,
            },
        };
        Pat {
            ty,
            kind: Box::new(kind),
        }
    }

    pub fn lower_expr(&mut self, expr: &hir::Expr) -> Expr<'tcx> {
        match expr {
            hir::Expr::Call { fun, args } => {
                let fun = self.lower_expr(fun);
                let args: Vec<Expr> = args.iter().map(|arg| self.lower_expr(arg)).collect();

                match *fun.ty() {
                    TyKind::FnDef(def) => {
                        let header = self.fn_headers.get(&def).unwrap();
                        if args.len() != header.inputs.len() {
                            panic!(
                                "this function takes {} arguments but {} argument was supplied.",
                                header.inputs.len(),
                                args.len()
                            );
                        }

                        for (supplied, taked) in args
                            .iter()
                            .map(|arg| arg.ty())
                            .zip(header.inputs.iter().map(|param| &param.ty))
                        {
                            if supplied != *taked {
                                panic!(
                                    "mismatched types. expected {:?} found {:?}",
                                    taked.kind(),
                                    supplied.kind()
                                );
                            }
                        }
                    }
                    _ => panic!("Type of expression tried to call is not a function type."),
                }

                let ty = match *fun.ty() {
                    TyKind::FnDef(def) => self.fn_headers.get(&def).unwrap().output.clone(),
                    _ => unreachable!(),
                };

                Expr::Call {
                    fun: Box::new(fun),
                    args,
                    ty,
                }
            }
            hir::Expr::Binary { op, lhs, rhs } => {
                let lhs = Box::new(self.lower_expr(lhs));
                let rhs = Box::new(self.lower_expr(rhs));

                let ty = match op {
                    BinOp::Mul | BinOp::Div | BinOp::Add | BinOp::Sub => self.tcx.common_types.i32,
                    BinOp::Eq | BinOp::Lt | BinOp::Le | BinOp::Ne | BinOp::Ge | BinOp::Gt => {
                        self.tcx.common_types.bool
                    }
                };

                Expr::Binary {
                    op: *op,
                    lhs,
                    rhs,
                    ty,
                }
            }
            hir::Expr::Unary { op, expr } => {
                let expr = Box::new(self.lower_expr(expr));
                let ty = match op {
                    UnOp::Neg => self.tcx.common_types.i32,
                };
                Expr::Unary { op: *op, expr, ty }
            }
            hir::Expr::If {
                cond,
                then,
                else_opt,
            } => {
                let cond = Box::new(self.lower_expr(cond));
                let then = Box::new(self.lower_block(then));
                let else_opt = else_opt.as_ref().map(|e| Box::new(self.lower_expr(e)));
                let ty = then.ty.clone();

                Expr::If {
                    cond,
                    then,
                    else_opt,
                    ty,
                }
            }
            hir::Expr::Loop { block } => {
                let block = Box::new(self.lower_block(block));

                Expr::Loop { block }
            }
            hir::Expr::Break { expr } => {
                let expr = expr.as_ref().map(|e| Box::new(self.lower_expr(e)));
                let ty = self.tcx.common_types.never;

                Expr::Break { expr, ty }
            }
            hir::Expr::Continue { expr } => {
                let expr = expr.as_ref().map(|e| Box::new(self.lower_expr(e)));
                let ty = self.tcx.common_types.never;

                Expr::Continue { expr, ty }
            }
            hir::Expr::Return { expr } => {
                let expr = expr.as_ref().map(|e| Box::new(self.lower_expr(e)));
                let ty = self.tcx.common_types.never;

                Expr::Return { expr, ty }
            }
            hir::Expr::Block { block } => {
                let block = Box::new(self.lower_block(block));

                Expr::Block { block }
            }
            hir::Expr::Assign { lhs, rhs } => {
                let rhs = Box::new(self.lower_expr(rhs));
                let lhs = Box::new(self.lower_expr(lhs));
                let ty = self.tcx.common_types.unit;

                Expr::Assign { lhs, rhs, ty }
            }
            hir::Expr::Lit { lit } => self.lower_lit(lit),
            hir::Expr::Path { path } => {
                let def = path.res.def;
                let ty = self.get_ty(def);
                Expr::VarRef { res: path.res, ty }
            }
        }
    }

    pub fn lower_stmt(&mut self, stmt: &hir::Stmt) -> Stmt<'tcx> {
        match stmt {
            hir::Stmt::Local { pat, ty, init } => {
                let init = self.lower_expr(init);

                let res = match pat.kind {
                    hir::PatKind::Binding { res, .. } => res,
                };
                let ty = ty
                    .as_ref()
                    .map(|ty| self.lower_ty(ty))
                    .expect("Type annotation is requred.");
                self.insert_ty(res.def, ty.clone());

                let pat = self.lower_pat(pat, ty.clone());

                Stmt::Local { pat, init }
            }
            hir::Stmt::Expr(expr) | hir::Stmt::Semi(expr) => Stmt::Expr(self.lower_expr(expr)),
            hir::Stmt::Println(expr) => Stmt::Println(self.lower_expr(expr)),
        }
    }

    pub fn lower_block(&mut self, block: &hir::Block) -> Block<'tcx> {
        let stmts = block.stmts.iter().map(|s| self.lower_stmt(s)).collect();
        let expr = block.expr.as_ref().map(|e| self.lower_expr(e));
        let ty = expr
            .as_ref()
            .map_or_else(|| self.tcx.common_types.unit, |e| e.ty());

        Block { stmts, expr, ty }
    }

    pub fn lower_items(&mut self, items: &[hir::Item]) -> Vec<Item<'tcx>> {
        // lower item decl
        for item in items {
            self.lower_item_header(item);
        }

        items.iter().map(|item| self.lower_item(item)).collect()
    }

    pub fn lower_item_header(&mut self, item: &hir::Item) {
        match &item.kind {
            hir::ItemKind::Fn(fun) => {
                self.lower_fun_header(item.res.def, item.name, &fun.inputs, &fun.output);
            }
        }
    }

    pub fn lower_item(&mut self, item: &hir::Item) -> Item<'tcx> {
        let kind = match &item.kind {
            hir::ItemKind::Fn(fun) => self.lower_fun(item.res.def, &fun.body),
        };

        Item {
            res: item.res,
            name: item.name,
            kind,
        }
    }

    fn lower_fun(&mut self, def: DefId, body: &hir::Block) -> ItemKind<'tcx> {
        let header = self
            .fn_headers
            .get(&def)
            .unwrap_or_else(|| panic!("not found function header(defId:{}).", def))
            .clone();

        let body = self.lower_block(body);

        ItemKind::Fn(Box::new(Fn { header, body }))
    }
}
