use std::collections::HashSet;
fn main() {
    let mut ans = 0;
    for i in 0..1000 {
        let target_num = (i as f64).sqrt().to_string();
        let num_vec: Vec<&str> = target_num.split('.').collect();
        if let Some(num) = num_vec.get(1) {
            if check_duplicate10(num) {
                ans = i;
                break;
            }
        }
    }
    println!("{}", ans);
}

fn check_duplicate10(target: &str) -> bool {
    let mut hash = HashSet::new();
    for (index, val) in target.chars().enumerate() {
        if index <= 9 {
            hash.insert(val);
        }
    }
    if hash.len() == 10 {
        true
    } else {
        false
    }
}
