#![feature(no_coverage)]
#![feature(stmt_expr_attributes)]

////////////////////////////////////////////////////////////////////////////////////////////////////

use proc_macro2::{TokenStream};
use quote::{format_ident, quote};
use syn::{parse2, Fields, FieldsUnnamed, FieldsNamed, Generics, Ident, Item, ItemEnum, ItemStruct, LitStr, Variant};
use ztd_coverage::assume_full_coverage;

////////////////////////////////////////////////////////////////////////////////////////////////////

fn write_display_impl(
    generics: &Generics,
    name: &Ident,
    r#impl: TokenStream,
) -> TokenStream {
    let (impl_generics, type_generics, where_clause) = generics.split_for_impl();

    quote!(
        impl #impl_generics ::std::fmt::Display for #name #type_generics #where_clause {
            fn fmt(&self, formatter: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                #r#impl
            }
        }
    )
}

////////////////////////////////////////////////////////////////////////////////////////////////////

fn write_display_named_fields<T>(
    name: &Ident,
    fields: &FieldsNamed,
    message: &Option<LitStr>,
    r#impl: T,
) -> TokenStream
where
    T: FnOnce(TokenStream) -> TokenStream,
{
    match &message {
        Some(message) => {
            r#impl(
                quote!(
                    write!(formatter, #message)
                )
            )
        }
        None => {
            let assignments = fields.named.iter().map(|field| {
                let ident = &field.ident;

                quote!(
                    .field(stringify!(#ident), &#ident)
                )
            });

            r#impl(
                quote!(
                    formatter
                        .debug_struct(stringify!(#name))
                        #(#assignments)*
                    .finish()
                )
            )
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

fn write_display_unnamed_fields<T>(
    name: &Ident,
    fields: &FieldsUnnamed,
    message: &Option<LitStr>,
    r#impl: T,
) -> TokenStream
where
    T: FnOnce(TokenStream) -> TokenStream,
{
    let field_idents = if fields.unnamed.len() == 1 {
        let ident = format_ident!("value");

        quote!(#ident)
    } else {
        let fields = fields
            .unnamed
            .iter()
            .enumerate()
            .map(|(index, _field)| format_ident!("value{}", index));

        quote!(#(#fields),*)
    };

    match message {
        Some(message) => {
            r#impl(
                quote!(
                    (#field_idents) => write!(formatter, #message)
                )
            )
        }
        None => {
            let assignments = if fields.unnamed.len() == 1 {
                let ident = format_ident!("value");

                quote!(.field(#ident))
            } else {
                let fields =
                    fields.unnamed.iter().enumerate().map(|(index, _field)| {
                        let field_ident = format_ident!("value{}", index);

                        quote!(.field(#field_ident))
                    });

                quote!(#(#fields)*)
            };

            r#impl(
                quote!(
                    (#field_idents) =>
                        formatter
                        .debug_tuple(stringify!(#name))
                        #assignments
                    .finish()
                )
            )
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

fn write_display_unit_fields(
    name: &Ident,
    message: &Option<LitStr>,
) -> TokenStream {
    match message {
        Some(message) => quote!(write!(formatter, #message)),
        None => quote!(formatter.debug_struct(stringify!(#name)).finish()),
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

struct EnumVariantData {
    message: Option<LitStr>,
}

////////////////////////////////////////////////////////////////////////////////////////////////////

struct EnumData<'a> {
    ast: &'a ItemEnum,
    variants: Vec<EnumVariantData>,
}

impl<'a> EnumData<'a> {
    fn read(ast: &'a ItemEnum) -> Self {
        let mut data = Self {
            ast,
            variants: Vec::with_capacity(ast.variants.len()),
        };

        for variant in &data.ast.variants {
            let mut variant_data = EnumVariantData { message: None };

            if let Some(attribute) = variant
                .attrs
                .iter()
                .find(|attribute| attribute.path().is_ident("Display"))
            {
                variant_data.message = Some(attribute.parse_args::<LitStr>().unwrap());
            }

            data.variants.push(variant_data);
        }

        data
    }

    fn write(self) -> TokenStream {
        let variants = self
            .ast
            .variants
            .iter()
            .enumerate()
            .map(|(variant_index, variant)| self.write_variant(variant, variant_index));

        write_display_impl(
            &self.ast.generics,
            &self.ast.ident,
            quote!(
                match self {
                    #(#variants),*
                }
            )
        )
    }

    fn write_variant(&self, variant: &Variant, variant_index: usize) -> TokenStream {
        let variant_data = assume_full_coverage!(match self.variants.get(variant_index) {
            Some(variant_data) => variant_data,
            None => unreachable!(),
        });

        let variant_ident = &variant.ident;

        match &variant.fields {
            Fields::Named(fields) => {
                let field_idents = fields.named.iter().map(|field| &field.ident);

                write_display_named_fields(
                    &variant.ident,
                    fields,
                    &variant_data.message,
                    |tokens| quote!(
                        Self::#variant_ident { #(#field_idents),* } => #tokens
                    )
                )
            }
            Fields::Unnamed(fields) => {
                write_display_unnamed_fields(
                    &variant.ident,
                    fields,
                    &variant_data.message,
                    |tokens| quote!(
                        Self::#variant_ident #tokens
                    )
                )
            }
            Fields::Unit => {
                let r#impl = write_display_unit_fields(
                    &variant.ident,
                    &variant_data.message,
                );

                quote!(
                    Self::#variant_ident => #r#impl
                )
            }
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

struct StructData<'a> {
    ast: &'a ItemStruct,
    message: Option<LitStr>,
}

impl<'a> StructData<'a> {
    fn read(ast: &'a ItemStruct) -> Self {
        let mut data = Self { ast, message: None };

        if let Some(attribute) = ast
            .attrs
            .iter()
            .find(|attribute| attribute.path().is_ident("Display"))
        {
            data.message = Some(attribute.parse_args::<LitStr>().unwrap());
        }

        data
    }

    fn write(self) -> TokenStream {
        let r#impl = match &self.ast.fields {
            Fields::Named(fields) => {
                let field_idents = fields.named.iter().map(|field| &field.ident);

                write_display_named_fields(
                    &self.ast.ident,
                    fields,
                    &self.message,
                    |tokens| quote!(
                        match self {
                            Self { #(#field_idents),* } => #tokens
                        }
                    )
                )
            }
            Fields::Unnamed(fields) => {
                write_display_unnamed_fields(
                    &self.ast.ident,
                    fields,
                    &self.message,
                    |tokens| quote!(
                        match self {
                            Self #tokens
                        }
                    )
                )
            }
            Fields::Unit => write_display_unit_fields(
                &self.ast.ident,
                &self.message,
            ),
        };

        write_display_impl(
            &self.ast.generics,
            &self.ast.ident,
            r#impl,
        )
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

enum Data<'a> {
    Enum(EnumData<'a>),
    Struct(StructData<'a>),
}

impl<'a> Data<'a> {
    fn read(item: &'a Item) -> Self {
        assume_full_coverage!(match item {
            Item::Enum(r#enum) => Self::Enum(EnumData::read(r#enum)),
            Item::Struct(r#struct) => Self::Struct(StructData::read(r#struct)),
            _ => panic!("Unsupported item"),
        })
    }

    fn write(self) -> TokenStream {
        match self {
            Self::Enum(r#enum) => r#enum.write(),
            Self::Struct(r#struct) => r#struct.write(),
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

pub struct Macro;

impl Macro {
    pub fn handle(stream: TokenStream) -> TokenStream {
        Data::read(&parse2::<Item>(stream).unwrap()).write()
    }
}
