const SECRET_NUMBER: i32 = 25;
static mut GLOBAL_COUNTER: i32 = 25;

enum Light {
    Red,
    Yellow,
    Green,
}

fn main() {
    let x = 10;
    let x = 20;
    let x = "String";

    println!("{}", x);
    {
        let x = 30;
        println!("{}", x);
    }
    println!("{}", x);
    println!("{}", SECRET_NUMBER);
    unsafe {
        println!("{}", GLOBAL_COUNTER);
    }

    let value = 10;
    match value {
        1 => println!("One"),
        10 => println!("Ten"),
        100 => println!("One hundred"),
        _ => println!("Something else"),
    }
    let light = Light::Green;
    let action = match light {
        Light::Red => "Stop",
        Light::Yellow => "Proceed with caution",
        Light::Green => "Go",
    };

    println!("{}", action);

    let unknown = Some("Apple");
    let string = match unknown {
        Some(something) => String::from("Hi, ") + something,
        None => String::from("Nothing"),
    };
    println!("{}", string);

    let ten = 10;
    let ten_reference = &ten;
    match ten_reference {
        number => assert_eq!(&10, number),
    }
    match ten_reference {
        &number => assert_eq!(10, number),
    }

    let abc = 42;
    let string = match abc {
        1 | 2 | 3 => "One or Two or Three",
        40..=50 => "From 40 to 50",
        _ => "Something else",
    };

    println!("{}", string);

    // let string = Some("This is a very long string");
    let string = Some("The Rust");
    let message = match string {
        Some(s) if s.len() >= 10 => "Long string",
        Some(s) if s.len() <= 3 => "Short string",
        // Some(_) => "String",
        None => "Nothing",
        _ => "something else",
    };
    println!("{}", message);

    let score = Some(100);
    if let Some(n) = score {
        println!("You got full marks!, {}", n);
    } else {
        println!("You didn't get full marks.")
    }
}
