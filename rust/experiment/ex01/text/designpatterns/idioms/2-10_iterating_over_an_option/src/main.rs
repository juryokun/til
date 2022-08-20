#![allow(unused)]
fn main() {
    sample1();
    sample2();
}

fn sample1() {
    let turing = Some("Turing");
    let mut logicians = vec!["Curry", "Kleene", "Markov"];

    logicians.extend(turing);
    println!("{:?}", logicians);
    println!("{:?}", turing);

    // equivalent to
    if let Some(turing_inner) = turing {
        logicians.push(turing_inner);
    }
    println!("{:?}", logicians);
}

fn sample2() {
    let turing = Some("Turing");
    let logicians = vec!["Curry", "Kleene", "Markov"];

    // let turing2 = "Turing2";
    // let turing_iter = std::iter::once(&turing2);

    // for logician in logicians.iter().chain(turing_iter) {
    for logician in logicians.iter().chain(turing.iter()) {
        println!("{} is a logician", logician);
    }
    let mut a = logicians.iter();
    println!("{:?}", a);
    println!("{:?}", a.next());
    println!("{:?}", a.next());
    println!("{:?}", a.next());
    println!("{:?}", a.next());
}

fn sample3() {
    #[derive(Debug)]
    enum TestA {
        AAA,
        BBB,
        CCC,
        DDD,
    }
    let logicians = vec![TestA::AAA, TestA::BBB];
    let turing_iter = std::iter::once(&TestA::CCC);

    // for logician in logicians.iter().chain(turing_iter) {
    for logician in turing_iter.chain(logicians.iter()) {
        println!("{:?} is a logician", logician);
    }
}

fn sample4() {
    let items = vec![1, 2, 3];
    for item in items.iter() {
        println!("{}", item)
    }
}
