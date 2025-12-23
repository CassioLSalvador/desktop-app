use iced::Theme;
use iced::font::{Family, Weight, Stretch, Style};
use iced::{Fill, Font};
use iced::window;
use iced::widget::{
  button,
  column,
  Column,
  container,
  text,
};

struct BulletPoint {
  title: String,
  description: Option<String>,
}

struct Interest {
  title: String,
  bullet_points: Vec<BulletPoint>,
  image: Option<String>,
  interests: Option<Vec<Interest>>,
}

#[derive(Default)]
struct State {
  interests: Option<Vec<Interest>>,
  value: i64,
}

#[derive(Debug, Clone, Copy)]
enum Message {
  Increment,
  Decrement,
}

impl State {
  fn update(&mut self, message: Message) {
    match message {
      Message::Increment => {
        self.value += 1;
      }
      Message::Decrement => {
        self.value -= 1;
      }
    }
  }

  fn view(&self) -> Column<'_, Message> {
    let font_definitions = Font {
      family: Family::Fantasy,
      weight: Weight::Bold,
      stretch: Stretch::Expanded,
      style: Style::Normal,
      // ..Default::default()
    };

    column![
      text("Manage your interests")
        .font(font_definitions)
        .size(42)
        .width(Fill)
        .height(150)
        .center(),
      // button("+").on_press(Message::Increment),
      // text(self.value),
      // button("-").on_press(Message::Decrement),
    ]
  }
}

pub fn main() -> iced::Result {
  let theme = Theme::TokyoNightLight;
  let window_settings = window::Settings {
    maximized: true,
    ..Default::default()
  };

  iced::application(State::default, State::update, State::view).theme(theme).window(window_settings).run()
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_counting() {
    let mut state = State::default();

    state.update(Message::Increment);
    state.update(Message::Increment);
    state.update(Message::Decrement);

    assert_eq!(state.value, 1);
  }
}  