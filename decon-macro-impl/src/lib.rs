use proc_macro2::{self, Ident, Span, TokenStream};
use syn::{Expr, ExprPath, Stmt, __private::ToTokens, parse_quote, visit_mut::{self, VisitMut}};
use uuid::Uuid;

#[proc_macro_attribute]
pub fn reset(_args: proc_macro::TokenStream, input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    _reset(input.into()).into()
}

fn _reset(input: TokenStream) -> TokenStream {
    let mut func: syn::ItemFn = syn::parse2(input).unwrap();

    let mut stmts = core::mem::take(&mut func.block.stmts);
    let mut inner_most_body = &mut func.block.stmts;

    while let Some(CPSTransformState { i, lambda, symbol }) = cps(&mut stmts) {
        let rest = stmts.drain(i..).collect();
        *inner_most_body = stmts;

        let tail_call: Expr = parse_quote! {
            (#lambda)(Box::new(move |#symbol| {}))
        };

        inner_most_body.push(Stmt::Expr(tail_call));
        loop { // how to write this clusterfuck properly?
            if let Stmt::Expr(Expr::Call(expr_call)) = inner_most_body.last_mut().unwrap() {
                if let Expr::Call(expr_call) = &mut expr_call.args[0] {
                    if let Expr::Closure(closure) = &mut expr_call.args[0] {
                        if let Expr::Block(block) = &mut *closure.body {
                            inner_most_body = &mut block.block.stmts;
                            break
                        }
                    }
                }
            }
            unreachable!()
        };
        stmts = rest;
    }

    inner_most_body.extend(stmts.into_iter());
    func.into_token_stream()
}

struct CPSTransformState {
    i: usize, // index of the first stmt that contains `shift`. It is also already modifed.
    lambda: Expr,
    symbol: Ident,
}

// this function split the statements into two parts, by the first occurence of `shift`
fn cps(code: &mut [Stmt]) -> Option<CPSTransformState> {
    for i in 0..code.len() {
        if let Some((lambda, symbol)) = transform(&mut code[i]) {
            return Some(CPSTransformState { i, lambda, symbol })
        }
    }

    None
}

fn transform(stmt: &mut Stmt) -> Option<(Expr, Ident)> {
    let expr = match stmt {
        Stmt::Local(x) => &mut x.init.as_mut()?.1,
        Stmt::Item(_) => unimplemented!(),
        Stmt::Expr(e) => e,
        Stmt::Semi(e, _) => e,
    };

    #[derive(Default)]
    struct ShiftVisitor {
        result: Option<(Expr, Ident)>, // the handler (argument to shift) and the placeholder symbol
    }

    impl VisitMut for ShiftVisitor {
        fn visit_expr_mut(&mut self, node: &mut syn::Expr) {
            if let Expr::Call(expr_call) = node {
                if let Expr::Path(ExprPath { attrs: _, qself: None, path }) = &*expr_call.func {
                    if path.is_ident("shift") {
                        if expr_call.args.len() != 1 {
                            panic!("shift accepts only one argument")
                        }

                        let lambda = expr_call.args.pop().unwrap().into_value(); // destructive, but we are going to discard it
                        let name = format!("decon_{}", Uuid::new_v4().to_simple());
                        let symbol = Ident::new(&name, Span::call_site());
                        *node = Expr::Verbatim(symbol.to_token_stream());
                        self.result = Some((lambda, symbol));
                    }
                }
            }

            // call "super"
            visit_mut::visit_expr_mut(self, node)
        }
    }

    let mut visitor: ShiftVisitor = Default::default();
    visitor.visit_expr_mut(expr);
    visitor.result
}

#[cfg(test)]
mod test {
    use super::*;
    
    #[test]
    fn test() {
        let text = r#"
        fn raw(n: usize) -> usize {
            let a = shift(|cont| {
                cont(1) + cont(2)
            });
            println!("{}", a);
            let b = shift(|cont| {
                cont(3) + 4
            });
            a + b
        }
        "#;

        _reset(text.parse().unwrap());
    }
}

// TODO: give warning if there are more than one `shift` in a single Stmt, as our visiting order may not be the execution order
// TODO: figure out span and improve error reporting