use proconio::input;

fn main() {
    input! {
        n: usize,
        l: [i32; n],
    }

    let mut count = 0;

    for i in 0..l.len() {
        for j in i+1..l.len() {
            for k in j+1..l.len() {
                if l[i] < l[k] + l[j] && l[j] < l[i] + l[k] && l[k] < l[i] + l[j] 
                    && l[i] != l[k] && l[j] != l[k] && l[j] != l[i] {
                    count += 1;
                }
            }
        }
    }
    println!("{}", count);
}