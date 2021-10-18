use proc_macro::*;
use syn::{Block, DeriveInput, Stmt, parse_macro_input};

#[proc_macro_attribute]
pub fn reset(_args: TokenStream, input: TokenStream) -> TokenStream {
    let func: syn::ItemFn = syn::parse(input).unwrap();



    todo!()
}

fn cps(code: &[Stmt]) -> Block {
    todo!()
}


