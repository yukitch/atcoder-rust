use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
      a: i32,
      b: i32,
      c: i32,
      i: String,
    };

    println!("{} {}", a + b + c, i);
}
