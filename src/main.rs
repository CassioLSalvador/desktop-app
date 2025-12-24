use std::fmt::format;

use iced::{Fill, Font, Theme, window};
use iced::font::{Family, Weight, Stretch, Style};
use iced::widget::{
  button,
  column,
  container,
  Container,
  row,
  Row,
  text,
};

#[derive(Clone)]
struct BulletPoint {
  title: String,
  description: Option<String>,
}

#[derive(Clone)]
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
  AddToList,
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
      Message::AddToList => {
        self.interests = Some(vec![Interest {
          title: String::from("Title 1"),
          bullet_points: vec![
            BulletPoint { title: String::from("BP title 1"), description: Some(String::from("BP description 1")) },
            BulletPoint { title: String::from("BP title 2"), description: Some(String::from("BP description 2")) }
          ],
          image: None,
          interests: None,
        }]);
      }
    }
  }

  fn view(&self) -> Container<'_, Message> {
    let font_definitions = Font {
      family: Family::Fantasy,
      weight: Weight::Bold,
      stretch: Stretch::Expanded,
      style: Style::Normal,
      // ..Default::default()
    };

    let mut interests_list = String::new();

    match &self.interests {
      Some(vector) => {
        for (index , value) in vector.iter().enumerate() {
          if index == 0 {
            interests_list = format!("{}", value.title);
          } else {
            interests_list = format!("{} {}", &interests_list, value.title);
          }
        }
      }
      None => {
        interests_list = String::from("No interests available, so far...");
      }
    };

    container(
      column![
        row![
          text("Manage your interests")
            .font(font_definitions)
            .size(42)
            .width(Fill)
            .height(75)
            .center(),
        ].width(Fill),
        row![
          container(
            button("Add interest").on_press(Message::AddToList)
          ).align_right(Fill)   
        ].width(Fill).padding(20),
        row![
          text(interests_list)
            .font(font_definitions)
            .size(22)
            .width(Fill)
        ].padding(20),
      ]
    ).width(Fill)
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