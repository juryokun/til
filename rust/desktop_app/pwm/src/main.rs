#![windows_subsystem = "windows"]
use iced::{
    button, executor, Align, Application, Button, Column, Command, Container, Element, Font,
    HorizontalAlignment, Length, Row, Settings, Subscription, Text,
};
fn main() {
    let mut settings = Settings::default();
    settings.window.size = (400u32, 120u32);
    GUI::run(settings);
}
#[derive(Clone, Debug)]
pub enum Message {
    Sart,
    End,
}
struct GUI {
    mode: button::State,
}

impl Application for GUI {
    type Executor = executor::Null;
    type Message = Message;
    type Flags = ();
    fn new(_flags: ()) -> (GUI, Command<Self::Message>) {
        (
            GUI {
                mode: button::State::new(),
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
        let mut context = Column::new();

        for i in 0..5 {
            let row = Row::new()
                .push(Text::new("test1"))
                .spacing(20)
                .push(Text::new("test2"));
            context = context.push(row);
        }
        Container::new(context).into()
    }
}
