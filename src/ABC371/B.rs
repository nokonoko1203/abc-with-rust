// https://atcoder.jp/contests/abc371/tasks/abc371_b
use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        n: u8,
        m: u8,
        ab: [(u8, char); m]
    }
    let mut map: HashMap<u8, bool> = HashMap::default();
    for (a, b) in ab {
        if b == 'F' {
            println!("No");
            continue;
        }
        if let std::collections::hash_map::Entry::Vacant(e) = map.entry(a) {
            e.insert(false);
            println!("Yes");
        } else {
            println!("No");
        }
    }
}
