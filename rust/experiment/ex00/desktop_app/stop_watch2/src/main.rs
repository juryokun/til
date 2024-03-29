#![windows_subsystem = "windows"]
use iced::{
    button, executor, Align, Application, Button, Clipboard, Column, Command, Element, Font,
    HorizontalAlignment, Length, Row, Settings, Subscription, Text,
};

use iced_futures::{self, futures};
use std::time::{Duration, Instant};

mod datastore;
use datastore::*;

const FONT: Font = Font::External {
    name: "PixelMplus12-Regular",
    bytes: include_bytes!("../rsc/PixelMplus12-Regular.ttf"),
};

const FPS: u64 = 30;
const MILLISEC: u64 = 1000;
const MINUTE: u64 = 60;
const HOUR: u64 = 60 * MINUTE;

struct GUI {
    last_update: Instant,
    total_duration: Duration,
    tick_state: TickState,
    register_state: RegisterState,
    start_stop_button_state: button::State,
    reset_button_state: button::State,
    send_button_state: button::State,
}

#[derive(Debug, Clone)]
pub enum Message {
    Start,
    Stop,
    Reset,
    Update,
    Register,
    NotRegister,
}

pub enum TickState {
    Stopped,
    Ticking,
}

pub enum RegisterState {
    Active,
    UnActive,
}

impl Application for GUI {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_flags: ()) -> (GUI, Command<Self::Message>) {
        (
            GUI {
                last_update: Instant::now(),
                total_duration: Duration::default(),
                tick_state: TickState::Stopped,
                register_state: RegisterState::UnActive,
                start_stop_button_state: button::State::new(),
                reset_button_state: button::State::new(),
                send_button_state: button::State::new(),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("DEMO")
    }

    fn update(
        &mut self,
        message: Self::Message,
        clipboard: &mut Clipboard,
    ) -> Command<Self::Message> {
        match message {
            Message::Start => {
                self.tick_state = TickState::Ticking;
                self.last_update = Instant::now();
                self.register_state = RegisterState::UnActive;
            }
            Message::Stop => {
                self.tick_state = TickState::Stopped;
                self.total_duration += Instant::now() - self.last_update;
                self.register_state = RegisterState::Active;
            }
            Message::Reset => {
                self.last_update = Instant::now();
                self.total_duration = Duration::default();
                self.register_state = RegisterState::UnActive;
            }
            Message::Update => match self.tick_state {
                TickState::Ticking => {
                    let now_update = Instant::now();
                    self.total_duration += now_update - self.last_update;
                    self.last_update = now_update;
                }
                _ => {}
            },
            Message::Register => {
                if let Ok(connection) = Firestore::new() {
                    let mut service = Service::<Firestore>::new(connection);
                    let record = Record::create(self.total_duration);
                    service.push_record(record);
                }
                self.register_state = RegisterState::UnActive;
            }
            Message::NotRegister => {}
        }
        Command::none()
    }

    fn subscription(&self) -> Subscription<Message> {
        let timer = Timer::new(Duration::from_millis(MILLISEC / FPS));
        Subscription::from_recipe(timer).map(|_| Message::Update)
    }

    fn view(&mut self) -> Element<Self::Message> {
        // prepare duration text
        let seconds = self.total_duration.as_secs();
        let duration_text = format!(
            "{:0>2}:{:0>2}:{:0>2}.{:0>2}",
            seconds / HOUR,
            (seconds % HOUR) / MINUTE,
            seconds % MINUTE,
            self.total_duration.subsec_millis() / 10,
        );

        // prepare start/stop text
        let start_stop_text = match self.tick_state {
            TickState::Stopped => Text::new("Start")
                .horizontal_alignment(HorizontalAlignment::Center)
                .font(FONT),
            TickState::Ticking => Text::new("Stop")
                .horizontal_alignment(HorizontalAlignment::Center)
                .font(FONT),
        };

        // prepare start/stop message on button press
        let start_stop_message = match self.tick_state {
            TickState::Stopped => Message::Start,
            TickState::Ticking => Message::Stop,
        };

        let register_text = match self.register_state {
            RegisterState::Active => Text::new("Register")
                .horizontal_alignment(HorizontalAlignment::Center)
                .font(FONT),
            RegisterState::UnActive => Text::new("-")
                .horizontal_alignment(HorizontalAlignment::Center)
                .font(FONT),
        };
        let register_message = match self.register_state {
            RegisterState::Active => Message::Register,
            RegisterState::UnActive => Message::NotRegister,
        };

        // init widgets
        let tick_text = Text::new(duration_text).font(FONT).size(60);
        let start_stop_button = Button::new(&mut self.start_stop_button_state, start_stop_text)
            .min_width(80)
            .on_press(start_stop_message);

        let send_button = Button::new(&mut self.send_button_state, register_text)
            .min_width(80)
            .on_press(register_message);

        let reset_button = Button::new(
            &mut self.reset_button_state,
            Text::new("Rest")
                .horizontal_alignment(HorizontalAlignment::Center)
                .font(FONT),
        )
        .min_width(80)
        .on_press(Message::Reset);

        Column::new()
            .push(tick_text)
            .push(
                Row::new()
                    .push(start_stop_button)
                    .push(reset_button)
                    .push(send_button)
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

pub struct Timer {
    duration: Duration,
}

impl Timer {
    fn new(duration: Duration) -> Timer {
        Timer { duration: duration }
    }
}

impl<H, E> iced_native::subscription::Recipe<H, E> for Timer
where
    H: std::hash::Hasher,
{
    type Output = Instant;

    fn hash(&self, state: &mut H) {
        use std::hash::Hash;
        std::any::TypeId::of::<Self>().hash(state);
        self.duration.hash(state);
    }

    fn stream(
        self: Box<Self>,
        _input: futures::stream::BoxStream<'static, E>,
    ) -> futures::stream::BoxStream<'static, Self::Output> {
        use futures::stream::StreamExt;
        async_std::stream::interval(self.duration)
            .map(|_| Instant::now())
            .boxed()
    }
}

fn main() {
    let mut settings = Settings::default();
    settings.window.size = (400u32, 120u32);
    GUI::run(settings);
}
