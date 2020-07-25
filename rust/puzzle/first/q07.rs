use chrono::prelude::*;
use chrono::Duration;

fn main() {
    let mut date = Utc.ymd(1964, 10, 10);
    let mut cnt = 0;
    while date != Utc.ymd(2020, 7, 24) {
        let date_bin = ymd_to_str(date);
        if date_bin == date_bin.chars().rev().collect::<String>() {
            println!("{}", date);
            cnt += 1;
        }
        date = date + Duration::days(1);
    }
    println!("{}", cnt);
}

fn ymd_to_str(date: Date<Utc>) -> String {
    format!(
        "{:b}",
        date.format("%Y%m%d").to_string().parse::<usize>().unwrap()
    )
}
