use proc_macro::TokenStream;
use ztd_display_macro::Macro;

////////////////////////////////////////////////////////////////////////////////////////////////////

#[proc_macro_derive(Display, attributes(Display))]
pub fn derive_error(stream: TokenStream) -> TokenStream {
    Macro::handle(stream.into()).into()
}
