use proc_macro::TokenStream;
use ztd_constructor_macro::Macro;

////////////////////////////////////////////////////////////////////////////////////////////////////

#[proc_macro_derive(Constructor, attributes(Constructor))]
pub fn derive_constructor(stream: TokenStream) -> TokenStream {
    Macro::handle(stream.into()).into()
}
