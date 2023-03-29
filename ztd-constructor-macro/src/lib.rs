use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{parse2, Field, Fields, ItemStruct};

////////////////////////////////////////////////////////////////////////////////////////////////////

struct Data<'a> {
    ast: &'a ItemStruct,
}

impl<'a> Data<'a> {
    fn read(ast: &'a ItemStruct) -> Self {
        Self { ast }
    }

    fn write(self) -> TokenStream {
        let visibility = &self.ast.vis;

        let constructor = match self.ast.fields {
            Fields::Named(ref fields) => {
                let arguments = self.write_arguments(fields.named.iter());
                let initializations = self.write_initializations(fields.named.iter());

                quote!(
                    #visibility fn new(#(#arguments),*) -> Self {
                        Self {
                            #(#initializations),*
                        }
                    }
                )
            }
            Fields::Unnamed(ref fields) => {
                let arguments = self.write_arguments(fields.unnamed.iter());
                let initializations = self.write_initializations(fields.unnamed.iter());

                quote!(
                    #visibility fn new(#(#arguments),*) -> Self {
                        Self(#(#initializations),*)
                    }
                )
            }
            Fields::Unit => {
                quote!(
                    #visibility fn new() -> Self {
                        Self
                    }
                )
            }
        };

        let struct_name = &self.ast.ident;
        let (impl_generics, type_generics, where_clause) = self.ast.generics.split_for_impl();

        quote!(
            impl #impl_generics #struct_name #type_generics #where_clause {
                #constructor
            }
        )
    }

    fn write_arguments<'b, T>(&self, iterator: T) -> impl Iterator<Item = TokenStream> + 'b
    where
        T: Iterator<Item = &'b Field> + 'b,
    {
        iterator.enumerate().map(|(index, field)| {
            let ident = match &field.ident {
                Some(ident) => ident.clone(),
                None => format_ident!("value{}", index),
            };

            let r#type = &field.ty;

            quote!(
                #ident: #r#type
            )
        })
    }

    fn write_initializations<'b, T>(&self, iterator: T) -> impl Iterator<Item = TokenStream> + 'b
    where
        T: Iterator<Item = &'b Field> + 'b,
    {
        iterator.enumerate().map(|(index, field)| {
            let ident = match &field.ident {
                Some(ident) => ident.clone(),
                None => format_ident!("value{}", index),
            };

            quote!(
                #ident
            )
        })
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

pub struct Macro;

impl Macro {
    pub fn handle(stream: TokenStream) -> TokenStream {
        Data::read(&parse2::<ItemStruct>(stream).unwrap()).write()
    }
}
