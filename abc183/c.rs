use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        t: [[usize; n]; n],
    }

    let mut ans = 0;
    for route in permutations(n - 1) {
        let mut past = 0;
        let mut sum = 0;
        for city in &route {
            sum += t[past][city + 1];
            past = city + 1;
        }
        sum += t[past][0];
        if sum == k {
            ans += 1;
        }
    }
    println!("{}", ans);
}


// https://rosettacode.org/wiki/Permutations#Rust
pub fn permutations(size: usize) -> Permutations {
    Permutations {
        idxs: (0..size).collect(),
        swaps: vec![0; size],
        i: 0,
    }
}

pub struct Permutations {
    idxs: Vec<usize>,
    swaps: Vec<usize>,
    i: usize,
}

impl Iterator for Permutations {
    type Item = Vec<usize>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.i > 0 {
            loop {
                if self.i >= self.swaps.len() {
                    return None;
                }
                if self.swaps[self.i] < self.i {
                    break;
                }
                self.swaps[self.i] = 0;
                self.i += 1;
            }
            self.idxs.swap(self.i, (self.i & 1) * self.swaps[self.i]);
            self.swaps[self.i] += 1;
        }
        self.i = 1;
        Some(self.idxs.clone())
    }
}
