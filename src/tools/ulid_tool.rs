use arboard::Clipboard;
use iced::{
    Element, Length,
    widget::{button, column, container, row, text, text_input, Column},
};
use ulid::Ulid;

#[derive(Debug, Clone)]
pub enum Message {
    Generate,
    Clear,
    CopyToClipboard,
}

#[derive(Default)]
pub struct UlidTool {
    generated_ulid: String,
    count: u32,
}

impl UlidTool {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::Generate => {
                let ulid = Ulid::new();
                self.generated_ulid = ulid.to_string();
                self.count += 1;
            }
            Message::Clear => {
                self.generated_ulid.clear();
                self.count = 0;
            }
            Message::CopyToClipboard => {
                if !self.generated_ulid.is_empty() {
                    if let Ok(mut clipboard) = Clipboard::new() {
                        let _ = clipboard.set_text(&self.generated_ulid);
                    }
                }
            }
        }
    }

    pub fn view(&self) -> Element<Message> {
        let info_section = column![
            text("ULID Generator").size(24),
            text("Generates Universally Unique Lexicographically Sortable Identifiers").size(14),
            text("ULIDs are timestamp-sortable and URL-safe")
                .size(12)
                .style(|_theme| iced::widget::text::Style {
                    color: Some(iced::Color::from_rgb(0.6, 0.6, 0.6))
                }),
        ]
        .spacing(5);

        let buttons = row![
            button(text("Generate ULID").size(14))
                .on_press(Message::Generate)
                .padding(10),
            button(text("Clear").size(14))
                .on_press(Message::Clear)
                .padding(10),
        ]
        .spacing(10);

        let output_section = if !self.generated_ulid.is_empty() {
            column![
                row![
                    text("Generated ULID").size(16),
                    button(text("📋 Copy").size(12))
                        .on_press(Message::CopyToClipboard)
                        .padding([5, 10]),
                ]
                .spacing(10)
                .align_y(iced::Alignment::Center),
                container(text_input("", &self.generated_ulid).size(14))
                    .style(container::rounded_box)
                    .padding(10)
                    .width(Length::Fill),
                text(format!("Total generated: {}", self.count))
                    .size(12)
                    .style(|_theme| iced::widget::text::Style {
                        color: Some(iced::Color::from_rgb(0.6, 0.6, 0.6))
                    }),
            ]
            .spacing(5)
        } else {
            column![
                text("Generated ULID").size(16),
                container(
                    text("Click 'Generate ULID' to create a new ULID")
                        .size(14)
                        .style(|_theme| iced::widget::text::Style {
                            color: Some(iced::Color::from_rgb(0.6, 0.6, 0.6))
                        })
                )
                .style(container::rounded_box)
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
