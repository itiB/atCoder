use proconio::input;

fn main() {
    input! {
        n: usize,
        mut st: [(String, i32); n],
    }

    st.sort_by_key(|a| -a.1);
    println!("{}", st[1].0);
}