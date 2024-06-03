use syn::{token, Token};

#[derive(Default)]
pub struct WindowInfoNestedAttributes {
    pub rename: Option<syn::LitStr>,
}

impl WindowInfoNestedAttributes {
    pub fn from_attr(attr: &syn::Attribute) -> syn::parse::Result<Self> {
        let mut result: WindowInfoNestedAttributes = Default::default();
        attr.parse_nested_meta( |meta| {
            if meta.path.is_ident("rename") {
                if meta.input.peek(token::Eq) {
                    let _eq: Token![=] = meta.input.parse()?;
                    let expr: syn::LitStr = meta.input.parse()?;
                    result.rename = Some(expr);
                    return Ok(());
                }
                result.rename = None;
                return Ok(());
            }

            Err(meta.error("unrecognized window_info"))
        })?;

        Ok(result)
    }
}
