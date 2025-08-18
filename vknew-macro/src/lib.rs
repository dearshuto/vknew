use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{Ident, parse_macro_input};

mod accessor;

// 指定した名前の Id 実装を生成するマクロ
// 使用例: vknew_macro::define_id!(DeviceId);
#[proc_macro]
pub fn define_id(input: TokenStream) -> TokenStream {
    let name = parse_macro_input!(input as Ident);

    let expanded = quote! {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub struct #name {
            internal: u64,
        }

        impl Into<u64> for #name {
            fn into(self) -> u64 {
                self.internal
            }
        }

        impl From<u64> for #name {
            fn from(value: u64) -> Self {
                Self {
                    internal: value
                }
            }
        }
    };

    expanded.into()
}

#[proc_macro]
pub fn generate_definition(input: TokenStream) -> TokenStream {
    let name = parse_macro_input!(input as Ident);
    let name_mut = format_ident!("{}Mut", name);

    quote! {
        pub struct #name<T, U>
        where T: ash::vk::Handle
        {
            handle: T,
            _marker: std::marker::PhantomData<U>,
        }

        pub struct #name_mut<'a, T, U>
        where T: ash::vk::Handle
        {
            handle_ref: &'a mut T,
            _marker: std::marker::PhantomData<U>,
        }
    }
    .into()
}

#[proc_macro_attribute]
pub fn generate_copyable_impl(attr: TokenStream, item: TokenStream) -> TokenStream {
    accessor::generate_copyable_impl(attr, item)
}

#[proc_macro_attribute]
pub fn generate_heap_impl(attr: TokenStream, item: TokenStream) -> TokenStream {
    accessor::generate_heap_impl(attr, item)
}
