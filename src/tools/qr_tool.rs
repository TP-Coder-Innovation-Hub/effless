use iced::{
    widget::{button, column, container, text, text_input, Column},
    Element, Length,
};

#[derive(Debug, Clone)]
pub enum Message {
    InputChanged(String),
    Generate,
    Clear,
}

#[derive(Default)]
pub struct QrTool {
    input: String,
    status: String,
}

impl QrTool {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::InputChanged(value) => {
                self.input = value;
                self.status.clear();
            }
            Message::Generate => {
                if !self.input.is_empty() {
                    self.status = format!("QR Code would be generated for: '{}'", self.input);
                } else {
                    self.status = "Please enter text to generate QR code".to_string();
                }
            }
            Message::Clear => {
                self.input.clear();
                self.status.clear();
            }
        }
    }

    pub fn view(&self) -> Element<Message> {
        let input_section = column![
            text("Text to encode").size(16),
            text_input("Enter text for QR code...", &self.input)
                .on_input(Message::InputChanged)
                .size(14)
                .padding(10),
        ]
        .spacing(5);

        let button_section = button(text("Generate QR Code").size(14))
            .on_press(Message::Generate)
            .padding(10);

        let output_section = if !self.status.is_empty() {
            column![
                text("Status").size(16),
                text(&self.status).size(14),
            ]
            .spacing(5)
        } else {
            column![]
        };

        let content = Column::new()
            .spacing(20)
            .push(text("QR Code Generator").size(24))
            .push(text("Note: QR code generation UI is a placeholder").size(12))
            .push(input_section)
            .push(button_section)
            .push(output_section);

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(20)
            .into()
    }
}