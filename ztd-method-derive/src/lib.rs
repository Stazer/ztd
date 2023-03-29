use proc_macro::TokenStream;
use ztd_method_macro::Macro;

////////////////////////////////////////////////////////////////////////////////////////////////////

#[proc_macro_derive(Method, attributes(Method))]
pub fn derive_method(stream: TokenStream) -> TokenStream {
    Macro::handle(stream.into()).into()
}
