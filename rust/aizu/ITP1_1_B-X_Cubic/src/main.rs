fn main() {
    let s = get_input();
    let max_cnt: i32 = s.parse().unwrap();

    let mut cnt = 0;
    let mut min: isize = 10000000000;
    let mut max: isize = -100000000000;
    while cnt < max_cnt {
        let t = get_input();
        let num = t.parse().unwrap();

        if (num-min) > max {
            max = num - min;
        }
        if num < min {
            min = num;
        }
        cnt += 1;
    }

    println!("{}", max);
}

fn get_input() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim_end().to_owned()
}