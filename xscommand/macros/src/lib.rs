//! Procedural Macros for #[derive(XSCommand)] Implementation
//! 

extern crate proc_macro;
#[macro_use]
extern crate quote;
#[macro_use]
extern crate syn;
extern crate xscommand;
// extern crate proc_macro2;
// extern crate core;

use  proc_macro::TokenStream;
use quote::quote;
// use xscommand::{XSCommand, XSCommandErr};

#[proc_macro_derive(XSCommand)]
pub fn xscommand_derive(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();
    let name = ast.ident;
    let gen = quote! {
        impl<'a> XSCommand<'a, DefaultErr> for #name<'a> {
            fn new() -> Self {
                let exe  = Command::new(stringify!(#name).to_ascii_lowercase());
                let args = Vec::new();
                Self {
                    exe,
                    args,
                    work_dir: None,
                }
            }
            
        }
    };
}



