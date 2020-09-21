use std::collections::HashMap;

const STAIR_MAX: i32 = 100;
const STEP_MAX: i32 = 5;

fn main() {
    let mut memo: HashMap<(i32, i32), usize> = HashMap::new();
    println!("{}", stair(0, STAIR_MAX, &mut memo));
}

fn stair(p: i32, q: i32, memo: &mut HashMap<(i32, i32), usize>) -> usize {
    if let Some(&v) = memo.get(&(p, q)) {
        return v;
    }
    if p > q || (p - q).abs() == 1 {
        memo.insert((p, q), 0);
        return 0;
    }
    if p == q {
        memo.insert((p, q), 1);
        return 1;
    }

    let mut cnt: usize = 0;
    for i in 1..=STEP_MAX {
        for j in 1..=STEP_MAX {
            // println!("{},{}", p, q);
            cnt += stair(p + i, q - j, memo);
        }
    }
    memo.insert((p, q), cnt);
    cnt
}
