use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse2, Generics, Ident, Item, ItemEnum, ItemStruct};

////////////////////////////////////////////////////////////////////////////////////////////////////

fn write_error_impl(generics: &Generics, name: &Ident) -> TokenStream {
    let (impl_generics, type_generics, where_clause) = generics.split_for_impl();

    quote!(
        impl #impl_generics ::core::error::Error for #name #type_generics #where_clause {}
    )
}

////////////////////////////////////////////////////////////////////////////////////////////////////

struct EnumData<'a> {
    ast: &'a ItemEnum,
}

impl<'a> EnumData<'a> {
    fn read(ast: &'a ItemEnum) -> Self {
        Self { ast }
    }

    fn write(self) -> TokenStream {
        write_error_impl(&self.ast.generics, &self.ast.ident)
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
        write_error_impl(&self.ast.generics, &self.ast.ident)
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
