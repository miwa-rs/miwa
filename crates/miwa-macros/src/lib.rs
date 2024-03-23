mod config;
mod extension;
mod injectable;
mod interface;
mod utils;

use config::ExtensionConfig;
use darling::FromDeriveInput;
use darling::FromMeta;
use extension::Extension;
use injectable::Injectable;
use interface::Interface;
use syn::parse_macro_input;
use syn::DeriveInput;
use syn::ItemFn;
use syn::ItemTrait;

macro_rules! parse_nested_meta {
    ($ty:ty, $args:expr) => {{
        let meta = match darling::ast::NestedMeta::parse_meta_list(proc_macro2::TokenStream::from(
            $args,
        )) {
            Ok(v) => v,
            Err(e) => {
                return proc_macro::TokenStream::from(darling::Error::from(e).write_errors());
            }
        };

        match <$ty>::from_list(&meta) {
            Ok(object_args) => object_args,
            Err(err) => return proc_macro::TokenStream::from(err.write_errors()),
        }
    }};
}

#[proc_macro_attribute]
pub fn extension(
    args: proc_macro::TokenStream,
    original: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let extension = parse_nested_meta!(Extension, args);
    let item_fn: ItemFn = parse_macro_input!(original as ItemFn);
    extension::generate(&extension, &item_fn)
}

#[proc_macro_attribute]
pub fn interface(
    args: proc_macro::TokenStream,
    original: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let interface = parse_nested_meta!(Interface, args);
    let item_trait: ItemTrait = parse_macro_input!(original as ItemTrait);
    interface::generate(&interface, &item_trait)
}

#[proc_macro_derive(Injectable, attributes(service))]
pub fn injectable(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let object_args =
        match Injectable::from_derive_input(&syn::parse_macro_input!(input as DeriveInput)) {
            Ok(object_args) => object_args,
            Err(err) => return proc_macro::TokenStream::from(err.write_errors()),
        };
    match injectable::generate(&object_args) {
        Ok(ts) => ts.into(),
        Err(e) => e.to_compile_error().into(),
    }
}

#[proc_macro_derive(ExtensionConfig, attributes(config))]
pub fn config(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let object_args =
        match ExtensionConfig::from_derive_input(&syn::parse_macro_input!(input as DeriveInput)) {
            Ok(object_args) => object_args,
            Err(err) => return proc_macro::TokenStream::from(err.write_errors()),
        };
    match config::generate(&object_args) {
        Ok(ts) => ts.into(),
        Err(e) => e.to_compile_error().into(),
    }
}
