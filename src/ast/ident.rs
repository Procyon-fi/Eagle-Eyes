use proc_macro2::Span;

pub struct Ident(String, Span);

impl From<&syn::Ident> for Ident {
    fn from(value: &syn::Ident) -> Self {
        Self(value.to_string(), value.span())
    }
}
