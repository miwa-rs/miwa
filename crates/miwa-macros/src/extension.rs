use darling::{ast::NestedMeta, FromMeta};
use proc_macro2::TokenStream;
use quote::quote;
use syn::{FnArg, ItemFn, Meta, Path};

use crate::utils::get_crate_name;

#[derive(FromMeta, Default)]
pub struct Extension {
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

pub fn generate(extension: &Extension, item_fn: &ItemFn) -> proc_macro::TokenStream {
    let crate_name = get_crate_name(extension.internal);
    let vis = &item_fn.vis;
    let ident = &item_fn.sig.ident;
    let def_struct = quote! {
        #vis struct #ident;
    };

    let extension_name = match extension.name {
        Some(ref n) => n.clone(),
        None => ident.clone().to_string(),
    };
    let mut extractors = Vec::new();
    let mut args = Vec::new();
    let mut requires = Vec::new();

    for (idx, input) in item_fn.sig.inputs.clone().into_iter().enumerate() {
        if let FnArg::Typed(pat) = input {
            let ty = &pat.ty;
            let id = quote::format_ident!("p{}", idx);
            args.push(id.clone());
            // requires.push(quote! { std::any::TypeId::of::<#ty>()});
            requires.push(quote! { <#ty as #crate_name::core::MiwaId>::component_id()});
            extractors.push(quote! {
                let #id = <#ty as #crate_name::core::FromMiwaContext>::from_context(&context)?;
            });
        }
    }

    let provides = provides(extension);

    quote! {
        #[allow(non_camel_case_types)]
        #def_struct


        #[async_trait::async_trait]
        impl #crate_name::core::ExtensionFactory for #ident {

            fn name(&self) -> &str {
                #extension_name
            }

            async fn init(&self, context : &#crate_name::core::MiwaContext) -> #crate_name::core::MiwaResult<Box<dyn #crate_name::core::Extension>> {
               #(#extractors)*
               #item_fn
               let extension = #ident(#(#args),*).await?;

               Ok(Box::new(extension))
            }

            fn requires(&self) -> Vec<std::any::TypeId> {
                vec![#(#requires),*]
            }

            fn exposes(&self) -> Vec<std::any::TypeId> {
                vec![#(#provides),*]
            }
        }
    }.into()
}

fn provides(extension: &Extension) -> Vec<TokenStream> {
    extension
        .provides
        .0
        .iter()
        .flat_map(|path| path.segments.last())
        .map(|segment| segment.ident.clone())
        .map(|inner_ident| quote! { std::any::TypeId::of::<#inner_ident>()})
        .collect()
}
