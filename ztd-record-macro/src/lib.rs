#![cfg_attr(coverage_nightly, feature(no_coverage))]

////////////////////////////////////////////////////////////////////////////////////////////////////

use proc_macro2::{Span, TokenStream};
use quote::{format_ident, quote};
use std::collections::hash_map::RandomState;
use std::collections::HashSet;
use std::mem::replace;
use syn::punctuated::Punctuated;
use syn::token::Pub;
use syn::{parse2, Fields, Index, ItemStruct, Path, PathSegment, Type, Visibility};

////////////////////////////////////////////////////////////////////////////////////////////////////

enum ExclusiveFieldModifier {
    Skip,
    Flatten,
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Default)]
struct FieldModifier {
    exclusive: Option<ExclusiveFieldModifier>,
}

impl FieldModifier {
    fn set_exclusive(&mut self, exclusive: ExclusiveFieldModifier) {
        self.exclusive = Some(exclusive)
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Default)]
struct FieldDetails {
    attribute_index: Option<usize>,
    modifier: FieldModifier,
}

////////////////////////////////////////////////////////////////////////////////////////////////////

struct Data<'a> {
    ast: &'a ItemStruct,
    fields: Vec<FieldDetails>,
}

impl<'a> Data<'a> {
    fn read(ast: &'a ItemStruct) -> Self {
        let mut data = Self {
            ast,
            fields: Vec::with_capacity(ast.fields.len()),
        };

        for field in &ast.fields {
            let mut details = FieldDetails::default();

            if let Some((attribute_index, attribute)) = field
                .attrs
                .iter()
                .enumerate()
                .find(|(_attribute_index, attribute)| attribute.path().is_ident("Record"))
            {
                details.attribute_index = Some(attribute_index);

                attribute
                    .parse_nested_meta(|meta| {
                        if meta.path.is_ident("flatten") {
                            details
                                .modifier
                                .set_exclusive(ExclusiveFieldModifier::Flatten);
                        }

                        if meta.path.is_ident("skip") {
                            details.modifier.set_exclusive(ExclusiveFieldModifier::Skip);
                        }

                        Ok(())
                    })
                    .expect("Valid nested meta parsing");
            }

            data.fields.push(details);
        }

        data
    }

    fn write(self) -> TokenStream {
        let visibility = self.ast.vis.clone();
        let struct_name = self.ast.ident.clone();
        let (impl_generics, type_generics, where_clause) = self.ast.generics.split_for_impl();
        let record_struct_name = format_ident!("{}Record", self.ast.ident);

        let r#impl = match self.ast.fields {
            Fields::Named(ref fields) => {
                let fields = fields
                    .named
                    .pairs()
                    .enumerate()
                    .flat_map(|(field_index, pair)| {
                        let field = pair.into_value();

                        let ident = field.ident.as_ref().expect("Valid identifier");
                        let details = self.fields.get(field_index).expect("FieldDetails");

                        match details.modifier.exclusive {
                            Some(ExclusiveFieldModifier::Flatten) => Some(quote!(
                                #ident: self.#ident.into_record()
                            )),
                            Some(ExclusiveFieldModifier::Skip) => None,
                            None => Some(quote!(
                                #ident: self.#ident
                            )),
                        }
                    });

                quote!(
                    impl #impl_generics #struct_name #type_generics #where_clause {
                        #visibility fn into_record(self) -> #record_struct_name #type_generics {
                            #record_struct_name {
                                #(#fields),*
                            }
                        }
                    }
                )
            }
            Fields::Unnamed(ref fields) => {
                let fields = fields
                    .unnamed
                    .pairs()
                    .enumerate()
                    .flat_map(|(field_index, _pair)| {
                        let details = self.fields.get(field_index).unwrap();

                        let index = Index {
                            index: field_index as u32,
                            span: Span::call_site(),
                        };

                        match details.modifier.exclusive {
                            Some(ExclusiveFieldModifier::Flatten) => Some(quote!(
                                self.#index.into_record()
                            )),
                            Some(ExclusiveFieldModifier::Skip) => None,
                            None => Some(quote!(
                                self.#index
                            )),
                        }
                    });

                quote!(
                    impl #impl_generics #struct_name #type_generics #where_clause {
                        #visibility fn into_record(self) -> #record_struct_name #type_generics {
                            #record_struct_name(
                                #(#fields),*
                            )
                        }
                    }
                )
            }
            Fields::Unit => {
                quote!(
                    impl #impl_generics #struct_name #type_generics #where_clause {
                        #visibility fn into_record(self) -> #record_struct_name {
                            #record_struct_name
                        }
                    }
                )
            }
        };

        let mut ast = self.ast.clone();

        ast.ident = record_struct_name;

        let mut skipped_fields = HashSet::<usize, RandomState>::default();

        for (field_index, field) in ast.fields.iter_mut().enumerate() {
            field.vis = Visibility::Public(Pub::default());

            let details = self.fields.get(field_index).unwrap();

            if let Some(attribute_index) = details.attribute_index {
                field.attrs.swap_remove(attribute_index);
            }

            match details.modifier.exclusive {
                Some(ExclusiveFieldModifier::Flatten) => {
                    if let Type::Path(type_path) = &mut field.ty {
                        #[cfg_attr(coverage_nightly, no_coverage)]
                        fn get_last_segment_mut(path: &mut Path) -> &mut PathSegment {
                            match path.segments.last_mut() {
                                Some(last) => last,
                                None => unreachable!(),
                            }
                        }

                        let mut segment = get_last_segment_mut(&mut type_path.path);
                        segment.ident = format_ident!("{}Record", segment.ident);
                    } else {
                        panic!("Cannot flatten {:?}", field.ty)
                    }
                }
                Some(ExclusiveFieldModifier::Skip) => {
                    skipped_fields.insert(field_index);
                }
                None => {}
            }
        }

        let punctuation = match &mut ast.fields {
            Fields::Named(ref mut fields) => Some(&mut fields.named),
            Fields::Unnamed(ref mut fields) => Some(&mut fields.unnamed),
            Fields::Unit => None,
        };

        if let Some(punctuation) = punctuation {
            for (pair_index, pair) in replace(punctuation, Punctuated::new())
                .into_pairs()
                .enumerate()
            {
                if !skipped_fields.contains(&pair_index) {
                    punctuation.push(pair.into_value());
                }
            }
        }

        quote!(
            #ast

            #r#impl
        )
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

pub struct Macro;

impl Macro {
    pub fn handle(stream: TokenStream) -> TokenStream {
        Data::read(&parse2::<ItemStruct>(stream).unwrap()).write()
    }
}
