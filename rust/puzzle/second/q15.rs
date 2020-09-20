fn main() {
    println!("{}", stair(0, 10));
}

fn stair(a: i32, b: i32) -> usize {
    if a > b || (a - b).abs() == 1 {
        return 0;
    }
    if a == b {
        return 1;
    }

    let mut cnt: usize = 0;
    for i in 1..=4 {
        for j in 1..=4 {
            // println!("{},{}", a, b);
            cnt += stair(a + i, b - j);
        }
    }
    cnt
}
