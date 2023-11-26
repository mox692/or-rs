# Rust `if`, `match` extension library
or-rs is a library that extends the syntax to allow different types to be returned in each branch of Rust's `if`, `match` via `or_gen!` macro.


## Usage
Currently, the `or_gen!` macro supports Rust's `if` and `match` statement.

### if expression
```rust
#![feature(proc_macro_hygiene)] // for now, you have to add this unstable feature flag

use macros::or_gen;
use or::enums::*;

fn main() {
    // you have to add a type annotation **explicitly** for `x`.
    #[or_gen]
    let x: Or3<i32, String, f32> = if true {
        3
    } else if false {
        "hello".to_string()
    } else {
        3.0
    };

    // Check if `x` is of type t2(=String)
    if x.is_t2() {
        // here `x` is String
    } else {
        // here `x` is not String
    }
}
```

### match expression
```rust
#![feature(proc_macro_hygiene)] // for now, you have to add this unstable feature flag

use macros::or_gen;
use or::enums::*;

fn main() {
    // you have to add a type annotation **explicitly** for `x`.
    #[or_gen]
    let x: Or3<i32, f32, String> = match 42 {
        1  => 22,
        10 => 3.2,
        _  => "hello".to_string(),
    };

    // map works **only** when the inner value of Or is i32.
    let y = x.map_t1(|a| a * a);
    assert_eq!(y.clone().as_t1().unwrap(), 9);

    // map works **only** when the inner value of Or is f32.
    let z = y.map_t2(|a| a.floor());
    assert_eq!(z.as_t1().unwrap(), 9);
}
```

## Background
In Rust, each branch of an `if`, `match` expression must return a unified type. This means that the following code will fail to compile:

```rust
let s = if true {
    3
} else if false {
    "tofs".to_string()
} else {
    11;
    3.0 // ERROR! `if` and `else` have incompatible types.
};
```

However, with this library's `or_gen!` macro and `Or` type, you can compile code like the following!  

```rust
use macros::or_gen;
use or::enums::*;

#[or_gen]
let s: Or3<i32, String, f32> = if true {
    3
} else if false {
    "tofs".to_string()
} else {
    11;
    3.0
};
// -> OK: This compiles!
```

Under the hood, the `or_gen!` macro converts the above if expression into code like this at compile time:  


```rust
let s: Or3<i32, String, f32> = if true {
    Or3::<i32, String, f32>::T1(3)
} else if false {
    Or3::<i32, String, f32>::T2("tofs".to_string())
} else {
    {
        11;
        Or3::<i32, String, f32>::T3(3.0)
    }
};
```

As each branch of the if returns an `Or3` enum type, so the above code compiles successfully.
Currently, `Or` types from `Or2` to `Or9` are provided.  

### Limitation
Currently, this library has some limitations.

* The number of type arguments in `Or` type and the number of branches in if(match) must match.
* You cannot use `return` within each branch of if(match).
* The `or_gen!` macro can only be used with `let` statements.
  * For example, you cannot use the macro for a single if(match) expression to return the `Or` type as a function's return value.
