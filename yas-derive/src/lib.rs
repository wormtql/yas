extern crate proc_macro;
mod window_info;
#[proc_macro_derive(YasWindowInfo, attributes(window_info))]
pub fn yas_window_info(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    window_info::yas_window_info(input)
}
