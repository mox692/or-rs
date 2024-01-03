//! Macros for use with [Or](../or/trait.Or.html) type.

// when compile with nightly, you need `proc_macro_span`
#![cfg_attr(feature = "macro_error_debugging", feature(proc_macro_span))]

#[allow(unused_extern_crates)]
extern crate proc_macro;

mod error;
mod parser;
use proc_macro::TokenStream;

/// A proc macro that converts `if` or `match` expressions that return multiple types
/// into [Or](../or/trait.Or.html) types.
///
/// Note that this feature depends on the unstable feature `proc_macro_hygiene`,
/// so you must add `#![feature(proc_macro_hygiene)]` annotation when you use it.
///
///
/// ## Example: usage for if expression
///
/// ```rust
/// #![feature(proc_macro_hygiene)]
///
/// use or_rs_macros::or_gen;
/// use or_rs::enums::Or3;
///
/// #[or_gen]
/// // add a type annotation explicitly
/// let s: Or3<i32, String, f32> = if true {
///     3
/// } else if false {
///     "hello".to_string()
/// } else {
///     3.0
/// };
/// ```
///
/// ## Example: usage for match expression
///
/// ```rust
/// #![feature(proc_macro_hygiene)]
///
/// use or_rs_macros::or_gen;
/// use or_rs::enums::Or3;
///
/// #[or_gen]
/// // add a type annotation explicitly
/// let s: Or3<i32, f32, String> = match 42 {
///     1  => 22,
///     10 => 3.2,
///     _  => "hello".to_string(),
/// };
/// ```
///
///
#[proc_macro_attribute]
pub fn or_gen(_attr: TokenStream, item: TokenStream) -> TokenStream {
    parser::MacroParser::parse(item)
}

#[proc_macro]
pub fn my_first_proc_macro(item: TokenStream) -> TokenStream {
    item
}

use proc_macro::TokenStream;
use quote::quote;
use syn::*;

#[proc_macro_attribute]
pub fn add_print(_attr: TokenStream, item: TokenStream) -> TokenStream {
    // 入力をItemFn(関数を表現する構文木データ型に変換)
    let input: ItemFn = parse_macro_input!(item as ItemFn);
    // 関数名を取得
    let name = &input.sig.ident;
    // 関数のブロックを取得
    let block = &input.block;

    // quoteマクロでproc_macro2::TokenStreamを生成
    let expanded: proc_macro2::TokenStream = quote! {
        fn #name() {
            println!("Function {} is called", stringify!(#name));
            #block
        }
    };

    // proc_macro2::TokenStreamからTokenStreamに変換
    TokenStream::from(expanded)
}

use proc_macro::{TokenStream, TokenTree};

#[proc_macro_attribute]
pub fn log(attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut iter = item.into_iter();

    if let Some(TokenTree::Ident(ident)) = iter.next() {
        let func_name = ident.to_string();
        let rest_of_stream: TokenStream = iter.collect();

        let new_stream = format!(
            "fn {}() {{ println!(\"Function '{}' called\"); {} }}",
            func_name, func_name, rest_of_stream
        );
        new_stream.parse().unwrap()
    } else {
        // トークンストリームが関数宣言でない場合はエラー
        "compile_error!(\"Expected function declaration\")"
            .parse()
            .unwrap()
    }
}
