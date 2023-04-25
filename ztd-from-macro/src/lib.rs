#![cfg_attr(coverage_nightly, feature(no_coverage))]

////////////////////////////////////////////////////////////////////////////////////////////////////

use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{
    parse2, Fields, FieldsNamed, FieldsUnnamed, Generics, Ident, Index, Item, ItemEnum, ItemStruct,
    Meta, Variant,
};

////////////////////////////////////////////////////////////////////////////////////////////////////

fn write_from_impl(
    generics: &Generics,
    name: &Ident,
    from: TokenStream,
    r#impl: TokenStream,
) -> TokenStream {
    let (impl_generics, type_generics, where_clause) = generics.split_for_impl();

    quote!(
        impl #impl_generics ::std::convert::From<#from> for #name #type_generics #where_clause {
            #r#impl
        }
    )
}

////////////////////////////////////////////////////////////////////////////////////////////////////

fn write_from_named_fields_impl(
    generics: &Generics,
    name: &Ident,
    r#impl: TokenStream,
    fields: &FieldsNamed,
) -> TokenStream {
    let types = fields.named.iter().map(|field| {
        let r#type = &field.ty;

        quote!(#r#type,)
    });

    let from_type = quote!((#(#types)*));

    let assignments = fields.named.iter().enumerate().map(|(index, field)| {
        let ident = &field.ident;
        let index = Index::from(index);

        quote!(
            #ident: value.#index
        )
    });

    write_from_impl(
        generics,
        name,
        from_type.clone(),
        quote!(
            fn from(value: #from_type) -> Self {
                #r#impl { #(#assignments),* }
            }
        ),
    )
}

////////////////////////////////////////////////////////////////////////////////////////////////////

fn write_from_unnamed_fields_impl(
    generics: &Generics,
    name: &Ident,
    r#impl: TokenStream,
    fields: &FieldsUnnamed,
) -> TokenStream {
    let mut field = fields.unnamed.iter();

    let (r#type, r#impl) = match (field.next(), field.next()) {
        (None, None) => (quote!(()), quote!(#r#impl())),
        (Some(field), None) => {
            let r#type = &field.ty;
            let ident = format_ident!("value");

            (quote!(#r#type), quote!(#r#impl(#ident)))
        }
        _ => {
            let types = fields.unnamed.iter().map(|field| &field.ty);
            let impls = fields.unnamed.iter().enumerate().map(|(index, _field)| {
                let index = Index::from(index);
                let ident = format_ident!("value");

                quote!(
                    #ident.#index
                )
            });

            (
                quote!((#(#types),*)),
                quote!(
                    #r#impl(#(#impls),*)
                ),
            )
        }
    };

    write_from_impl(
        generics,
        name,
        r#type.clone(),
        quote!(
            fn from(value: #r#type) -> Self {
                #r#impl
            }
        ),
    )
}

////////////////////////////////////////////////////////////////////////////////////////////////////

fn write_from_unit_fields_impl(
    generics: &Generics,
    name: &Ident,
    r#impl: TokenStream,
) -> TokenStream {
    write_from_impl(
        generics,
        name,
        quote!(()),
        quote!(
            fn from(value: ()) -> Self {
                #r#impl
            }
        ),
    )
}

////////////////////////////////////////////////////////////////////////////////////////////////////

fn write_from_fields_impl(
    generics: &Generics,
    name: &Ident,
    r#impl: TokenStream,
    fields: &Fields,
) -> TokenStream {
    match fields {
        Fields::Named(fields) => write_from_named_fields_impl(generics, name, r#impl, fields),
        Fields::Unnamed(fields) => write_from_unnamed_fields_impl(generics, name, r#impl, fields),
        Fields::Unit => write_from_unit_fields_impl(generics, name, r#impl),
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

enum EnumVariantDataModifier {
    Enabled,
    Skipped,
}

////////////////////////////////////////////////////////////////////////////////////////////////////

struct EnumVariantData {
    modifier: Option<EnumVariantDataModifier>,
}

impl EnumVariantData {
    fn set_modifier(&mut self, modifier: EnumVariantDataModifier) {
        self.modifier = Some(modifier)
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

struct EnumData<'a> {
    ast: &'a ItemEnum,
    variants: Vec<EnumVariantData>,
    named: bool,
    unnamed: bool,
    unit: bool,
}

impl<'a> EnumData<'a> {
    fn enable_named(&mut self) {
        self.named = true
    }

    fn enable_unnamed(&mut self) {
        self.unnamed = true
    }

    fn enable_unit(&mut self) {
        self.unit = true
    }

    fn read(ast: &'a ItemEnum) -> Self {
        let mut data = Self {
            ast,
            variants: Vec::with_capacity(ast.variants.len()),
            named: false,
            unnamed: false,
            unit: false,
        };

        if let Some(attribute) = ast
            .attrs
            .iter()
            .find(|attribute| attribute.path().is_ident("From"))
        {
            attribute
                .parse_nested_meta(|meta| {
                    if meta.path.is_ident("all") {
                        data.enable_named();
                        data.enable_unnamed();
                        data.enable_unit();
                    }

                    if meta.path.is_ident("named") {
                        data.enable_named();
                    }

                    if meta.path.is_ident("unnamed") {
                        data.enable_unnamed();
                    }

                    if meta.path.is_ident("unit") {
                        data.enable_unit();
                    }

                    Ok(())
                })
                .unwrap();
        }

        for variant in &ast.variants {
            let mut variant_data = EnumVariantData { modifier: None };

            if let Some(attribute) = variant
                .attrs
                .iter()
                .find(|attribute| attribute.path().is_ident("From"))
            {
                if matches!(&attribute.meta, Meta::Path(_path)) {
                    variant_data.set_modifier(EnumVariantDataModifier::Enabled)
                } else {
                    attribute
                        .parse_nested_meta(|meta| {
                            if meta.path.is_ident("enable") {
                                variant_data.set_modifier(EnumVariantDataModifier::Enabled)
                            }

                            if meta.path.is_ident("skip") {
                                variant_data.set_modifier(EnumVariantDataModifier::Skipped);
                            }

                            Ok(())
                        })
                        .unwrap();
                }
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
            .flat_map(|(variant_index, variant)| self.write_variant(variant_index, variant));

        quote!(
            #(#variants)*
        )
    }

    fn write_variant(&self, variant_index: usize, variant: &Variant) -> Option<TokenStream> {
        let variant_ident = &variant.ident;

        #[cfg_attr(coverage_nightly, no_coverage)]
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

        if matches!(
            variant_data.modifier,
            Some(EnumVariantDataModifier::Skipped)
        ) {
            return None;
        }

        let enabled_with_all = match &variant.fields {
            Fields::Named(_) => self.named,
            Fields::Unnamed(_) => self.unnamed,
            Fields::Unit => self.unit,
        };

        if !enabled_with_all
            && !matches!(
                variant_data.modifier,
                Some(EnumVariantDataModifier::Enabled)
            )
        {
            return None;
        }

        Some(write_from_fields_impl(
            &self.ast.generics,
            &self.ast.ident,
            quote!(Self::#variant_ident),
            &variant.fields,
        ))
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

struct StructData<'a> {
    ast: &'a ItemStruct,
}

impl<'a> StructData<'a> {
    fn read(ast: &'a ItemStruct) -> Self {
        Self { ast }
    }

    fn write(self) -> TokenStream {
        write_from_fields_impl(
            &self.ast.generics,
            &self.ast.ident,
            quote!(Self),
            &self.ast.fields,
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
