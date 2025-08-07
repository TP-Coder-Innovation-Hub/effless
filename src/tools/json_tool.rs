use arboard::Clipboard;
use iced::{
    Element, Length,
    widget::{button, column, container, row, scrollable, text, text_input, Column},
};
use serde_json::Value;

#[derive(Debug, Clone)]
pub enum Message {
    InputChanged(String),
    Format,
    Minify,
    Clear,
    CopyToClipboard,
}

#[derive(Default)]
pub struct JsonTool {
    input: String,
    output: String,
    error: Option<String>,
}

impl JsonTool {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::InputChanged(value) => {
                self.input = value;
                self.error = None;
            }
            Message::Format => match serde_json::from_str::<Value>(&self.input) {
                Ok(parsed) => match serde_json::to_string_pretty(&parsed) {
                    Ok(formatted) => {
                        self.output = formatted;
                        self.error = None;
                    }
                    Err(e) => {
                        self.error = Some(format!("Formatting error: {}", e));
                    }
                },
                Err(e) => {
                    self.error = Some(format!("Invalid JSON: {}", e));
                }
            },
            Message::Minify => match serde_json::from_str::<Value>(&self.input) {
                Ok(parsed) => match serde_json::to_string(&parsed) {
                    Ok(minified) => {
                        self.output = minified;
                        self.error = None;
                    }
                    Err(e) => {
                        self.error = Some(format!("Minification error: {}", e));
                    }
                },
                Err(e) => {
                    self.error = Some(format!("Invalid JSON: {}", e));
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
            text("JSON Input").size(16),
            text_input("Enter JSON to format/minify...", &self.input)
                .on_input(Message::InputChanged)
                .size(14)
                .padding(10),
        ]
        .spacing(5);

        let buttons = row![
            button(text("Format").size(14))
                .on_press(Message::Format)
                .padding(10),
            button(text("Minify").size(14))
                .on_press(Message::Minify)
                .padding(10),
            button(text("Clear").size(14))
                .on_press(Message::Clear)
                .padding(10),
        ]
        .spacing(10);

        let output_section = if !self.output.is_empty() {
            column![
                row![
                    text("Formatted Output").size(16),
                    button(text("📋 Copy").size(12))
                        .on_press(Message::CopyToClipboard)
                        .padding([5, 10]),
                ]
                .spacing(10)
                .align_y(iced::Alignment::Center),
                container(
                    scrollable(text_input("", &self.output).size(14)).height(Length::Fixed(150.0))
                )
                .style(container::rounded_box)
                .padding(10)
                .width(Length::Fill),
            ]
            .spacing(5)
        } else {
            column![
                text("Formatted Output").size(16),
                container(text("Result will appear here...").size(14).style(
|_theme| iced::widget::text::Style {
                        color: Some(iced::Color::from_rgb(0.6, 0.6, 0.6))
                    }
                ))
                .style(container::rounded_box)
                .padding(10)
                .width(Length::Fill)
                .height(Length::Fixed(150.0)),
            ]
            .spacing(5)
        };

        let mut content = Column::new()
            .spacing(20)
            .push(text("JSON Formatter").size(24))
            .push(input_section)
            .push(buttons)
            .push(output_section);

        if let Some(error) = &self.error {
            content = content.push(text(error).size(14).style(
|_theme| iced::widget::text::Style {
                    color: Some(iced::Color::from_rgb(0.8, 0.2, 0.2))
                }
            ));
        }

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(20)
            .into()
    }
}
