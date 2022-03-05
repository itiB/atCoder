use proconio::input;
use std::cmp::min;
use std::collections::HashSet;

fn main() {
    input! {
        a: usize,
        n: usize
    }

    let mut seen: HashSet<usize> = HashSet::new();
    println!("{}", serach(1, a, n, &mut seen));
}

fn serach(x: usize, a: usize, n: usize, seen: &mut HashSet<usize>) -> i32 {
    // println!("{}", x);
    let mut ans: i32 = -1;
    if x * a == n || bb(x) == n {
        ans = 1;
    } else {
        let mut tmp = 1_000_000_000;
        println!("  a. {}", x*a);
        if x * a <= n * 10 { println!("    Yes");}
        println!("  b. {}", bb(x));
        if x > 9 && bb(x) <= n * 10 && x != bb(x) { println!("    Yes");}
        if x * a <= n * 10 && !seen.contains(&(x * a)) {
            seen.insert(x * a);
            let p1 = serach(x * a, a, n, seen);
            if p1 > 0 {
                tmp = min(tmp, p1 + 1)
            }
            seen.remove(&(x * a));
        }
        if x > 9 && x % 10 != 0 && bb(x) <= n * 10 && x != bb(x) && !seen.contains(&(bb(x))) && x*a != bb(x) {
            seen.insert(bb(x));
            let p2 = serach(bb(x), a, n, seen);
            if p2 > 0 {
                tmp = min(tmp, p2 + 1)
            }
            seen.remove(&bb(x));
        }
        if tmp != 1_000_000_000 {
            ans = tmp;
        }
    }
    return ans
}

fn bb(x: usize) -> usize {
    let m = x % 10;
    let l = x.to_string().len() - 1;
    return x/10 + m * 10usize.pow(l as u32);
}
