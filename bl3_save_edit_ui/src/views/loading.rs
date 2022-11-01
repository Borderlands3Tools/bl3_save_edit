use iced::alignment::{Horizontal, Vertical};
use iced::{Color, Container, Length, Text};

use crate::bl3_ui::Bl3Message;
use crate::resources::fonts::SOURCE_HAN_SANS;

pub fn view<'a>() -> Container<'a, Bl3Message> {
    let loading_text = Text::new(t!("Loading..."))
        .font(SOURCE_HAN_SANS)
        .size(20)
        .color(Color::from_rgb8(220, 220, 220));

    Container::new(loading_text)
        .width(Length::Fill)
        .height(Length::Fill)
        .align_x(Horizontal::Center)
        .align_y(Vertical::Center)
}
