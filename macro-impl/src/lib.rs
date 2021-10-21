use syn::{Block, Expr, ExprPath, Ident, Stmt, __private::{Span, ToTokens}, parse::Parser, parse_quote, token::Brace, visit_mut::{self, VisitMut}};
use uuid::Uuid;

enum ContinuationCaptureOption { Box, Ref, Mut }

#[proc_macro_attribute]
pub fn reset_func(_args: proc_macro::TokenStream, input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let mut func: syn::ItemFn = syn::parse(input).unwrap();
    transform_block(&mut func.block);
    func.into_token_stream().into()
}

#[proc_macro]
pub fn reset(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let stmts: Vec<Stmt> = Block::parse_within.parse(input).unwrap();
    let mut block = Block { brace_token: Brace { span: Span::call_site() }, stmts };
    transform_block(&mut block);
    block.into_token_stream().into()
}

fn transform_block(block: &mut Block) {
    if let Some(CPSTransformState { i, opt, lambda, symbol }) = cps_first(&mut block.stmts[..]) {
        let mut rest = Block { brace_token: block.brace_token, stmts: block.stmts.drain(i..).collect() };
        
        transform_block(&mut rest);

        let tail_call: Expr = match opt {
            ContinuationCaptureOption::Box => parse_quote! {
                (#lambda)(Box::new(move |#symbol| {#rest}))
            },
            ContinuationCaptureOption::Ref => parse_quote! {
                (#lambda)(&|#symbol| {#rest})
            },
            ContinuationCaptureOption::Mut => parse_quote! {
                (#lambda)(&mut |#symbol| {#rest})
            }
        };

        block.stmts.push(Stmt::Expr(tail_call))
    }
}

struct CPSTransformState {
    i: usize, // index of the first stmt that contains `shift`. It is also already modifed.
    opt: ContinuationCaptureOption,
    lambda: Expr,
    symbol: Ident,
}

// this function split the statements into two parts, by the first occurence of `shift`
fn cps_first(code: &mut [Stmt]) -> Option<CPSTransformState> {
    for (i, stmt) in code.iter_mut().enumerate() {
        if let Some((opt, lambda, symbol)) = transform(stmt) {
            return Some(CPSTransformState { i, opt, lambda, symbol })
        }
    }

    None
}

fn transform(stmt: &mut Stmt) -> Option<(ContinuationCaptureOption, Expr, Ident)> {
    let expr = match stmt {
        Stmt::Local(x) => &mut x.init.as_mut()?.1,
        Stmt::Item(_) => unimplemented!(),
        Stmt::Expr(e) => e,
        Stmt::Semi(e, _) => e,
    };

    #[derive(Default)]
    struct ShiftVisitor {
        result: Option<(ContinuationCaptureOption, Expr, Ident)>, // the handler (argument to shift) and the placeholder symbol
    }

    impl VisitMut for ShiftVisitor {
        fn visit_expr_mut(&mut self, node: &mut syn::Expr) {
            if let Expr::Call(expr_call) = node {
                if let Expr::Path(ExprPath { attrs: _, qself: None, path }) = &*expr_call.func {
                    if path.is_ident("shift") {
                        let opt = match expr_call.args.len() {
                            1 => ContinuationCaptureOption::Box,
                            2 => loop {
                                if let Expr::Path(ExprPath { attrs: _, qself: None, path }) = expr_call.args.pop().unwrap().into_value() {
                                    if let Some(ident) = path.get_ident() {
                                        match &ident.to_string()[..] {
                                            "Cont" | "ContBox" | "ContBoxMut" | "ContBoxOnce" | "ContBoxClonable" | "ContBoxMutClonable" | "ContBoxOnceClonable" => break ContinuationCaptureOption::Box,
                                            "ContRef" | "ContRefClonable" => break ContinuationCaptureOption::Ref,
                                            "ContMut" | "ContMutClonable" => break ContinuationCaptureOption::Mut,
                                            _ => {}
                                        }
                                    }
                                }
                                panic!("the second argument to `shift` can only be one of the following:
                                    Cont, ContBox, ContRef, ContRefClonable, ContMut, ContMutClonable, ContBoxMut, ContBoxOnce, ContBoxClonable, ContBoxMutClonable, ContBoxOnceClonable")
                            }
                            _ => panic!("shift accepts only either one or two argument(s)")
                        };

                        let lambda = expr_call.args.pop().unwrap().into_value();
                        let name = format!("decon_{}", Uuid::new_v4().to_simple());
                        let symbol = Ident::new(&name, Span::call_site());
                        *node = Expr::Verbatim(symbol.to_token_stream());
                        self.result = Some((opt, lambda, symbol));
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
