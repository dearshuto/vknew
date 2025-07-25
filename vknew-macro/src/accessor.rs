use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{
    ItemStruct, Token, Type, parenthesized, parse::Parse, parse_macro_input, punctuated::Punctuated,
};

// (Type0, Type1) のようにタプル表現をパースするための型
// アトリビュートに可変長引数を受け付けるために必要
struct TypePair {
    handle_type: Type,
    _comma: Token![,],
    dst_type: Type,
}

impl Parse for TypePair {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let content;
        parenthesized!(content in input);
        let handle_type = content.parse()?;
        let comma = content.parse()?;
        let dst_type = content.parse()?;
        Ok(TypePair {
            handle_type,
            _comma: comma,
            dst_type,
        })
    }
}

struct MacroArgs {
    pairs: Punctuated<TypePair, Token![,]>,
}

impl Parse for MacroArgs {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let pairs = Punctuated::parse_terminated(input)?;
        Ok(MacroArgs { pairs })
    }
}

pub fn generate_copyable_impl(attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemStruct);
    let struct_name = &input.ident;
    let struct_name_mut = format_ident!("{}Mut", struct_name);

    let args = parse_macro_input!(attr as MacroArgs);

    let expanded = args.pairs.iter().map(|pair| {
        let handle_type = &pair.handle_type;
        let dst_type = &pair.dst_type;
        quote! {
            // 整数値として扱える型をあつかうバージョン
            impl From<#handle_type> for #struct_name<#handle_type, #dst_type>
            {
                fn from(value: #handle_type) -> Self {
                    Self {
                        handle: value,
                        _marker: std::marker::PhantomData
                    }
                }
            }

            impl #struct_name<#handle_type, #dst_type>
            where #dst_type: From<u64> + Copy
            {
                pub fn peek(self) -> #dst_type
                {
                    #dst_type::from(self.handle.as_raw())
                }
            }

            impl<'a> From<&'a mut #handle_type> for #struct_name_mut<'a, #handle_type, #dst_type>
            {
                fn from(value: &'a mut #handle_type) -> Self {
                    Self {
                        handle_ref: value,
                        _marker: std::marker::PhantomData
                    }
                }
            }

            impl<'a> #struct_name_mut<'a, #handle_type, #dst_type>
            where #dst_type: Into<u64> + Copy
            {
                fn with(self, value: #dst_type) -> #struct_name<#handle_type, #dst_type> {
                    *self.handle_ref = #handle_type::from_raw(value.into());
                    #struct_name {
                        handle: *self.handle_ref,
                        _marker: std::marker::PhantomData
                    }
                }
            }
        }
    });

    quote! {
        #(#expanded)*
    }
    .into()
}

// データをヒープに配置するコードを生成します
pub fn generate_heap_impl(attr: TokenStream, item: TokenStream) -> TokenStream {
    // 構造体の名前
    let input = parse_macro_input!(item as ItemStruct);
    let struct_name = &input.ident;
    // Mutable 用の構造体の名前
    // 末尾に Mut をつける命名規則です
    let struct_name_mut = format_ident!("{}Mut", struct_name);

    // アトリビュートに渡された可変長引数
    // ash の型とそれに対応する型のタプルで表現します
    let args = parse_macro_input!(attr as MacroArgs);

    let expanded = args.pairs.iter().map(|pair| {
        // ash::vk::Device のようなハンドル型
        let handle_type = &pair.handle_type;
        // ハンドルにひもづける型
        let dst_type = &pair.dst_type;
        quote! {
            // ハンドルからアクセサ構造体を生成するコード
            // new 関数を impl にするだけだと型解決できない
            // From にしておくとトレイト境界ができるので意図通りに動作する
            impl From<#handle_type> for #struct_name<#handle_type, #dst_type>
            {
                fn from(value: #handle_type) -> Self {
                    Self {
                        handle: value,
                        _marker: std::marker::PhantomData
                    }
                }
            }

            // Immutable にアクセスする実装の生成
            impl #struct_name<#handle_type, #dst_type>
            {
                // 内部表現を意識させないように FnOnce を渡すような設計にしている
                pub fn peek<F>(self, func: F) -> Self
                where F: FnOnce(&#dst_type)
                {
                    let instance = unsafe{ Box::from_raw(self.handle.as_raw() as *mut #dst_type) };
                    func(&instance as &#dst_type);
                    // drop するとインスタンスを破棄してしまうので所有権を放棄する
                    let _ = Box::into_raw(instance);
                    self
                }

                // 処理した結果を返すバージョン
                pub fn peek_return<T, F>(self, func: F) -> T
                where F: FnOnce(&#dst_type) -> T
                {
                    let instance = unsafe{ Box::from_raw(self.handle.as_raw() as *mut #dst_type) };
                    let return_value = func(&instance as &#dst_type);
                    // インスタンスが生き残り続けるように所有権を放棄する実装にしている
                    let _ = Box::into_raw(instance);
                    return_value
                }
            }

            // ミュータブルな参照を受け取ってハンドル自体を書き換えるアクセサを生成するコード
            impl<'a> From<&'a mut #handle_type> for #struct_name_mut<'a, #handle_type, #dst_type>
            {
                fn from(value: &'a mut #handle_type) -> Self {
                    Self {
                        handle_ref: value,
                        _marker: std::marker::PhantomData
                    }
                }
            }

            impl<'a> #struct_name_mut<'a, #handle_type, #dst_type>
            {
                fn allocate(self, value: #dst_type) -> #struct_name<#handle_type, #dst_type> {
                    // 使い終わったら free() を呼ばないとメモリーリークする
                    // メモリ管理は呼び出し元に責務があるとする
                    let data = Box::new(value);
                    *self.handle_ref = #handle_type::from_raw(Box::into_raw(data) as u64);
                    #struct_name {
                        handle: *self.handle_ref,
                        _marker: std::marker::PhantomData
                    }
                }

                fn update<T, F>(self, updater: F) -> T
                where F: FnOnce(&mut #dst_type) -> T
                {
                    let handle = *self.handle_ref;
                    let mut instance = unsafe{ Box::from_raw(handle.as_raw() as *mut #dst_type) };
                    let return_value = updater(&mut instance);
                    let _ = Box::into_raw(instance);
                    return_value
                }

                fn free(self) {
                    unsafe{ drop(Box::from_raw(self.handle_ref.as_raw() as *mut c_void)) }
                }
            }
        }
    });

    quote! {
        #(#expanded)*
    }
    .into()
}
