use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, ItemFn, LitStr};

mod evaluation_fn;
mod evaluation_test;


#[proc_macro_attribute]
pub fn evaluation_fn(_args: TokenStream, item: TokenStream) -> TokenStream {
    let mut item = parse_macro_input!(item as ItemFn);
    let gen = evaluation_fn::expand(item.clone());

    item.sig.ident = format_ident!("friendly_{}", item.sig.ident);

    quote! {
        #item

        #gen
    }.into()
}

#[proc_macro_attribute]
pub fn evaluation_test(args: TokenStream, item: TokenStream) -> TokenStream {
    let args = parse_macro_input!(args as LitStr);
    let mut item = parse_macro_input!(item as ItemFn);
    evaluation_test::expand(&mut item, args);

    quote! {
        #item
    }.into()
}
