fn main() {
    let s = get_input();

    let buf: i32 = s.parse().unwrap();
    let result = buf.pow(3);

    println!("{}", result);
}

fn get_input() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim_end().to_owned()
}
