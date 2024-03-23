use darling::{ast::NestedMeta, FromMeta};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{FnArg, ItemFn, ItemTrait, Meta, Path};

use crate::{
    injectable::{self, Injectable},
    utils::get_crate_name,
};

#[derive(FromMeta, Default)]
pub struct Interface {
    #[darling(default)]
    internal: bool,
    name: Option<String>,
    #[darling(default)]
    provides: PathList,
}

#[derive(Default, Debug)]
pub struct PathList(pub Vec<Path>);

impl FromMeta for PathList {
    fn from_list(items: &[NestedMeta]) -> darling::Result<Self> {
        let mut res = Vec::new();
        for item in items {
            if let NestedMeta::Meta(Meta::Path(p)) = item {
                res.push(p.clone());
            } else {
                return Err(darling::Error::custom("Invalid path list"));
            }
        }
        Ok(PathList(res))
    }
}

pub fn generate(extension: &Interface, item_trait: &ItemTrait) -> proc_macro::TokenStream {
    let crate_name = get_crate_name(extension.internal);
    let vis = &item_trait.vis;
    let original_ident = &item_trait.ident;
    let ident = format_ident!("{}Ref", item_trait.ident);
    let def_struct = quote! {
        #vis struct #ident(std::sync::Arc<dyn #original_ident + std::marker::Send + std::marker::Sync + 'static>);

        impl #ident {
           pub fn wrap(s : impl #original_ident + std::marker::Send + std::marker::Sync + 'static) -> #ident {
               #ident(std::sync::Arc::new(s))
           }
        }

        impl std::ops::Deref for #ident {
                type Target = dyn #original_ident + Send + Sync + 'static;

                fn deref(&self) -> &Self::Target {
                        self.0.as_ref()
                }
        }

    };

    let injectable = injectable::generate(&Injectable {
        ident: ident.clone(),
        internal: false,
    })
    .unwrap();

    quote! {
        #item_trait

        #[allow(non_camel_case_types)]
        #[derive(Clone)]
        #def_struct

        #injectable

    }
    .into()
}
