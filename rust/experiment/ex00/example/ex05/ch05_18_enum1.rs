use crate::Task::*;

#[derive(Debug, PartialEq)]
enum Weekday {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
}
#[derive(Debug, PartialEq)]
enum Month {
    January = 1,
    February = 2,
    March = 3,
    December = 12,
}

type UserName = String;

#[derive(Debug)]
enum Task {
    Open,
    AssignedTo(UserName),
    Working {
        assignee: UserName,
        remaining_hours: u16,
    },
    Done,
}

fn say_something(weekday: Weekday) {
    if weekday == Weekday::Friday {
        println!("TGIF!");
    } else {
        println!("まだ{:?}か", weekday);
    }
}

fn main() {
    say_something(Weekday::Monday);
    assert_eq!(Month::March, Month::March);
    let tasks = vec![
        AssignedTo(String::from("junko")),
        Working {
            assignee: String::from("hiro"),
            remaining_hours: 18,
        },
        Done,
    ];
    for (i, task) in tasks.iter().enumerate() {
        match task {
            AssignedTo(assignee) => println!("タスク{}は{}さんにアサインされています", i, assignee),
            Working {
                assignee,
                remaining_hours,
            } => println!(
                "タスク{}は{}さんが作業中です。残り{}時間の見込み",
                i, assignee, remaining_hours
            ),
            _ => println!("タスク{}はその他のステータス（{:?}）です", i, task),
        }
    }
}
