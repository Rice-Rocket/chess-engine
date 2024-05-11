use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_quote, punctuated::Punctuated, ItemFn};

pub fn expand(mut item: ItemFn) -> TokenStream {
    item.attrs.push(parse_quote!(#[must_use]));
    item.sig.ident = format_ident!("flip_{}", item.sig.ident);
    
    for stmt in item.block.stmts.iter_mut() {
        expand_stmt(stmt);
    }
    
    quote! {
        #item
    }
}


fn expand_stmt(stmt: &mut syn::Stmt) {
    match stmt {
        syn::Stmt::Local(local) => if let Some(init) = &mut local.init {
            expand_expr(init.expr.as_mut());
            if let Some(div) = &mut init.diverge {
                expand_expr(div.1.as_mut());
            }
        },
        syn::Stmt::Item(_item) => unreachable!(),
        syn::Stmt::Expr(expr, _tokens) => expand_expr(expr),
        syn::Stmt::Macro(_mac) => (),
    }
}


fn expand_expr(expr: &mut syn::Expr) {
    match expr {
        syn::Expr::Field(expr_field) => {
            expand_expr(expr_field.base.as_mut());
            match &expr_field.member {
                syn::Member::Named(name) => {
                    if name == "color" {
                        let span = name.span();
                        *expr = syn::Expr::MethodCall(syn::ExprMethodCall {
                            attrs: Vec::new(),
                            receiver: Box::new(expr.clone()),
                            dot_token: syn::Token![.](span),
                            method: format_ident!("flip"),
                            turbofish: None,
                            paren_token: syn::token::Paren(span),
                            args: Punctuated::new(),
                        })
                    }
                },
                syn::Member::Unnamed(_) => (),
            }
        },
        syn::Expr::MethodCall(expr_call) => {
            expand_expr(expr_call.receiver.as_mut());
            for arg in expr_call.args.iter_mut() {
                expand_expr(arg);
            }
        },
        _ => propagate_expansion(expr)
    }
}


fn propagate_expansion(expr: &mut syn::Expr) {
    match expr {
        syn::Expr::Array(expr_arr) => {
            for elem in expr_arr.elems.iter_mut() {
                expand_expr(elem);
            }
        },
        syn::Expr::Assign(expr_assign) => {
            expand_expr(expr_assign.left.as_mut());
            expand_expr(expr_assign.right.as_mut());
        },
        syn::Expr::Async(expr_async) => {
            for stmt in expr_async.block.stmts.iter_mut() {
                expand_stmt(stmt);
            }
        },
        syn::Expr::Await(expr_await) => {
            expand_expr(expr_await.base.as_mut());
        },
        syn::Expr::Binary(expr_binary) => {
            expand_expr(expr_binary.left.as_mut());
            expand_expr(expr_binary.right.as_mut());
        },
        syn::Expr::Block(expr_block) => {
            for stmt in expr_block.block.stmts.iter_mut() {
                expand_stmt(stmt);
            }
        },
        syn::Expr::Break(expr_break) => {
            if let Some(ex) = expr_break.expr.as_mut() {
                expand_expr(ex);
            }
        },
        syn::Expr::Call(expr_call) => {
            expand_expr(expr_call.func.as_mut());
            for arg in expr_call.args.iter_mut() {
                expand_expr(arg);
            }
        },
        syn::Expr::Cast(expr_cast) => {
            expand_expr(expr_cast.expr.as_mut());
        },
        syn::Expr::Closure(expr_closure) => {
            expand_expr(expr_closure.body.as_mut());
        },
        syn::Expr::Const(expr_const) => {
            for stmt in expr_const.block.stmts.iter_mut() {
                expand_stmt(stmt);
            }
        },
        syn::Expr::Continue(_) => (),
        syn::Expr::ForLoop(expr_for_loop) => {
            expand_expr(expr_for_loop.expr.as_mut());
            for stmt in expr_for_loop.body.stmts.iter_mut() {
                expand_stmt(stmt);
            }
        },
        syn::Expr::Group(expr_group) => {
            expand_expr(expr_group.expr.as_mut());
        },
        syn::Expr::If(expr_if) => {
            expand_expr(expr_if.cond.as_mut());
            for stmt in expr_if.then_branch.stmts.iter_mut() {
                expand_stmt(stmt);
            }
            if let Some(else_branch) = &mut expr_if.else_branch {
                expand_expr(else_branch.1.as_mut());
            }
        },
        syn::Expr::Index(expr_index) => {
            expand_expr(expr_index.expr.as_mut());
            expand_expr(expr_index.index.as_mut());
        },
        syn::Expr::Infer(_) => (),
        syn::Expr::Let(expr_let) => {
            expand_expr(expr_let.expr.as_mut());
        },
        syn::Expr::Lit(_) => (),
        syn::Expr::Loop(expr_loop) => {
            for stmt in expr_loop.body.stmts.iter_mut() {
                expand_stmt(stmt);
            }
        },
        syn::Expr::Macro(_) => (),
        syn::Expr::Match(expr_match) => {
            expand_expr(expr_match.expr.as_mut());
            for arm in expr_match.arms.iter_mut() {
                if let Some(guard) = &mut arm.guard {
                    expand_expr(guard.1.as_mut());
                }
                expand_expr(arm.body.as_mut());
            }
        },
        syn::Expr::Paren(expr_paren) => {
            expand_expr(expr_paren.expr.as_mut());
        },
        syn::Expr::Path(_) => (),
        syn::Expr::Range(expr_range) => {
            if let Some(start) = &mut expr_range.start {
                expand_expr(start);
            }
            if let Some(end) = &mut expr_range.end {
                expand_expr(end);
            }
        },
        syn::Expr::Reference(expr_ref) => {
            expand_expr(expr_ref.expr.as_mut());
        },
        syn::Expr::Repeat(expr_repeat) => {
            expand_expr(expr_repeat.expr.as_mut());
            expand_expr(expr_repeat.len.as_mut());
        },
        syn::Expr::Return(expr_ret) => {
            if let Some(ex) = &mut expr_ret.expr {
                expand_expr(ex);
            }
        },
        syn::Expr::Struct(expr_struct) => {
            for field in expr_struct.fields.iter_mut() {
                expand_expr(&mut field.expr);
            }
            if let Some(rest) = &mut expr_struct.rest {
                expand_expr(rest);
            }
        },
        syn::Expr::Try(expr_try) => {
            expand_expr(expr_try.expr.as_mut());
        },
        syn::Expr::TryBlock(expr_try_block) => {
            for stmt in expr_try_block.block.stmts.iter_mut() {
                expand_stmt(stmt);
            }
        },
        syn::Expr::Tuple(expr_tuple) => {
            for elem in expr_tuple.elems.iter_mut() {
                expand_expr(elem);
            }
        },
        syn::Expr::Unary(expr_unary) => {
            expand_expr(expr_unary.expr.as_mut());
        },
        syn::Expr::Unsafe(expr_unsafe) => {
            for stmt in expr_unsafe.block.stmts.iter_mut() {
                expand_stmt(stmt);
            }
        },
        syn::Expr::Verbatim(_) => (),
        syn::Expr::While(expr_while) => {
            expand_expr(expr_while.cond.as_mut());
            for stmt in expr_while.body.stmts.iter_mut() {
                expand_stmt(stmt);
            }
        },
        syn::Expr::Yield(expr_yield) => {
            if let Some(ex) = &mut expr_yield.expr {
                expand_expr(ex);
            }
        },
        _ => todo!(),
    }
}
