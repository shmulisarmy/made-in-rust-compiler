use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn, Type, FnArg};

#[proc_macro]
pub fn generate_function_header(input: TokenStream) -> TokenStream {
    let input_fn = parse_macro_input!(input as ItemFn);
    let inputs = input_fn.sig.inputs;
    let output = match &input_fn.sig.output {
        syn::ReturnType::Type(_, ty) => ty,
        syn::ReturnType::Default => {
            // If no return type, use unit `()`
            Box::new(syn::parse_quote!(()))
        }
    };

    let input_types: Vec<Type> = inputs
        .into_iter()
        .map(|arg| match arg {
            FnArg::Typed(pat_type) => *pat_type.ty,
            FnArg::Receiver(_) => panic!("Method receivers not supported."),
        })
        .collect();

    let struct_tokens = quote! {
        struct FunctionHeader {
            input: (#(#input_types),*),
            output: #output,
        }
    };

    struct_tokens.into()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(4, 4);
    }
}
