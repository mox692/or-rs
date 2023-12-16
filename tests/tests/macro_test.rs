#![feature(proc_macro_hygiene)]

use macros::or_gen;
use or::enums::*;

#[test]
fn test_compile() {
    use macros::or_gen;
    use or::enums::*;

    fn f1() -> i32 {
        42
    }
    fn f2() -> f32 {
        42.0
    }

    // simple if expr1
    #[or_gen]
    let _: Or3<i32, String, f32> = if true {
        3
    } else if false {
        "tofs".to_string()
    } else {
        11;
        3.0
    };

    // simple if expr2
    #[or_gen]
    let _: Or2<String, f32> = if false {
        3;
        let a = f1();
        3;
        if a == 3 {}
        ();
        3;
        "".to_string()
    } else {
        let f = f2();
        if let Some(x) = Some(3.0) {
            f + x
        } else {
            f
        }
    };

    // simple match expr
    #[or_gen]
    let s: Or3<i32, f32, String> = match a {
        1 => 33,
        3 => 3.2,
        _ => "hello".to_string(),
    };
}
