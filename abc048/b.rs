use proconio::input;

fn main() {
    input! {
        mut a: i64,
        b: i64,
        x: i64,
    }

    // 終わりからの個数 - 始めからの個数 + 最初を含むか含まないか
    println!("{}", (b / x) - (a / x) + if a % x == 0 { 1 } else { 0 });
}