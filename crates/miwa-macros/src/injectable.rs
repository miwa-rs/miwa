use darling::FromDeriveInput;
use quote::quote;
use syn::Ident;

use crate::utils::get_crate_name;

#[derive(FromDeriveInput, Debug)]
#[darling(attributes(service), forward_attrs(doc))]
pub struct Injectable {
    pub ident: Ident,
    #[darling(default)]
    pub internal: bool,
}

pub fn generate(input: &Injectable) -> syn::Result<proc_macro2::TokenStream> {
    let crate_name = get_crate_name(input.internal);
    let ident = &input.ident;
    Ok(quote! {
        impl<'a> #crate_name::core::FromMiwaContext<'a> for #ident {
            fn from_context(context : &'a #crate_name::core::MiwaContext) -> #crate_name::core::MiwaResult<Self> {
                context.resolve::<#ident>().ok_or_else(|| #crate_name::core::MiwaError::component_missing(stringify!(#ident)))
            }
        }
    })
}
