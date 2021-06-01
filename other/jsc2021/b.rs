use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [usize; n],
        b: [usize; m],
    }

    let mut map: Vec<bool> = vec![false; 1001];

    for aa in a {
        map[aa] = !map[aa];
    }
    for bb in b {
        map[bb] = !map[bb];
    }
    for i in 0..=1000 {
        if map[i] == true {
            print!("{} ", i);
        }
    }
}