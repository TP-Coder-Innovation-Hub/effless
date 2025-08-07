use arboard::Clipboard;
use base64::{Engine as _, engine::general_purpose};
use iced::{
    Element, Length,
    widget::{button, column, container, row, scrollable, text, text_input, Column},
};

#[derive(Debug, Clone)]
pub enum Message {
    InputChanged(String),
    Encode,
    Decode,
    Clear,
    CopyToClipboard,
}

#[derive(Default)]
pub struct Base64Tool {
    input: String,
    output: String,
    error: Option<String>,
}

impl Base64Tool {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::InputChanged(value) => {
                self.input = value;
                self.error = None;
            }
            Message::Encode => {
                self.output = general_purpose::STANDARD.encode(&self.input);
                self.error = None;
            }
            Message::Decode => match general_purpose::STANDARD.decode(&self.input) {
                Ok(decoded) => match String::from_utf8(decoded) {
                    Ok(text) => {
                        self.output = text;
                        self.error = None;
                    }
                    Err(_) => {
                        self.error = Some("Invalid UTF-8 in decoded data".to_string());
                    }
                },
                Err(_) => {
                    self.error = Some("Invalid Base64 input".to_string());
                }
            },
            Message::Clear => {
                self.input.clear();
                self.output.clear();
                self.error = None;
            }
            Message::CopyToClipboard => {
                if !self.output.is_empty() {
                    if let Ok(mut clipboard) = Clipboard::new() {
                        let _ = clipboard.set_text(&self.output);
                    }
                }
            }
        }
    }

    pub fn view(&self) -> Element<Message> {
        let input_section = column![
            text("Input").size(16),
            text_input("Enter text to encode/decode...", &self.input)
                .on_input(Message::InputChanged)
                .size(14)
                .padding(10),
        ]
        .spacing(5);

        let buttons = row![
            button(text("Encode").size(14))
                .on_press(Message::Encode)
                .padding(10),
            button(text("Decode").size(14))
                .on_press(Message::Decode)
                .padding(10),
            button(text("Clear").size(14))
                .on_press(Message::Clear)
                .padding(10),
        ]
        .spacing(10);

        let output_section = if !self.output.is_empty() {
            column![
                row![
                    text("Output").size(16),
                    button(text("📋 Copy").size(12))
                        .on_press(Message::CopyToClipboard)
                        .padding([5, 10]),
                ]
                .spacing(10)
                .align_y(iced::Alignment::Center),
                container(
                    scrollable(text_input("", &self.output).size(14)).height(Length::Fixed(100.0))
                )
                .style(container::rounded_box)
                .padding(10)
                .width(Length::Fill),
            ]
            .spacing(5)
        } else {
            column![
                text("Output").size(16),
                container(text("Result will appear here...").size(14).style(
|_theme| iced::widget::text::Style {
                        color: Some(iced::Color::from_rgb(0.6, 0.6, 0.6))
                    }
                ))
                .style(container::rounded_box)
                .padding(10)
                .width(Length::Fill)
                .height(Length::Fixed(100.0)),
            ]
            .spacing(5)
        };

        let mut content = Column::new()
            .spacing(20)
            .push(text("Base64 Encoder/Decoder").size(24))
            .push(input_section)
            .push(buttons)
            .push(output_section);

        if let Some(error) = &self.error {
            content = content.push(text(error).size(14).style(
|_theme| iced::widget::text::Style {
                    color: Some(iced::Color::from_rgb(0.8, 0.2, 0.2))
                },
            ));
        }

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(20)
            .into()
    }
}
