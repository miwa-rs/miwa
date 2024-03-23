use darling::FromMeta;
use quote::{format_ident, quote};
use syn::ItemTrait;

use crate::injectable::{self, Injectable};

#[derive(FromMeta, Default)]
pub struct Interface {
    name: Option<String>,
}

pub fn generate(extension: &Interface, item_trait: &ItemTrait) -> proc_macro::TokenStream {
    let vis = &item_trait.vis;
    let original_ident = &item_trait.ident;

    let ident = if let Some(name) = &extension.name {
        format_ident!("{}", name)
    } else {
        format_ident!("{}Ref", item_trait.ident)
    };
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

        #[derive(Clone)]
        #def_struct

        #injectable

    }
    .into()
}
