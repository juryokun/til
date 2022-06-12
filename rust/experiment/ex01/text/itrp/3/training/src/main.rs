fn main() {
    unimplemented!();
}

fn run_string() {
    let s1: String = String::from("Hello, World!");
    let s2 = &s1;
    let s3 = s2.to_string(); // &str->String メモリ確保が行われる

    println!("{}", s3);
}

fn run_tupple() {
    let mut t = (1, "2");
    t.0 = 2;
    t.1 = "3";
    println!("{:?}", t);
}

fn run_array() {
    let mut a: [i32; 3] = [0, 1, 2];
    let b: [i32; 3] = [0; 3];

    assert_eq!(a, [0, 1, 2]);
    assert_eq!(b, [0, 0, 0]);

    a[1] = b[1];
    a[2] = b[2];
    assert_eq!(a, [0, 0, 0]);
    assert_eq!(&a, &[0, 0, 0]);

    println!("{:?}", &a[1..3]);
}

fn run_user_definition() {
    #[derive(Debug)]
    struct Person {
        name: String,
        age: u32,
    }

    let p = Person {
        name: String::from("John"),
        age: 8,
    };
    println!("{:?}", p);
}

fn run_enum() {
    #[derive(Debug)]
    enum Event {
        Quit,
        KeyDown(u8),
        MouseDown { x: i32, y: i32 },
    }
    let e1 = Event::Quit;
    let e2 = Event::MouseDown { x: 10, y: 10 };
    println!("{:?}", e1);
    println!("{:?}", e2);
}

mod tests {
    use super::*;
    #[test]
    fn test_run() {
        run_enum();
    }
}
