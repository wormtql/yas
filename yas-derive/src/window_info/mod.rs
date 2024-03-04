extern crate proc_macro;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(YasWindowInfo)]
pub fn yas_window_info(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let struct_name = &input.ident;

    let mut fields = Vec::new();

    if let syn::Data::Struct(data_struct) = &input.data {
        for field in data_struct.fields.iter() {
            let name = field.ident.as_ref().unwrap();
            fields.push(quote! {
                #name: match repo.get_auto_scale(&format!("{}", #name), window_size) {
                    None => {
                        return Err(anyhow::anyhow!("cannot find window info key \"{}\"", #name));
                    },
                    Some(value) => value
                }
            });
        }

        let trait_impl = quote! {
            impl yas::window_info::FromWindowInfoRepository for #struct_name {
                fn from_window_info_repository(window_size: yas::common::positioning::Size, repo: &yas::window_info::WindowInfoRepository) -> anyhow::Result<Self> {
                    Ok(Self {
                        #(fields),*
                    })
                }
            }
        };

        return trait_impl.into();
    }

    proc_macro::TokenStream::new()
}