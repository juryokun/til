#![windows_subsystem = "windows"]
use iced::{
    button, executor, scrollable, Align, Application, Button, Column, Command, Container, Element,
    Font, HorizontalAlignment, Length, PaneGrid, Row, Scrollable, Settings, Subscription, Text,
};
use regex::Regex;
use serde::Deserialize;
use serde::Serialize;
use std::fs::File;

const FILE_NAME: &str = "serviceList.csv";

fn load_data() -> Vec<Record> {
    let file = DataFile::new();
    let mut rdr = csv::Reader::from_reader(file.file_open().unwrap());

    let mut rel: Vec<Record> = vec![];
    for result in rdr.deserialize() {
        let record: Record = result.unwrap();
        rel.push(record);
    }
    rel
}

fn main() {
    let mut settings = Settings::default();
    // settings.window.size = (400u32, 120u32);
    GUI::run(settings);
}
#[derive(Clone, Debug)]
pub enum Message {
    Sart,
    End,
}
struct GUI {
    mode: button::State,
    scroll: scrollable::State,
}

impl Application for GUI {
    type Executor = executor::Null;
    type Message = Message;
    type Flags = ();
    fn new(_flags: ()) -> (GUI, Command<Self::Message>) {
        (
            GUI {
                mode: button::State::new(),
                scroll: scrollable::State::new(),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("PWM")
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        Command::none()
    }

    fn view(&mut self) -> Element<Self::Message> {
        let mut context = Scrollable::new(&mut self.scroll);

        let data = load_data();
        for rec in data.iter() {
            let row = Row::with_children(vec![
                Column::new()
                    .width(Length::FillPortion(1))
                    .push(Text::new(&rec.service))
                    .into(),
                Column::new()
                    .width(Length::FillPortion(1))
                    .push(Text::new(&rec.id))
                    .into(),
                Column::new()
                    .width(Length::FillPortion(1))
                    .push(Text::new(&rec.mail))
                    .into(),
                Column::new()
                    .width(Length::FillPortion(1))
                    .push(Text::new(&rec.password))
                    .into(),
                Column::new()
                    .width(Length::FillPortion(1))
                    .push(Text::new(&rec.memo))
                    .into(),
            ]);
            context = context.push(row);
        }
        Container::new(context).into()
    }
}

struct DataFile {
    name: String,
    home_path: String,
    file_path: String,
}
impl DataFile {
    fn new() -> Self {
        let name = FILE_NAME.to_string();
        let home_path = Self::get_home_path();
        let file_path = format!("{}/{}", home_path, name);
        Self {
            name: name,
            home_path: home_path,
            file_path: file_path,
        }
    }
    #[cfg(any(unix))]
    fn get_home_path() -> String {
        let home = std::env::var("HOME");
        home.unwrap()
    }
    #[cfg(target_os = "windows")]
    fn get_home_path() -> String {
        let userprofile = std::env::var("USERPROFILE");
        userprofile.unwrap()
    }
    fn file_open(&self) -> Result<File, std::io::Error> {
        File::open(&self.file_path)
    }
}
#[derive(Debug, Deserialize, Serialize)]
struct Record {
    service: String,
    id: String,
    mail: String,
    password: String,
    memo: String,
}
