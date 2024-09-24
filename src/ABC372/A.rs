// https://atcoder.jp/contests/abc372/tasks/abc372_a
use proconio::input;

fn main() {
    input! {
        s: String,
    }
    let mut result = vec![];
    for a in s.chars() {
        if a != '.' {
            result.push(a);
        }
    }
    println!("{}", result.iter().collect::<String>());
}
