extern crate proc_macro2;

use proc_macro::TokenStream;
use crate::echoes::EchoDataItem;
use quote::quote;

mod echoes;

fn get_echo_names(data: &[EchoDataItem]) -> Vec<proc_macro2::TokenStream> {
    let mut result = Vec::new();

    for item in data.iter() {
        result.push(item.name.parse().unwrap());
    }

    result
}

fn echo_name_from_chs(data: &[EchoDataItem], echo_names: &[proc_macro2::TokenStream]) -> proc_macro2::TokenStream {
    let chs_names: Vec<_> = data.iter().map(|x| x.name_chs.clone()).collect();

    quote! {
        impl WWEchoName {
            pub fn from_chs(chs: &str) -> Self {
                match chs {
                    #(#chs_names => Self::#echo_names),*
                    _ => panic!("Unknown chs name"),
                }
            }
        }
    }
}

#[proc_macro]
pub fn yas_wuthering_waves_echoes(input: TokenStream) -> TokenStream {
    let ast: syn::LitStr = syn::parse(input).unwrap();

    let filename = ast.value();

    let content = std::fs::read_to_string(filename).unwrap();
    let echo_data: Vec<EchoDataItem> = serde_json::from_str(&content).unwrap();

    let echo_names = get_echo_names(&echo_data);
    println!("{:?}", echo_names);

    let echo_name_enum = quote! {
        pub enum WWEchoName {
            #(#echo_names),*
        }
    };
    let echo_name_from_chs_impl = echo_name_from_chs(&echo_data, &echo_names);

    let result = quote! {
        #echo_name_enum
        #echo_name_from_chs_impl
    };

    result.into()
}
