use proconio::input;

fn main() {
    input! {
        n: i64,
    }

    for num in 1..n + 1 {
        let mut count = 0;

        'outer: for x in 1..=num {
            for y in x..=num {
                if x * x + 3 * y * y + 2 * x * y > num {
                    break;
                }
                for z in y..=num {
                    if x * x + y * y + z * z + x * y + y * z + z * x == num {
                        count += 6;
                        if x == y || y == z || z == x {
                            count -= 3;
                        }
                        if x == y && x == z {
                            count -= 2;
                        }
                        break 'outer;
                    }
                    if x * x + y * y + z * z + x * y + y * z + z * x > num {
                        break;
                    }
                }
            }
        }
        println!("{}", count);
    }
}