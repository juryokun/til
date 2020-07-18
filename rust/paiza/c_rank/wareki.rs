use std::io::Read;

fn main() {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    s.trim_right().to_owned();

    let buf = s.split_whitespace().collect::<Vec<&str>>();

    let target = Date::new(buf[0], buf[1], buf[2]);
    println!("{}年{}月{}日", target.judge(), target.month, target.day);
}

#[derive(Debug, Copy, Clone, PartialEq)]
struct Date {
    year: u16,
    month: u16,
    day: u16,
}

impl Date {
    fn new(y: &str, m: &str, d: &str) -> Self {
        Self {
            year: y.parse::<u16>().unwrap(),
            month: m.parse::<u16>().unwrap(),
            day: d.parse::<u16>().unwrap(),
        }
    }
    fn concat(self) -> usize {
        format!("{:04}{:02}{:02}", self.year, self.month, self.day)
            .parse()
            .unwrap()
    }
    fn judge(self) -> String {
        let rel = match self.concat() {
            0..=19120729 => "明治",
            19120730..=19261224 => "大正",
            19261225..=19890107 => "昭和",
            19890107..=20190430 => "平成",
            _ => "令和",
        };
        rel.to_string()
    }
}

#[test]
fn test_new() {
    let target = Date::new("2019", "4", "30");
    let exp = Date {
        year: 2019,
        month: 4,
        day: 30,
    };
    assert_eq!(target, exp);
}
#[test]
fn test_judge() {
    let target = Date {
        year: 2019,
        month: 4,
        day: 30,
    };
    assert_eq!(target.judge(), "平成");
}
