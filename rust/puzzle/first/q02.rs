use eval::{eval, to_value};

fn main() {
    let result = arithmetic_operation_game();
    println!("{}", result);
}

fn arithmetic_operation_game() -> usize {
    let mut cnt = 0;
    let op = vec!["*", ""];
    let mut number = 1000;
    while number < 10000 {
        let target = number.to_string();
        for i in op.iter() {
            for j in op.iter() {
                for k in op.iter() {
                    let val = format!(
                        "{}{}{}{}{}{}{}",
                        target.chars().nth(0).unwrap(),
                        i,
                        target.chars().nth(1).unwrap(),
                        j,
                        target.chars().nth(2).unwrap(),
                        k,
                        target.chars().nth(3).unwrap()
                    );
                    let result = eval(&val);
                    let comp: usize = target.chars().rev().collect::<String>().parse().unwrap();
                    if result == Ok(to_value(comp)) && val.chars().count() > 4 && comp >= 1000 {
                        println!("{},{},{}", target, result.unwrap(), val);
                        cnt += 1;
                    }
                }
            }
        }
        number += 1;
    }
    cnt
}
