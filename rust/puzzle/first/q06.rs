fn main() {
    let mut cnt = 0;
    for i in 0..1001 {
        cnt += colatz(i, i * 3 + 1);
    }
    println!("{}", cnt);
}

fn colatz(target: usize, mut n: usize) -> u8 {
    n = if n % 2 == 0 { n / 2 } else { n * 3 + 1 };
    if n == 1 {
        return 0;
    } else if n == target {
        return 1;
    }

    colatz(target, n)
}
