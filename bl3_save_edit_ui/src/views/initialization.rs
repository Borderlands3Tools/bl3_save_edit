use iced::alignment::{Horizontal, Vertical};
use iced::{Color, Container, Length, Text};

use crate::bl3_ui::Bl3Message;
use crate::resources::fonts::SOURCE_HAN_SANS;

#[derive(Debug, Clone)]
pub enum InitializationMessage {
    LoadSaves,
}

pub fn view<'a>() -> Container<'a, Bl3Message> {
    let initializing_text = Text::new(t!("view.init"))
        .font(SOURCE_HAN_SANS)
        .size(20)
        .color(Color::from_rgb8(220, 220, 220));

    Container::new(initializing_text)
        .width(Length::Fill)
        .height(Length::Fill)
        .align_x(Horizontal::Center)
        .align_y(Vertical::Center)
}
