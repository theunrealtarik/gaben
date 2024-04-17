use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(PeriodicPunishment)]
pub fn derive_periodic_punishment(input: TokenStream) -> TokenStream {
    let DeriveInput { ident, .. } = parse_macro_input!(input as DeriveInput);

    let expanded = quote! {
        impl #ident {
            pub fn new() -> Self {
                Self {
                    schedule: PunishmentSchedule::Periodic,
                    name: stringify!(#ident).to_string(),
                    ..Default::default()
                }
            }
        }
    };

    TokenStream::from(expanded)
}
