use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
        q: usize,
        bc: [(usize, usize); q],
    }

    let mut counter = vec![0; 100001];
    for num in a {
        counter[num] += 1;
    }

    for (b, c) in bc {
        let mut sum = 0;
        let diff = counter[b];
        counter[c] += diff;
        counter[b] = 0;

        for c in 0..counter.len() {
            sum += c * counter[c];
        }
        println!("{}", sum);
    }
}