use proconio::input;

fn main() {
    input! {
        n: usize, // 展望台
        m: usize, // 道
        h: [i32; n], // 標高
        ab: [(usize, usize); m],
    }
    let mut res_vec = vec![true; n];

    for (a, b) in ab{
        if h[a - 1] >= h[b - 1] {
            res_vec[b - 1] = false;
        }
        if h[b - 1] >= h[a - 1] {
            res_vec[a - 1] = false;
        }
    }

    println!("{}", res_vec.iter().filter(|x| x == &&true).count());
}