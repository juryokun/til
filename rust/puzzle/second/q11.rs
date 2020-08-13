fn main() {
    let mut memo: Vec<usize> = vec![];
    let mut cnt: u8 = 0;
    for i in 0..100 {
        let rel = fib(i, &mut memo);
        if check(rel) {
            cnt += 1;
            println!("{}", rel);
        }

        if cnt > 12 {
            break;
        }
    }
}

fn fib(n: usize, memo: &mut Vec<usize>) -> usize {
    if memo.get(n) == None {
        if n == 0 || n == 1 {
            memo.push(1);
        } else {
            let rel: usize = fib(n - 2, memo) + fib(n - 1, memo);
            memo.push(rel);
        }
    }
    return memo[n];
}

fn check(n: usize) -> bool {
    let st: &str = &n.to_string();
    let mut target: usize = 0;
    for i in st.chars() {
        target += i as usize - 48;
    }
    if n % target == 0 {
        true
    } else {
        false
    }
}
