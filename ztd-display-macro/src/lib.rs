#![feature(coverage_attribute)]

////////////////////////////////////////////////////////////////////////////////////////////////////

use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{
    parse2, Attribute, ExprBlock, ExprCall, ExprClosure, ExprPath, Fields, FieldsNamed,
    FieldsUnnamed, Generics, Ident, Item, ItemEnum, ItemStruct, LitStr, Variant,
};

////////////////////////////////////////////////////////////////////////////////////////////////////

fn read_strategy_from_attribute(attribute: &Attribute) -> Option<Strategy> {
    if let Ok(message) = attribute.parse_args::<LitStr>() {
        return Some(Strategy::Message(message));
    }

    if let Ok(closure) = attribute.parse_args::<ExprClosure>() {
        return Some(Strategy::Closure(closure));
    }

    if let Ok(block) = attribute.parse_args::<ExprBlock>() {
        return Some(Strategy::Block(block));
    }

    if let Ok(call) = attribute.parse_args::<ExprCall>() {
        return Some(Strategy::Call(call));
    }

    if let Ok(call) = attribute.parse_args::<ExprPath>() {
        return Some(Strategy::Path(call));
    }

    None
}

////////////////////////////////////////////////////////////////////////////////////////////////////

fn read_strategy_from_attributes<'a, T>(mut iterator: T) -> Option<Strategy>
where
    T: Iterator<Item = &'a Attribute>,
{
    let attribute = match iterator.find(|attribute| attribute.path().is_ident("Display")) {
        Some(attribute) => attribute,
        None => return None,
    };

    let strategy = read_strategy_from_attribute(attribute);

    if strategy.is_none() {
        panic!("Unsupported strategy")
    }

    strategy
}

////////////////////////////////////////////////////////////////////////////////////////////////////

fn write_display_impl(generics: &Generics, name: &Ident, r#impl: TokenStream) -> TokenStream {
    let (impl_generics, type_generics, where_clause) = generics.split_for_impl();

    quote!(
        impl #impl_generics ::core::fmt::Display for #name #type_generics #where_clause {
            fn fmt(&self, formatter: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                #r#impl
            }
        }
    )
}

////////////////////////////////////////////////////////////////////////////////////////////////////

fn write_display_named_fields<T>(
    name: &Ident,
    fields: &FieldsNamed,
    strategy: &Option<Strategy>,
    r#impl: T,
) -> TokenStream
where
    T: FnOnce(TokenStream) -> TokenStream,
{
    match strategy {
        Some(Strategy::Message(message)) => r#impl(quote!(write!(formatter, #message))),
        Some(Strategy::Closure(closure)) => r#impl(quote!(write!(formatter, "{}", (#closure)()))),
        Some(Strategy::Block(block)) => r#impl(quote!(write!(formatter, "{}", #block))),
        Some(Strategy::Call(call)) => r#impl(quote!(write!(formatter, "{}", #call))),
        Some(Strategy::Path(path)) => {
            let idents = fields.named.iter().map(|field| &field.ident);

            r#impl(quote!(write!(formatter, "{}", #path(#(#idents),*))))
        }
        None => {
            let assignments = fields.named.iter().map(|field| {
                let ident = &field.ident;

                quote!(
                    .field(stringify!(#ident), &#ident)
                )
            });

            r#impl(quote!(
                formatter
                    .debug_struct(stringify!(#name))
                    #(#assignments)*
                .finish()
            ))
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

fn write_display_unnamed_fields<T>(
    name: &Ident,
    fields: &FieldsUnnamed,
    strategy: &Option<Strategy>,
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

    match strategy {
        Some(Strategy::Message(message)) => r#impl(quote!(
            (#field_idents) => write!(formatter, #message)
        )),
        Some(Strategy::Closure(closure)) => r#impl(quote!(
            (#field_idents) => write!(formatter, "{}", (#closure)())
        )),
        Some(Strategy::Block(block)) => r#impl(quote!(
            (#field_idents) => write!(formatter, "{}", #block)
        )),
        Some(Strategy::Call(call)) => r#impl(quote!(
            (#field_idents) => write!(formatter, "{}", #call)
        )),
        Some(Strategy::Path(path)) => r#impl(quote!(
            (#field_idents) => write!(formatter, "{}", #path(#field_idents))
        )),
        None => {
            let assignments = if fields.unnamed.len() == 1 {
                let ident = format_ident!("value");

                quote!(.field(#ident))
            } else {
                let fields = fields.unnamed.iter().enumerate().map(|(index, _field)| {
                    let field_ident = format_ident!("value{}", index);

                    quote!(.field(#field_ident))
                });

                quote!(#(#fields)*)
            };

            r#impl(quote!(
                (#field_idents) =>
                    formatter
                    .debug_tuple(stringify!(#name))
                    #assignments
                .finish()
            ))
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

fn write_display_unit_fields(name: &Ident, strategy: &Option<Strategy>) -> TokenStream {
    match strategy {
        Some(Strategy::Message(message)) => quote!(write!(formatter, #message)),
        Some(Strategy::Closure(closure)) => {
            quote!(write!(formatter, "{}", (#closure)()))
        }
        Some(Strategy::Block(block)) => {
            quote!(write!(formatter, "{}", #block))
        }
        Some(Strategy::Call(call)) => {
            quote!(write!(formatter, "{}", #call))
        }
        Some(Strategy::Path(path)) => {
            quote!(write!(formatter, "{}", #path()))
        }
        None => quote!(formatter.debug_struct(stringify!(#name)).finish()),
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

enum Strategy {
    Message(LitStr),
    Closure(ExprClosure),
    Block(ExprBlock),
    Call(ExprCall),
    Path(ExprPath),
}

////////////////////////////////////////////////////////////////////////////////////////////////////

struct EnumVariantData {
    strategy: Option<Strategy>,
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
            data.variants.push(EnumVariantData {
                strategy: read_strategy_from_attributes(variant.attrs.iter()),
            });
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
            ),
        )
    }

    fn write_variant(&self, variant: &Variant, variant_index: usize) -> TokenStream {
        #[coverage(off)]
        fn get_variant_data<'a>(
            data: &'a EnumData<'a>,
            variant_index: usize,
        ) -> &'a EnumVariantData {
            match data.variants.get(variant_index) {
                Some(variant_data) => variant_data,
                None => unreachable!(),
            }
        }

        let variant_data = get_variant_data(self, variant_index);

        let variant_ident = &variant.ident;

        match &variant.fields {
            Fields::Named(fields) => {
                let field_idents = fields.named.iter().map(|field| &field.ident);

                write_display_named_fields(
                    &variant.ident,
                    fields,
                    &variant_data.strategy,
                    |tokens| {
                        quote!(
                            Self::#variant_ident { #(#field_idents),* } => #tokens
                        )
                    },
                )
            }
            Fields::Unnamed(fields) => write_display_unnamed_fields(
                &variant.ident,
                fields,
                &variant_data.strategy,
                |tokens| {
                    quote!(
                        Self::#variant_ident #tokens
                    )
                },
            ),
            Fields::Unit => {
                let r#impl = write_display_unit_fields(&variant.ident, &variant_data.strategy);

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
    strategy: Option<Strategy>,
}

impl<'a> StructData<'a> {
    fn read(ast: &'a ItemStruct) -> Self {
        Self {
            ast,
            strategy: read_strategy_from_attributes(ast.attrs.iter()),
        }
    }

    fn write(self) -> TokenStream {
        let r#impl = match &self.ast.fields {
            Fields::Named(fields) => {
                let field_idents = fields.named.iter().map(|field| &field.ident);

                write_display_named_fields(&self.ast.ident, fields, &self.strategy, |tokens| {
                    quote!(
                        match self {
                            Self { #(#field_idents),* } => #tokens
                        }
                    )
                })
            }
            Fields::Unnamed(fields) => {
                write_display_unnamed_fields(&self.ast.ident, fields, &self.strategy, |tokens| {
                    quote!(
                        match self {
                            Self #tokens
                        }
                    )
                })
            }
            Fields::Unit => write_display_unit_fields(&self.ast.ident, &self.strategy),
        };

        write_display_impl(&self.ast.generics, &self.ast.ident, r#impl)
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

enum Data<'a> {
    Enum(EnumData<'a>),
    Struct(StructData<'a>),
}

impl<'a> Data<'a> {
    fn read(item: &'a Item) -> Self {
        match item {
            Item::Enum(r#enum) => Self::Enum(EnumData::read(r#enum)),
            Item::Struct(r#struct) => Self::Struct(StructData::read(r#struct)),
            _ => panic!("Unsupported item"),
        }
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
