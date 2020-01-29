extern crate proc_macro;

use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn method(_args: TokenStream, method: TokenStream) -> TokenStream {
    let method = syn::parse_macro_input!(method as syn::ImplItemMethod);
    let receiver = method.sig.inputs.first().unwrap();
    syn::Error::new_spanned(receiver, "this error should also cover the type")
        .to_compile_error()
        .into()
}
