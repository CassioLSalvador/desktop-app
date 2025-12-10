use iced::widget::{button, text};
use iced::Element;

#[derive(Default)]
struct AppState {
    value: u64,
}

#[derive(Debug, Clone)]
enum Message {
    Increment,
}

fn update(counter: &mut AppState, message: Message) {
    match message {
        Message::Increment => counter.value += 1,
    }
}

fn view(counter: &AppState) -> Element<'_, Message> {
    button(text(counter.value)).on_press(Message::Increment).into()
}

pub fn main() -> iced::Result {
    iced::run(update, view)
}