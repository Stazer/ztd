use proc_macro::TokenStream;
use ztd_error_macro::Macro;

////////////////////////////////////////////////////////////////////////////////////////////////////

#[proc_macro_derive(Error, attributes(Error))]
pub fn derive_from(stream: TokenStream) -> TokenStream {
    Macro::handle(stream.into()).into()
}
