#![feature(proc_macro_hygiene)] // for now, you have to add this unstable feature flag

use or_rs::enums::*;
use or_rs_macros::my_first_proc_macro;
use or_rs_macros::or_gen;

fn main() {
    #[or_gen]
    let x: Or3<i32, f32, String> = // you have to add a type annotation **explicitly** for `x`.
    match 42 {
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
// fn main() {
//     #[or_gen]
//     let x: Or3<i32, String, f32> = // you have to add a type annotation **explicitly** for `x`.
//     if true {
//         3
//     } else if false {
//         "hello".to_string()
//     } else {
//         3.0
//     };

//     // Check if `x` is of type t2(=String)
//     if x.is_t2() {
//         // here `x` is String
//     } else {
//         // `is_type` function allows explicitly
//         // specifying the type assertion for `x`.
//         if x.is_type::<i32>() {
//             // here `x` is i32, so we can unwrap the
//             // inner value safely by `as_t1().unwrap()`
//             let a: i32 = x.as_t1().unwrap();
//         }
//     }
// }
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_or_gen() {
        use or_rs::enums::*;
        use or_rs_macros::or_gen;

        #[or_gen]
        let x: Or3<i32, String, f32> = if true {
            3
        } else if false {
            "tofs".to_string()
        } else {
            11;
            3.0
        };

        // map works **only** when the inner value of Or is i32.
        let y = x.map_t1(|a| a * a);
        assert_eq!(y.clone().as_t1().unwrap(), 9);
        // map works **only** when the inner value of Or is f32.
        let z = y.map_t3(|a| a.floor());
        assert_eq!(z.as_t1().unwrap(), 9);
    }

    #[allow(dead_code, unused_variables)]
    #[test]
    fn test_match() {
        let a = 3;

        #[or_gen]
        let s: Or3<i32, f32, String> = match a {
            1 => 33,
            3 => 3.2,
            _ => "hello".to_string(),
        };
    }
}
