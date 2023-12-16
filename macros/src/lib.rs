//! Macros for use with [Or](../or/trait.Or.html) type.

// when compile with nightly, you need `proc_macro_span`
#![feature(proc_macro_span)]

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
/// use macros::or_gen;
/// use or::enums::Or3;
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
/// use macros::or_gen;
/// use or::enums::Or3;
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
