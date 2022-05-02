use iced::{
    button, executor, Align, Application, Button, Clipboard, Column, Command, Element, Font,
    HorizontalAlignment, Length, Row, Settings, Subscription, Text,
};

struct GUI {
    start_stop_button_state: button::State,
    reset_button_state: button::State,
}

const FONT: Font = Font::External {
    name: "PixelMplus12-Regular",
    bytes: include_bytes!("../rsc/PixelMplus12-Regular.ttf"),
};

impl Application for GUI {
    type Executor = executor::Default;
    type Message = ();
    type Flags = ();

    fn new(_flags: ()) -> (GUI, Command<Self::Message>) {
        (
            GUI {
                start_stop_button_state: button::State::new(),
                reset_button_state: button::State::new(),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("DEMO")
    }

    fn update(
        &mut self,
        _message: Self::Message,
        _clipboard: &mut Clipboard,
    ) -> Command<Self::Message> {
        Command::none()
    }

    fn view(&mut self) -> Element<Self::Message> {
        let tick_text = Text::new("00:00:00.0").font(FONT).size(60);
        let start_stop_button_state = Button::new(
            &mut self.start_stop_button_state,
            Text::new("Start")
                .horizontal_alignment(HorizontalAlignment::Center)
                .font(FONT),
        )
        .min_width(80);

        let reset_button = Button::new(
            &mut self.reset_button_state,
            Text::new("Rest")
                .horizontal_alignment(HorizontalAlignment::Center)
                .font(FONT),
        )
        .min_width(80);

        Column::new()
            .push(tick_text)
            .push(
                Row::new()
                    .push(start_stop_button_state)
                    .push(reset_button)
                    .spacing(10),
            )
            .spacing(10)
            .padding(10)
            .width(Length::Fill)
            .height(Length::Fill)
            .align_items(Align::Center)
            .into()
    }
}

fn main() {
    let mut settings = Settings::default();
    settings.window.size = (400u32, 120u32);
    GUI::run(settings);
}
