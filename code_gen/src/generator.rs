//! ## Code generation of `Or` type
//!
//! The `Or` type provided by this library provides a separate enum type for each number of elements in the generics, but its boilerplate is generated automatically by this binary crate.
//!
//! ## Generate code
//!
//! ```bash
//! # Output code to `or/src/enums.rs`
//! cargo run --bin code_gen
//! ```

use std::{fs::File, os::unix::prelude::FileExt};

const GEN_COUNT: usize = 9;
const OUT_PUT_DIR: &'static str = "../or/src/enums.rs";

pub fn gen_code() -> Result<(), String> {
    let data = gen_code_string(GEN_COUNT);
    let file = match File::create(OUT_PUT_DIR) {
        Ok(file) => file,
        Err(e) => {
            return Err(format!("failed to create a file: {}", e));
        }
    };

    match file.write_all_at(data.as_bytes(), 0) {
        Ok(_) => format!("writing to {} has been done", OUT_PUT_DIR),
        Err(e) => format!("failed to create a file: {}", e),
    };
    Ok(())
}

fn gen_code_string(gen_count: usize) -> String {
    let common = format!(
        "
{}
{}
    ",
        gen_module_top_doc_comment(),
        gen_import_stmts(),
    );
    let repeating = (2..=gen_count)
        .into_iter()
        .map(|i| {
            format!(
                "
{}
{}
{}
",
                gen_enum_decl(i),
                gen_impl_block(i),
                gen_impl_block_with_trait_bound(i)
            )
        })
        .collect::<Vec<_>>()
        .join("");

    format!("{} {}", common, repeating)
}

fn gen_module_top_doc_comment() -> String {
    format!(
        "
//! A concrete implementation of the type Or that represents values of multiple types.
//! 
//! Different enum types `OrX` (where X is the number of types the enum can contain) are provided
//! depending on the number of types it can contain.
//! 
//! The implementation of these enums includes several basic function such as `is_tx`
//! for assertion and `as_tx` for cast, and also have some util functions, like `map`, `fold`.\n"
    )
}

fn gen_import_stmts() -> String {
    format!("use std::any::TypeId;")
}

// gen
// ```
// impl<T1, T2, T3> Or3<T1, T2, T3> {
//     ...
// }
// ```
fn gen_impl_block(idx: usize) -> String {
    format! {"

impl <{}> {} <{}> {{
    {}
    {}
    {}
    {}
}}
    ",
        gen_enum_generics(idx),
        gen_enum_name(idx),
        gen_enum_generics(idx),
        gen_method_is_tx(idx),
        gen_method_as_tx(idx),
        gen_method_map_tx(idx),
        gen_method_fold(idx)
    }
}

// gen
// ```
// impl<T1, T2> Or2<T1, T2>
// where
//     T1: 'static,
//     T2: 'static,
// {
// }
// ```
fn gen_impl_block_with_trait_bound(idx: usize) -> String {
    fn gen_impl_block_with_trait_bound_comment(map_idx: usize) -> String {
        format!(
            "
/// Extension to `Or{}` to check if the enum's type matches a arbitrary type.
/// Currently, these functions depend on the rustc intrinsics, and the constraints
/// of the intrinsics require that the type must satisfy `'static'`.",
            map_idx,
        )
    }

    fn gen_trait_bound_params(g_idx: usize, trait_bound_str: String) -> String {
        (1..=g_idx)
            .into_iter()
            .map(|i| format!("T{}: {}", i, trait_bound_str))
            .collect::<Vec<_>>()
            .join(",\n")
    }

    format! {"
{}
impl <{}> {} <{}>
where
    {}
{{   
    {}
}}
    ",
    gen_impl_block_with_trait_bound_comment(idx),
    gen_enum_generics(idx),
    gen_enum_name(idx),
    gen_enum_generics(idx),
    gen_trait_bound_params(idx, "'static".to_string()),
    gen_method_is(idx)
    }
}

// gen
// ```
// pub fn is<T: 'static>(&self) -> bool {
//     match self {
//         Self::T1(_) => TypeId::of::<T>() == TypeId::of::<T1>(),
//         Self::T2(_) => TypeId::of::<T>() == TypeId::of::<T2>(),
//     }
// }
// ```
fn gen_method_is(idx: usize) -> String {
    fn gen_is_match_arm(g_idx: usize) -> String {
        (1..=g_idx)
            .into_iter()
            .map(|i| {
                format!(
                    "Self::T{}(_) => TypeId::of::<T>() == TypeId::of::<T{}>()",
                    i, i
                )
            })
            .collect::<Vec<_>>()
            .join(",\n")
    }

    format!(
        "
pub fn is_type<T: 'static>(&self) -> bool {{
    match self {{
        {}
    }}
}}
        ",
        gen_is_match_arm(idx)
    )
}

// gen
// ```
// pub fn is_t1(&self) -> bool {
// }
// pub fn is_t2(&self) -> bool {
// }
// ...
// ```
fn gen_method_is_tx(idx: usize) -> String {
    fn gen_method_is_tx_comment(map_idx: usize) -> String {
        format!(
            "
/// Returns true if the enum is of type T{}.",
            map_idx,
        )
    }

    let closure = |x: usize| {
        format!(
            "
{}
pub fn is_t{}(&self) -> bool {{
    match self {{
        Self::T{}(_) => true,
        _ => false,
    }}
}}
        ",
            gen_method_is_tx_comment(x),
            x,
            x
        )
    };

    (1..=idx)
        .into_iter()
        .map(|i| closure(i))
        .collect::<Vec<_>>()
        .join("")
}

// gen
// ```
// pub fn fold<T, F1, F2, F3>(self, f1: F1, f2: F2, f3: F3) -> T
// where
//     F1: FnOnce(T1) -> T,
//     F2: FnOnce(T2) -> T,
//     F3: FnOnce(T3) -> T,
// {
// }
// ```
fn gen_method_fold(idx: usize) -> String {
    fn gen_method_fold_comment(g_idx: usize) -> String {
        format!(
            "
/// Consolidates the `Or{}` enum into a single value of type `T`,
/// by applying provided functions.",
            g_idx,
        )
    }

    // gen `self, f1: F1, f2: F2, f3: F3`
    fn gen_fold_args(g_idx: usize) -> String {
        (1..=g_idx)
            .into_iter()
            .map(|i| format!("f{}: F{}", i, i))
            .collect::<Vec<_>>()
            .join(",")
    }

    // gen
    // ```
    // F1: FnOnce(T1) -> T,
    // F2: FnOnce(T2) -> T,
    // F3: FnOnce(T3) -> T,
    // ```
    fn gen_fold_where(g_idx: usize) -> String {
        (1..=g_idx)
            .into_iter()
            .map(|i| format!("F{}: FnOnce(T{}) -> T", i, i))
            .collect::<Vec<_>>()
            .join(",")
    }

    // gen
    // ```
    // Self::T1(t1) => f1(t1),
    // Self::T2(t2) => f2(t2),
    // Self::T3(t3) => f3(t3),
    // ```
    fn gen_fold_match_arms(g_idx: usize) -> String {
        (1..=g_idx)
            .into_iter()
            .map(|i| format!("Self::T{}(t{}) => f{}(t{})", i, i, i, i))
            .collect::<Vec<_>>()
            .join(",")
    }

    // gen
    // ```
    // F1, F2, F3
    // ```
    fn gen_fold_generics_arg(g_idx: usize) -> String {
        (0..=g_idx)
            .into_iter()
            .map(|i| format!("F{}", i))
            .collect::<Vec<_>>()
            .join(",")
    }

    format!(
        "
{}
pub fn fold<T, {}>(self, {}) -> T
where
        {}
{{
    match self {{
        {}
    }}
}}
    ",
        gen_method_fold_comment(idx),
        gen_fold_generics_arg(idx),
        gen_fold_args(idx),
        gen_fold_where(idx),
        gen_fold_match_arms(idx)
    )
}

// gen
// ```
// pub fn as_t1(self) -> Option<T1> {
// }
// pub fn as_t2(self) -> Option<T2> {
// }
// ...
// ```
fn gen_method_as_tx(idx: usize) -> String {
    fn gen_method_as_tx_comment(map_idx: usize) -> String {
        format!(
            "
/// Converts the enum to an Option containing the T{} value, if it is of type T{}.",
            map_idx, map_idx,
        )
    }

    let closure = |x: usize| {
        format!(
            "
{}
pub fn as_t{}(self) -> Option<T{}>{{
    match self {{
        Self::T{}(t{}) => Some(t{}),
        _ => None,
    }}
}}
        ",
            gen_method_as_tx_comment(x),
            x,
            x,
            x,
            x,
            x
        )
    };

    (1..=idx)
        .into_iter()
        .map(|i| closure(i))
        .collect::<Vec<_>>()
        .join("")
}

// gen
// ```
// pub fn map_t1<F, B>(self, f: F) -> Or3<B, T2, T3>
// where
//     F: FnOnce(T1) -> B,
// {
//     ...
// }
// ...
// ```
fn gen_method_map_tx(idx: usize) -> String {
    fn gen_method_map_tx_comment(map_idx: usize) -> String {
        format!(
            "
/// Transforms the T{} value of the enum using a provided function, 
/// maintaining other types as is.",
            map_idx,
        )
    }

    // gen
    // ```
    // Self::T1(t1) => Or3::<B, T2, T3>::T1(f(t1)),
    // Self::T2(t2) => Or3::<B, T2, T3>::T2(t2),
    // Self::T3(t3) => Or3::<B, T2, T3>::T3(t3),
    // ```
    fn gen_map_inner_match_arms(g_idx: usize, map_idx: usize) -> String {
        (1..=g_idx)
            .into_iter()
            .map(|i| {
                let rewrited_str = if i == map_idx {
                    format!("f(t{})", i)
                } else {
                    format!("t{}", i)
                };
                format!(
                    "Self::T{}(t{}) => Or{}::<{}>::T{}({}),",
                    i,
                    i,
                    g_idx,
                    gen_rewrited_generic_type(gen_enum_generics(g_idx), map_idx, "B".to_string()),
                    i,
                    rewrited_str
                )
            })
            .collect::<Vec<_>>()
            .join("")
    }

    let closure = |x: usize| {
        format!(
            "
{}
pub fn map_t{}<F, B>(self, f: F) -> {}<{}>
where
    F: FnOnce(T{}) -> B,
{{
    match self {{
        {}
    }}
}}
",
            gen_method_map_tx_comment(x),
            x,
            gen_enum_name(idx),
            gen_rewrited_generic_type(gen_enum_generics(idx), x, "B".to_string()),
            x,
            gen_map_inner_match_arms(idx, x)
        )
    };

    let res = (1..=idx)
        .into_iter()
        .map(|i| closure(i))
        .collect::<Vec<_>>()
        .join("");

    res
}

// gen
// ```
// pub enum Or3<T1, T2, T3> {
//     T1(T1),
//     T2(T2),
//     T3(T3),
// }
// ```
fn gen_enum_decl(idx: usize) -> String {
    fn gen_enum_decl_comment(g_idx: usize) -> String {
        format!(
            "
/// `Or{}` is an enum representing a value that can be either of {} types, T1 ... T{}.",
            g_idx, g_idx, g_idx
        )
    }

    format!(
        "
{}
pub enum {} <{}> {{
   {} 
}}
    ",
        gen_enum_decl_comment(idx),
        gen_enum_name(idx),
        gen_enum_generics(idx),
        gen_enum_field(idx)
    )
}

// gen `Or3` in `Or3<T1, T2, T3>` with idx = 3
fn gen_enum_name(idx: usize) -> String {
    format!("Or{}", idx)
}

// gen `T1, T2, T3` in Or3<T1, T2, T3> with idx = 3
fn gen_enum_generics(idx: usize) -> String {
    let enum_generics = (1..=idx)
        .into_iter()
        .map(|i| format!("T{}", i))
        .collect::<Vec<_>>()
        .join(",");
    format!("{}", enum_generics)
}

// gen
//
//    T1(T1),
//    T2(T2),
//    T3(T3),
//
// in
//
//    pub enum Or3<T1, T2, T3> {
//      T1(T1),
//      T2(T2),
//      T3(T3),
//    }
fn gen_enum_field(idx: usize) -> String {
    let s = (1..=idx)
        .into_iter()
        .map(|i| format!("T{}(T{}),", i, i))
        .collect::<Vec<_>>()
        .join("\n");

    s
}

// "T1, T2, T3", B, 1 -> T1, B, T3
fn gen_rewrited_generic_type(input_typ: String, g_idx: usize, rewrited_type_str: String) -> String {
    input_typ.replace(format!("T{}", g_idx).as_str(), &rewrited_type_str)
}
