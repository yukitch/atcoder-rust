use proconio::{input, fastout};
use proconio::marker::Chars;

#[fastout]
fn main() {
    input! {
      a: Chars
    }

    println!("{}", a.iter().fold(0, |a, x| a + x.to_digit(10).unwrap()))
}
