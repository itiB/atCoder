use proconio::input;

fn main() {
    input! {
        sx: i32,
        sy: i32,
        tx: i32,
        ty: i32,
    }

    let start = [(sx, sy + 1), (tx, ty + 1), (sx, sy - 1), (tx, ty - 1)];
    let goal  = [(tx - 1, ty), (sx - 1, sy), (tx + 1, ty), (sx + 1, sy)];

    for i in 0..4 {
        // 進行方向の+-を制御する、行きは+1, 帰りは-1
        let m = if i == 2 || i == 0 { 1 } else { -1 };

        match i {
            0..=1 => print!("U"),
            2..=3 => print!("D"),
            _ => unreachable!(),
        }


        let y = goal[i].1 - start[i].1;
        let x = goal[i].0 - start[i].0;

        if i == 0 || i == 3 {
            for _ in 0..y.abs() {
                if m > 0 { print!("U"); } else { print!("D"); }
            }
            for _ in 0..x.abs() {
                if m > 0 { print!("R"); } else { print!("L"); }
            }
        } else {
            for _ in 0..x.abs() {
                if m > 0 { print!("R"); } else { print!("L"); }
            }
            for _ in 0..y.abs() {
                if m > 0 { print!("U"); } else { print!("D"); }
            }
        }

        match i {
            0..=1 => print!("R"),
            2..=3 => print!("L"),
            _ => unreachable!(),
        }
    }
    println!("");
}