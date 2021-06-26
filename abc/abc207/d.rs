use proconio::input;

fn main() {
    input! {
        n: usize,
        mut ab: [(i32, i32); n],
        mut cd: [(i32, i32); n],
    }

    let mut ab90  = ab.iter().map(|(a, b)| ( 1 * b, -1 * a)).collect::<Vec<(i32, i32)>>();
    let mut ab180 = ab.iter().map(|(a, b)| (-1 * a, -1 * b)).collect::<Vec<(i32, i32)>>();
    let mut ab270 = ab.iter().map(|(a, b)| (-1 * b,  1 * a)).collect::<Vec<(i32, i32)>>();
    cd.sort();
    ab.sort();
    ab90.sort();
    ab180.sort();
    ab270.sort();

    for i in 0..n {
        ab[i].0 -= cd[i].0;
        ab[i].1 -= cd[i].1;
        ab90[i].0 -= cd[i].0;
        ab90[i].1 -= cd[i].1;
        ab180[i].0 -= cd[i].0;
        ab180[i].1 -= cd[i].1;
        ab270[i].0 -= cd[i].0;
        ab270[i].1 -= cd[i].1;
    }

    println!("{}", if ab.iter().filter(|(a, b)| *a == ab[0].0 && *b == ab[0].1).count() == n ||
        ab90.iter().filter(|(a, b)| *a == ab90[0].0 && *b == ab90[0].1).count() == n ||
        ab180.iter().filter(|(a, b)| *a == ab180[0].0 && *b == ab180[0].1).count() == n ||
        ab270.iter().filter(|(a, b)| *a == ab270[0].0 && *b == ab270[0].1).count() == n {
             "Yes" } else { "No" });
}