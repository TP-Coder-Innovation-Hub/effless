use iced::{
    widget::{button, column, container, row, text, text_input, scrollable, Column},
    Element, Length,
};
use uuid::Uuid;
use arboard::Clipboard;

#[derive(Debug, Clone)]
pub enum Message {
    Generate,
    Clear,
    CopyToClipboard,
}

#[derive(Default)]
pub struct UuidTool {
    generated_uuid: String,
    count: u32,
}

impl UuidTool {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::Generate => {
                let uuid = Uuid::new_v4();
                self.generated_uuid = uuid.to_string();
                self.count += 1;
            }
            Message::Clear => {
                self.generated_uuid.clear();
                self.count = 0;
            }
            Message::CopyToClipboard => {
                if !self.generated_uuid.is_empty() {
                    if let Ok(mut clipboard) = Clipboard::new() {
                        let _ = clipboard.set_text(&self.generated_uuid);
                    }
                }
            }
        }
    }

    pub fn view(&self) -> Element<Message> {
        let info_section = column![
            text("UUID v4 Generator").size(24),
            text("Generates random UUIDs using version 4 (random)").size(14),
        ]
        .spacing(5);

        let buttons = row![
            button(text("Generate UUID").size(14))
                .on_press(Message::Generate)
                .padding(10),
            button(text("Clear").size(14))
                .on_press(Message::Clear)
                .padding(10),
        ]
        .spacing(10);

        let output_section = if !self.generated_uuid.is_empty() {
            column![
                row![
                    text("Generated UUID").size(16),
                    button(text("📋 Copy").size(12))
                        .on_press(Message::CopyToClipboard)
                        .padding([5, 10]),
                ]
                .spacing(10)
                .align_items(iced::Alignment::Center),
                container(
                    text_input("", &self.generated_uuid)
                        .size(14)
                )
                .style(iced::theme::Container::Box)
                .padding(10)
                .width(Length::Fill),
                text(format!("Total generated: {}", self.count))
                    .size(12)
                    .style(iced::theme::Text::Color(iced::Color::from_rgb(0.6, 0.6, 0.6))),
            ]
            .spacing(5)
        } else {
            column![
                text("Generated UUID").size(16),
                container(
                    text("Click 'Generate UUID' to create a new UUID")
                        .size(14)
                        .style(iced::theme::Text::Color(iced::Color::from_rgb(0.6, 0.6, 0.6)))
                )
                .style(iced::theme::Container::Box)
                .padding(10)
                .width(Length::Fill),
            ]
            .spacing(5)
        };

        let content = Column::new()
            .spacing(20)
            .push(info_section)
            .push(buttons)
            .push(output_section);

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(20)
            .into()
    }
}