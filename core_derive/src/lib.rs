use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Task)]
pub fn task(tokens: TokenStream) -> TokenStream {
    let input = parse_macro_input!(tokens as DeriveInput);
    let task_name = input.ident;
    let task_name_str = task_name.to_string();

    let mut c = task_name_str.chars();
    let name = match c.next() {
        None => String::new(),
        Some(f) => f.to_lowercase().collect::<String>() + c.as_str(),
    };

    let out = quote! {
        impl crate::task::task::Named for #task_name {
            fn name() -> ::std::string::String {
                #name.to_owned()
            }
        }
        impl crate::task::task::TaskGenerator for #task_name {
            fn get_factory() -> crate::task::task::TaskFactory {
                crate::task::task::TaskFactory {
                    instantiate: #task_name::new,
                }
            }
        }
    };

    TokenStream::from(out)
}
