use proconio::{input, fastout};

#[fastout]
fn main() {
    input!{
        a: i32,
        b: i32,
    };
    match a * b % 2 {
        0 => println!("Even"),
        1 => println!("Odd"),
        _ => ()
    }
}
