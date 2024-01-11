use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse_macro_input, Data, DataStruct, DeriveInput, Fields, FieldsNamed};

#[proc_macro_derive(SquareType)]
pub fn square_type_derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    assert!(
        match ast.data {
            Data::Struct(DataStruct {
                fields: Fields::Unit,
                ..
            }) => true,
            _ => false,
        },
        "Can only derive 'SquareType' on empty structs."
    );
    let name = ast.ident;

    let gen = quote! {
        impl Default for #name {
            fn default() -> Self {
                Self
            }
        }
        impl SquareType for #name {}
        impl Component for #name {
            type Storage = TableStorage;
        }
    };
    gen.into()
}

#[proc_macro_derive(Builder)]
pub fn builder_derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let name = &ast.ident;
    let (impl_generics, generics, where_clause) = &ast.generics.split_for_impl();
    let fields = match ast.data {
        Data::Struct(DataStruct {
            fields: Fields::Named(FieldsNamed { named: fields, .. }),
            ..
        }) => fields,
        _ => panic!("Can only derive 'Builder' for Structs with named parameters."),
    };

    let mut block = proc_macro2::TokenStream::new();
    for field in fields {
        let name = &field.ident;
        let ty = &field.ty;
        let func = quote! {
            pub fn #name (mut self, value: #ty) -> Self {
                self. #name = value;
                self
            }
        };
        func.to_tokens(&mut block);
    }
    let gen = quote! {
        impl #impl_generics #name #generics #where_clause {
            #block
        }
    };
    gen.into()
}
