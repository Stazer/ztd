use proc_macro::TokenStream;
use ztd_inner_macro::Macro;

////////////////////////////////////////////////////////////////////////////////////////////////////

#[proc_macro_derive(Inner, attributes(Inner))]
pub fn derive_inner(stream: TokenStream) -> TokenStream {
    Macro::handle(stream.into()).into()
}
