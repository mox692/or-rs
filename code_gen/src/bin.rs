use code_gen::gen_code;

fn main() {
    match gen_code() {
        Ok(_) => (),
        Err(e) => println!("{}", e),
    }
}
