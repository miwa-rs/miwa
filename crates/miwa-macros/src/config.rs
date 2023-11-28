use darling::FromDeriveInput;
use quote::quote;
use syn::Ident;

use crate::utils::get_crate_name;

#[derive(FromDeriveInput, Debug)]
#[darling(attributes(config), forward_attrs(doc))]
pub struct ExtensionConfig {
    pub ident: Ident,
    #[darling(default)]
    pub internal: bool,
    pub prefix: String,
}

pub fn generate(input: &ExtensionConfig) -> syn::Result<proc_macro2::TokenStream> {
    let crate_name = get_crate_name(input.internal);
    let ident = &input.ident;
    let prefix = &input.prefix;
    Ok(quote! {
        impl #crate_name::Configurable for #ident {
            fn prefix() -> &'static str {
                #prefix
            }
        }
    })
}
