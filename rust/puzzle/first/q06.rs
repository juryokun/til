fn main() {
    let mut cnt = 0;
    for i in 0..10001 {
        cnt += colatz(i, i, 1);
    }
    println!("{}", cnt);
}

fn colatz(target: usize, n: usize, loop_num: usize) -> usize {
    if n == 1 && loop_num != 1 {
        return 0;
    }
    if n == target && loop_num != 1 {
        return 1;
    }

    if n % 2 == 0 && loop_num != 1 {
        return colatz(target, n / 2, loop_num + 1);
    } else {
        return colatz(target, n * 3 + 1, loop_num + 1);
    }
}
