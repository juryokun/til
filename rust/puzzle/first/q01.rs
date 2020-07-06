fn palindrome() -> u64 {
    let mut number = 11;
    loop {
        if is_palindrome(number.to_string())
            && is_palindrome(format!("{:b}", number))
            && is_palindrome(format!("{:o}", number))
        {
            return number;
        }
        number += 1;
    }
}
fn is_palindrome(num_str: String) -> bool {
    let num_str_rev: String = num_str.chars().rev().collect();
    if num_str == num_str_rev {
        return true;
    }
    false
}

fn main() {
    let result = palindrome();
    println!("{}", result);
}

#[test]
fn test_is_palindrome() {
    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    struct data {
        value: String,
        expected: bool,
    }
    let datas = vec![
        data {
            value: 12345654321u64.to_string(),
            expected: true,
        },
        data {
            value: 120.to_string(),
            expected: false,
        },
        data {
            value: format!("{:b}", 21),
            expected: true,
        },
        data {
            value: format!("{:b}", 22),
            expected: false,
        },
        data {
            value: format!("{:o}", 13395),
            expected: true,
        },
        data {
            value: format!("{:o}", 26892),
            expected: false,
        },
    ];
    for target in datas {
        assert_eq!(
            is_palindrome(target.value.clone()),
            target.expected,
            "{:?}",
            target.value
        );
    }
}
