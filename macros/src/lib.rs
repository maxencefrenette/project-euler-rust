#![recursion_limit = "256"]

extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::parse::{Parse, ParseStream, Result};
use syn::punctuated::Punctuated;
use syn::{parse_macro_input, Ident, LitInt, LitStr, Token};

struct Register {
    ids: Vec<LitInt>,
}

impl Parse for Register {
    fn parse(input: ParseStream) -> Result<Self> {
        let ids_expr = Punctuated::<LitInt, Token![,]>::parse_terminated(input)?;
        let ids = ids_expr.into_iter().collect();

        Ok(Register { ids })
    }
}

#[proc_macro]
pub fn register(input: TokenStream) -> TokenStream {
    let Register { ids } = parse_macro_input!(input as Register);

    let identifiers = ids
        .iter()
        .map(|id| Ident::new(format!("sol_{}", id.value()).as_str(), id.span()));
    let identifiers2 = identifiers.clone();

    let string_lits = ids
        .iter()
        .map(|id| LitStr::new(format!("{}", id.value()).as_str(), id.span()));

    let expanded = quote! {
        use lazy_static::lazy_static;
        use std::collections::HashMap;

        #(mod #identifiers;)*

        lazy_static! {
            pub static ref SOLUTIONS: HashMap<&'static str, fn() -> ()> = {
                let mut m = HashMap::new();

                #(
                    m.insert(#string_lits, (|| println!("{}", #identifiers2::solve())) as fn() -> ());
                )*

                m
            };
        }
    };

    TokenStream::from(expanded)
}
