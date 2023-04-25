use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{parse2, Field, Fields, ItemStruct, Visibility};

////////////////////////////////////////////////////////////////////////////////////////////////////

struct Data<'a> {
    ast: &'a ItemStruct,
    visibility: Option<Visibility>,
}

impl<'a> Data<'a> {
    fn read(ast: &'a ItemStruct) -> Self {
        Self {
            ast,
            visibility: ast
                .attrs
                .iter()
                .find(|attribute| attribute.path().is_ident("Constructor"))
                .and_then(|attribute| {
                    let mut visibility = None;

                    let result = attribute.parse_nested_meta(|meta| {
                        if meta.path.is_ident("visibility") {
                            let value = meta.value()?;

                            visibility = Some(value.parse::<Visibility>().unwrap());

                            // Visibility::Inherited catches everything when being parsed
                            if matches!(visibility, Some(Visibility::Inherited)) {
                                return Err(meta.error("Unknown visibility"));
                            }

                            Ok(())
                        } else {
                            Err(meta.error("Unknown attribute"))
                        }
                    });

                    if let Err(error) = result {
                        panic!("{}", error)
                    }

                    visibility
                }),
        }
    }

    fn write(self) -> TokenStream {
        let visibility = self.visibility.as_ref().or(Some(&self.ast.vis));

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
