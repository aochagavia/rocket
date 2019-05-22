//! A crate to custom derive `Position` and `Advance` for any type that has a field named `vector`

extern crate proc_macro;

use crate::proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(Advance)]
pub fn advance_derive(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();

    let name = &ast.ident;
    let gen = quote! {
        impl Advance for #name {
            fn direction(&self) -> f32 {
                self.vector.direction
            }

            fn direction_mut(&mut self) -> &mut f32 {
                &mut self.vector.direction
            }
        }
    };
    gen.into()
}

#[proc_macro_derive(Position)]
pub fn position_derive(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();

    let name = &ast.ident;
    let gen = quote! {
        impl Position for #name {
            fn x(&self) -> f32 { self.vector.position.x }
            fn y(&self) -> f32 { self.vector.position.y }
            fn x_mut(&mut self) -> &mut f32 { &mut self.vector.position.x }
            fn y_mut(&mut self) -> &mut f32 { &mut self.vector.position.y }
        }
    };
    gen.into()
}
