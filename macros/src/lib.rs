use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(ContinuousPunishment)]
pub fn derive_continuous_punishment(input: TokenStream) -> TokenStream {
    let DeriveInput { ident, .. } = parse_macro_input!(input as DeriveInput);

    let expanded = quote! {
        impl Default for #ident {
            fn default() -> Self {
                Self {
                    ..Default::default()
                }
            }
        }

        impl #ident {
            pub fn new() -> Self {
                Self {
                    schedule: PunishmentSchedule::Continuous,
                    ..Default::default()
                }
            }
        }
    };

    TokenStream::from(expanded)
}

#[proc_macro_derive(PeriodicPunishment)]
pub fn derive_periodic_punishment(input: TokenStream) -> TokenStream {
    let DeriveInput { ident, .. } = parse_macro_input!(input as DeriveInput);

    let expanded = quote! {
        impl Default for #ident {
            fn default() -> Self {
                Self {
                    ..Default::default()
                }
            }
        }

        impl #ident {
            pub fn new() -> Self {
                Self {
                    schedule: PunishmentSchedule::Periodic,
                    ..Default::default()
                }
            }
        }
    };

    TokenStream::from(expanded)
}
