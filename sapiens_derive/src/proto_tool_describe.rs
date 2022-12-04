use darling::FromDeriveInput;
use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::Expr;

/// A derive macro for the `ProtoToolDescribe` trait.
#[derive(Debug, FromDeriveInput)]
#[darling(attributes(tool), supports(struct_named), forward_attrs(doc))]
struct DeriveReceiver {
    ident: syn::Ident,
    attrs: Vec<syn::Attribute>,
    generics: syn::Generics,

    name: Option<String>,

    input: syn::Path,
    output: syn::Path,
}

impl ToTokens for DeriveReceiver {
    fn to_tokens(&self, out: &mut proc_macro2::TokenStream) {
        let DeriveReceiver {
            ref ident,
            ref attrs,
            ref generics,
            ref name,
            ref input,
            ref output,
            ..
        } = *self;

        let (imp, ty, wher) = generics.split_for_impl();

        let doc = attrs
            .iter()
            .filter(|attr| attr.path().is_ident("doc"))
            .filter_map(|attr| match &