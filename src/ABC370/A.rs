// https://atcoder.jp/contests/abc370/tasks/abc370_a
use proconio::input;

fn main() {
    input! {
        l: u8,
        r: u8,
    }
    if l == 1 && r == 0 {
        println!("Yes");
    } else if l == 0 && r == 1 {
        println!("No");
    } else {
        println!("Invalid");
    }
}
