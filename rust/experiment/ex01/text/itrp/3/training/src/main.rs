mod trait_text;
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

fn control() {
    let number = 1;
    let result = if 0 <= number { number } else { -number };

    let mut count = 0;
    let result = loop {
        count += 1;
        if count == 10 {
            break count;
        }
    };

    let count: i32;
    for count in 0..10 {
        println!("count: {}", count);
    }

    let i: i32 = 1;
    match i {
        1 => println!("1"),
        2 => println!("2"),
        _ => println!("misc"),
    }

    enum Color {
        Red,
        Blue,
        Green,
    }
    let c = Color::Red;
    match c {
        Color::Red => println!("Red"),
        Color::Green => println!("Green"),
        Color::Blue => println!("Blue"),
    }

    let result: Result<i32, String> = Ok(100);
    let result_number = match result {
        Ok(number) => number,
        Err(message) => {
            println!("Error: {}", message);
            -1
        }
    };
}

fn run_itr() {
    let it = Iter {
        current: 0,
        max: 10,
    };
    for num in it {
        println!("{}", num);
    }
}
struct Iter {
    current: usize,
    max: usize,
}

impl Iterator for Iter {
    type Item = usize; //出力する型の紐付け

    fn next(&mut self) -> Option<usize> {
        self.current += 1;
        if self.current - 1 < self.max {
            Some(self.current - 1)
        } else {
            None
        }
    }
}

struct Person {
    name: String,
    age: u32,
}

impl Person {
    fn say_name(&self) -> &Self {
        println!("I am {}.", self.name);
        self
    }

    fn say_age(&self) -> &Self {
        println!("I am {} year(s) old.", self.age);
        self
    }
}
fn run_impl() {
    let p = Person {
        name: String::from("Taro"),
        age: 20,
    };
    p.say_name().say_age();
}

use std::cell::RefCell;
use std::rc::Rc;
fn run_rc() {
    let data: Rc<RefCell<Vec<String>>> = Rc::new(RefCell::new(vec![]));
    for i in 0..10 {
        push_data(Rc::clone(&data), "a".to_string());
        // push_data(data, "a".to_string());
    }
    println!("{:?}", data);
}

fn push_data(data: Rc<RefCell<Vec<String>>>, target: String) {
    data.borrow_mut().push(target);
}

mod tests {
    use super::*;
    #[test]
    fn test_run() {
        run_rc();
    }
}
