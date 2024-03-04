use quote::quote;
use syn::{parse_macro_input, DeriveInput};
use syn::parse::ParseStream;
use crate::window_info::WindowInfoNestedAttributes;

pub fn yas_window_info(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let struct_name = &input.ident;

    if let syn::Data::Struct(data_struct) = &input.data {
        let mut fields = Vec::new();
        for field in data_struct.fields.iter() {
            let name = field.ident.as_ref().unwrap();

            let mut window_info_key: String = name.to_string();
            for attr in field.attrs.iter() {
                if attr.path().is_ident("window_info") {
                    let nested_attributes = WindowInfoNestedAttributes::from_attr(attr).unwrap();
                    if nested_attributes.rename.is_some() {
                        window_info_key = nested_attributes.rename.clone().unwrap().value();
                    }
                }
            }

            fields.push(quote! {
                #name: match repo.get_auto_scale(#window_info_key, window_size) {
                    None => {
                        return Err(anyhow::anyhow!("cannot find window info key \"{}\"", #window_info_key));
                    },
                    Some(value) => value
                }
            });
        }

        let trait_impl = quote! {
            impl yas::window_info::FromWindowInfoRepository for #struct_name {
                fn from_window_info_repository(window_size: yas::common::positioning::Size<usize>, repo: &yas::window_info::WindowInfoRepository) -> anyhow::Result<Self> {
                    Ok(Self {
                        #(#fields),*
                    })
                }
            }
        };

        return trait_impl.into();
    }

    proc_macro::TokenStream::new()
}