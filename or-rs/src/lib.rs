//! Provides an enum-like data, `Or` type, that can contain elements of N types.
//! The `Or` type provided by this crate is primarily intended to be used with the
//! [or_gen!](../macros/attr.or_gen.html) macro, but can also be used standalone
//! as an extension of the regular Rust's enum.
//!
//! An implementation of enum exists in the [enums](./enums/index.html) module,
//! which is automatically code-generated by [code_gen](../code_gen/index.html) crate.
//! For more information on each `Or` type, please refer to the module documentation.

#![cfg_attr(feature = "unstable_feature", feature(core_intrinsics))]

pub mod enums;
