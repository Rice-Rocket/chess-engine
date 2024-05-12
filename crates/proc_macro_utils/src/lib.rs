use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, ItemFn};

mod flip_eval;


#[proc_macro_attribute]
pub fn flipped_eval(_args: TokenStream, item: TokenStream) -> TokenStream {
    let mut item = parse_macro_input!(item as ItemFn);
    let gen = flip_eval::expand(item.clone());

    item.sig.ident = format_ident!("friendly_{}", item.sig.ident);

    quote! {
        #item

        #gen
    }.into()
}
