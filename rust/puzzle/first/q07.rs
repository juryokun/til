use chrono::prelude::*;
use chrono::Duration;

fn main() {
    let mut date = Utc.ymd(1964, 10, 10);
    let mut cnt = 0;
    while date != Utc.ymd(2020, 7, 24) {
        let date_str = date.format("%Y%m%d").to_string().parse::<usize>().unwrap();
        let date_2pre = format!("{:b}", date_str);
        let date_2aft = date_2pre.chars().rev().collect::<String>();
        let date_str_aft = usize::from_str_radix(&date_2aft, 2).unwrap();
        if date_str == date_str_aft {
            println!("{}", date_str);
            cnt += 1;
        }
        date = date + Duration::days(1);
    }
    println!("{}", cnt);
}
