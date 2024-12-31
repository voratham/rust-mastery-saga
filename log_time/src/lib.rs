use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

#[proc_macro_attribute]
pub fn log_time(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);
    let fn_name = &input.sig.ident;
    let fn_body = &input.block;
    let fn_visibility = &input.vis;
    let fn_signature = &input.sig;

    let expanded = quote! {
        #fn_visibility #fn_signature {
                let start = std::time::Instant::now();

                #fn_body

                let elapsed = start.elapsed();
                println!("{} took: {:?}", stringify!(#fn_name), elapsed);
        }
    };

    expanded.into()
}
