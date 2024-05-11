use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

mod flip_eval;


#[proc_macro_attribute]
pub fn flipped_eval(_args: TokenStream, item: TokenStream) -> TokenStream {
    let item = parse_macro_input!(item as ItemFn);
    let gen = flip_eval::expand(item.clone());

    quote! {
        #item

        #gen
    }.into()
}
