use proc_macro::TokenStream;
use ztd_record_macro::Macro;

////////////////////////////////////////////////////////////////////////////////////////////////////

#[proc_macro_derive(Record, attributes(Record))]
pub fn derive_record(stream: TokenStream) -> TokenStream {
    Macro::handle(stream.into()).into()
}
