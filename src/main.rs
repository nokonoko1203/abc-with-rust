use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n],
    }
    let result: Vec<usize> = a[n - k..]
        .iter()
        .chain(a[..n - k].iter())
        .cloned()
        .collect();

    println!(
        "{}",
        result
            .iter()
            .map(|&x| x.to_string())
            .collect::<Vec<_>>()
            .join(" ")
    );
}
