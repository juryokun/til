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

fn run_option() {
    fn ret_option(v: bool) -> Option<usize> {
        if v {
            Some(100)
        } else {
            None
        }
    }
}

fn run_result() {
    let result1: Result<i32, String> = Ok(200);

    match result1 {
        Ok(code) => println!("code: {}", code),
        Err(err) => println!("Err: {}", err),
    };

    let result2: Result<i32, String> = Ok(200);

    if let Ok(code) = result2 {
        println!("code: {}", code);
    }

    let result3: Result<i32, String> = Ok(200);
    println!("code: {}", result3.unwrap_or(-1)); //code: 200
    let result4: Result<i32, String> = Err("Error".to_string());
    println!("code: {}", result4.unwrap_or(-1)); //code: -1

    fn func(code: i32) -> Result<i32, String> {
        println!("code: {}", code);
        Ok(100)
    }
    let result5: Result<i32, String> = Ok(200);
    let next_result = result5.and_then(func); //funcが実行される
    let result6: Result<i32, String> = Err("error".to_string());
    let next_result = result6.and_then(func); //funcが実行されない
}

fn run_vec() {
    let v1 = vec![1, 2, 3, 4, 5];
    let v2 = vec![0; 5];
    assert_eq!(v1[0], 1);

    for element in &v2 {
        println!("{}", element);
    }
}

fn run_box() {
    let byte_array = [b'h', b'e', b'l', b'l', b'o'];
    print(Box::new(byte_array));

    fn print(s: Box<[u8]>) {
        println!("{:?}", s);
    }
}

mod tests {
    use super::*;
    #[test]
    fn test_run() {
        run_box();
    }
}
