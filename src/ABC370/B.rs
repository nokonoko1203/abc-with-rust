// https://atcoder.jp/contests/abc370/tasks/abc370_b
use proconio::input;

fn main() {
    input! {
        N: usize,
    }
    let mut A = vec![];
    for n in 1..=N {
        input! { a: [usize; n] }
        A.push(a);
    }
    let mut i = 0;
    for j in 0..N {
        if (i + 1 >= j + 1) {
            i = A[i][j] - 1
        } else {
            i = A[j][i] - 1
        }
    }
    println!("{}", i + 1);
}
