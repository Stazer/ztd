#![feature(coverage_attribute)]
#![feature(stmt_expr_attributes)]
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::token::{And, Mut};
use syn::{parse2, Index, ItemStruct, Type, TypeReference};

////////////////////////////////////////////////////////////////////////////////////////////////////

enum FieldDataModifier {
    Enabled,
    Skipped,
}

////////////////////////////////////////////////////////////////////////////////////////////////////

enum FieldDataAccessorReturnModifier {
    Automatic,
    Reference,
    Copy,
}

////////////////////////////////////////////////////////////////////////////////////////////////////

fn is_copy(r#type: &Type) -> bool {
    let path = match r#type {
        Type::Path(path) => path,
        Type::Tuple(tuple) if tuple.elems.is_empty() => return true,
        Type::Tuple(tuple) => match tuple.elems.iter().next() {
            Some(next) if tuple.elems.len() == 1 => return is_copy(next),
            _ => return false,
        },
        _ => return false,
    };

    let ident = match path.path.get_ident() {
        Some(ident) => ident,
        None => return false,
    };

    matches!(
        ident.to_string().as_str(),
        "i8" | "i16"
            | "i32"
            | "i64"
            | "i128"
            | "isize"
            | "u8"
            | "u16"
            | "u32"
            | "u64"
            | "u128"
            | "usize"
            | "f32"
            | "f64"
            | "char"
            | "bool"
    )
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Default)]
struct FieldData {
    accessor: Option<FieldDataModifier>,
    accessor_return: Option<FieldDataAccessorReturnModifier>,
    mutator: Option<FieldDataModifier>,
    setter: Option<FieldDataModifier>,
}

impl FieldData {
    fn set_accessor(&mut self, _data: &Data, accessor: FieldDataModifier) {
        self.accessor = Some(accessor)
    }

    fn set_accessor_return(
        &mut self,
        _data: &Data,
        accessor_return: FieldDataAccessorReturnModifier,
    ) {
        self.accessor_return = Some(accessor_return)
    }

    fn set_mutator(&mut self, _data: &Data, mutator: FieldDataModifier) {
        self.mutator = Some(mutator)
    }

    fn set_setter(&mut self, _data: &Data, setter: FieldDataModifier) {
        self.setter = Some(setter)
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

struct Data<'a> {
    ast: &'a ItemStruct,
    fields: Vec<FieldData>,
    accessors: bool,
    accessors_return: Option<FieldDataAccessorReturnModifier>,
    mutators: bool,
    setters: bool,
}

impl<'a> Data<'a> {
    fn enable_accessors(&mut self) {
        self.accessors = true;
    }

    fn set_accessors_return(&mut self, accessors_return: FieldDataAccessorReturnModifier) {
        self.accessors_return = Some(accessors_return);
    }

    fn enable_mutators(&mut self) {
        self.mutators = true;
    }

    fn enable_setters(&mut self) {
        self.setters = true;
    }

    fn read(ast: &'a ItemStruct) -> Self {
        let mut data = Self {
            ast,
            fields: Vec::with_capacity(ast.fields.len()),
            accessors: false,
            accessors_return: None,
            mutators: false,
            setters: false,
        };

        if let Some(attribute) = ast
            .attrs
            .iter()
            .find(|attribute| attribute.path().is_ident("Method"))
        {
            attribute
                .parse_nested_meta(|meta| {
                    if meta.path.is_ident("all") {
                        data.enable_accessors();
                        data.enable_mutators();
                        data.enable_setters();
                    }

                    if meta.path.is_ident("accessors") {
                        data.enable_accessors();
                    }

                    if meta.path.is_ident("accessors_return_automatic") {
                        data.set_accessors_return(FieldDataAccessorReturnModifier::Automatic);
                    }

                    if meta.path.is_ident("accessors_return_reference") {
                        data.set_accessors_return(FieldDataAccessorReturnModifier::Reference);
                    }

                    if meta.path.is_ident("accessors_return_copy") {
                        data.set_accessors_return(FieldDataAccessorReturnModifier::Copy);
                    }

                    if meta.path.is_ident("mutators") {
                        data.enable_mutators();
                    }

                    if meta.path.is_ident("setters") {
                        data.enable_setters();
                    }

                    Ok(())
                })
                .unwrap();
        }

        for field in &ast.fields {
            let mut field_data = FieldData::default();

            if let Some(attribute) = field
                .attrs
                .iter()
                .find(|attribute| attribute.path().is_ident("Method"))
            {
                attribute
                    .parse_nested_meta(|meta| {
                        if meta.path.is_ident("all") {
                            field_data.set_accessor(&data, FieldDataModifier::Enabled);
                            field_data.set_mutator(&data, FieldDataModifier::Enabled);
                            field_data.set_setter(&data, FieldDataModifier::Enabled);
                        }

                        if meta.path.is_ident("skip_all") {
                            field_data.set_accessor(&data, FieldDataModifier::Skipped);
                            field_data.set_mutator(&data, FieldDataModifier::Skipped);
                            field_data.set_setter(&data, FieldDataModifier::Skipped);
                        }

                        if meta.path.is_ident("accessor") {
                            field_data.set_accessor(&data, FieldDataModifier::Enabled);
                        }

                        if meta.path.is_ident("accessor_returns_automatic") {
                            field_data.set_accessor_return(
                                &data,
                                FieldDataAccessorReturnModifier::Automatic,
                            );
                        }

                        if meta.path.is_ident("accessor_returns_reference") {
                            field_data.set_accessor_return(
                                &data,
                                FieldDataAccessorReturnModifier::Reference,
                            );
                        }

                        if meta.path.is_ident("accessor_returns_copy") {
                            field_data
                                .set_accessor_return(&data, FieldDataAccessorReturnModifier::Copy);
                        }

                        if meta.path.is_ident("skip_accessor") {
                            field_data.set_accessor(&data, FieldDataModifier::Skipped);
                        }

                        if meta.path.is_ident("mutator") {
                            field_data.set_mutator(&data, FieldDataModifier::Enabled);
                        }

                        if meta.path.is_ident("skip_mutator") {
                            field_data.set_mutator(&data, FieldDataModifier::Skipped);
                        }

                        if meta.path.is_ident("setter") {
                            field_data.set_setter(&data, FieldDataModifier::Enabled);
                        }

                        if meta.path.is_ident("skip_setter") {
                            field_data.set_setter(&data, FieldDataModifier::Skipped);
                        }

                        Ok(())
                    })
                    .unwrap();
            }

            data.fields.push(field_data);
        }

        data
    }

    fn write(self) -> TokenStream {
        let accessors = self.write_accessors();
        let mutators = self.write_mutators();
        let setters = self.write_setters();

        let struct_name = &self.ast.ident;
        let (impl_generics, type_generics, where_clause) = self.ast.generics.split_for_impl();

        quote!(
            impl #impl_generics #struct_name #type_generics #where_clause {
                #accessors

                #mutators

                #setters
            }
        )
    }

    fn write_accessors(&self) -> TokenStream {
        let visibility = &self.ast.vis;

        let methods = self
            .ast
            .fields
            .iter()
            .enumerate()
            .flat_map(|(field_index, field)| {
                let field_data = self.fields.get(field_index).expect("existing field");

                if matches!(field_data.accessor, Some(FieldDataModifier::Skipped)) {
                    return None;
                }

                if !self.accessors
                    && !matches!(field_data.accessor, Some(FieldDataModifier::Enabled))
                {
                    return None;
                }

                let (method_ident, field_token) = match &field.ident {
                    Some(ident) => (ident.clone(), quote!(#ident)),
                    None => {
                        let index = Index::from(field_index);
                        (format_ident!("value{}", field_index), quote!(#index))
                    }
                };

                let r#type = match field_data
                    .accessor_return
                    .as_ref()
                    .or(self.accessors_return.as_ref())
                {
                    Some(FieldDataAccessorReturnModifier::Automatic) | None
                        if is_copy(&field.ty) =>
                    {
                        field.ty.clone()
                    }
                    Some(FieldDataAccessorReturnModifier::Automatic) | None => {
                        Type::Reference(TypeReference {
                            and_token: And::default(),
                            mutability: None,
                            lifetime: None,
                            elem: Box::new(field.ty.clone()),
                        })
                    }
                    Some(FieldDataAccessorReturnModifier::Reference) => {
                        Type::Reference(TypeReference {
                            and_token: And::default(),
                            mutability: None,
                            lifetime: None,
                            elem: Box::new(field.ty.clone()),
                        })
                    }
                    Some(FieldDataAccessorReturnModifier::Copy) => field.ty.clone(),
                };

                let reference_token = match &r#type {
                    Type::Reference(_reference) => quote!(&),
                    _ => quote!(),
                };

                Some(quote!(
                    #visibility fn #method_ident(&self) -> #r#type {
                        #reference_token self.#field_token
                    }
                ))
            });

        quote!(
            #(#methods)*
        )
    }

    fn write_mutators(&self) -> TokenStream {
        let visibility = &self.ast.vis;

        let methods = self
            .ast
            .fields
            .iter()
            .enumerate()
            .flat_map(|(field_index, field)| {
                let field_data = self.fields.get(field_index).expect("existing field");

                if matches!(field_data.mutator, Some(FieldDataModifier::Skipped)) {
                    return None;
                }

                if !self.mutators && !matches!(field_data.mutator, Some(FieldDataModifier::Enabled))
                {
                    return None;
                }

                let (method_ident, field_token) = match &field.ident {
                    Some(ident) => (format_ident!("{}_mut", ident), quote!(#ident)),
                    None => {
                        let index = Index::from(field_index);
                        (format_ident!("value{}_mut", field_index), quote!(#index))
                    }
                };

                let return_type = Type::Reference(TypeReference {
                    and_token: And::default(),
                    mutability: Some(Mut::default()),
                    lifetime: None,
                    elem: Box::new(field.ty.clone()),
                });

                let r#impl = quote!(
                    &mut self.#field_token
                );

                Some(quote!(
                    #visibility fn #method_ident(&mut self) -> #return_type {
                        #r#impl
                    }
                ))
            });

        quote!(
            #(#methods)*
        )
    }

    fn write_setters(&self) -> TokenStream {
        let visibility = &self.ast.vis;

        let methods = self
            .ast
            .fields
            .iter()
            .enumerate()
            .flat_map(|(field_index, field)| {
                let field_data = self.fields.get(field_index).expect("existing field");

                if matches!(field_data.setter, Some(FieldDataModifier::Skipped)) {
                    return None;
                }

                if !self.setters && !matches!(field_data.setter, Some(FieldDataModifier::Enabled)) {
                    return None;
                }

                let (method_ident, field_token, argument_ident) = match &field.ident {
                    Some(ident) => (
                        format_ident!("set_{}", ident),
                        quote!(#ident),
                        ident.clone(),
                    ),
                    None => {
                        let index = Index::from(field_index);
                        (
                            format_ident!("set_value{}", field_index),
                            quote!(#index),
                            format_ident!("value{}", field_index),
                        )
                    }
                };

                let argument_type = &field.ty;

                let r#impl = quote!(
                    self.#field_token = #argument_ident;
                );

                Some(quote!(
                    #visibility fn #method_ident(&mut self, #argument_ident: #argument_type) {
                        #r#impl
                    }
                ))
            });

        quote!(
            #(#methods)*
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
