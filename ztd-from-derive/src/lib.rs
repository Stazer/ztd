use proc_macro::TokenStream;
use ztd_from_macro::Macro;

////////////////////////////////////////////////////////////////////////////////////////////////////

#[proc_macro_derive(From, attributes(From))]
pub fn derive_from(stream: TokenStream) -> TokenStream {
    Macro::handle(stream.into()).into()
}
