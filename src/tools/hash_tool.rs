use arboard::Clipboard;
use iced::{
    Element, Length,
    widget::{button, column, container, pick_list, row, text, text_input, Column},
};
use md5;
use sha2::{Digest, Sha256, Sha512};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HashType {
    Md5,
    Sha256,
    Sha512,
}

impl HashType {
    const ALL: [HashType; 3] = [HashType::Md5, HashType::Sha256, HashType::Sha512];
}

impl std::fmt::Display for HashType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HashType::Md5 => write!(f, "MD5"),
            HashType::Sha256 => write!(f, "SHA-256"),
            HashType::Sha512 => write!(f, "SHA-512"),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    InputChanged(String),
    HashTypeSelected(HashType),
    Generate,
    Clear,
    CopyToClipboard,
}

pub struct HashTool {
    input: String,
    output: String,
    hash_type: HashType,
}

impl Default for HashTool {
    fn default() -> Self {
        Self {
            input: String::new(),
            output: String::new(),
            hash_type: HashType::Sha256,
        }
    }
}

impl HashTool {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::InputChanged(value) => {
                self.input = value;
            }
            Message::HashTypeSelected(hash_type) => {
                self.hash_type = hash_type;
            }
            Message::Generate => {
                self.output = self.compute_hash(&self.input);
            }
            Message::Clear => {
                self.input.clear();
                self.output.clear();
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

    fn compute_hash(&self, input: &str) -> String {
        match self.hash_type {
            HashType::Md5 => {
                format!("{:x}", md5::compute(input.as_bytes()))
            }
            HashType::Sha256 => {
                let mut hasher = Sha256::new();
                hasher.update(input.as_bytes());
                format!("{:x}", hasher.finalize())
            }
            HashType::Sha512 => {
                let mut hasher = Sha512::new();
                hasher.update(input.as_bytes());
                format!("{:x}", hasher.finalize())
            }
        }
    }

    pub fn view(&self) -> Element<Message> {
        let input_section = column![
            text("Text to Hash").size(16),
            text_input("Enter text to hash...", &self.input)
                .on_input(Message::InputChanged)
                .size(14)
                .padding(10),
        ]
        .spacing(5);

        let hash_type_picker = column![
            text("Hash Algorithm").size(16),
            pick_list(
                &HashType::ALL[..],
                Some(self.hash_type),
                Message::HashTypeSelected
            )
            .padding(10)
            .text_size(14),
        ]
        .spacing(5);

        let buttons = row![
            button(text("Generate Hash").size(14))
                .on_press(Message::Generate)
                .padding(10),
            button(text("Clear").size(14))
                .on_press(Message::Clear)
                .padding(10),
        ]
        .spacing(10);

        let output_section = if !self.output.is_empty() {
            column![
                row![
                    text("Hash Result").size(16),
                    button(text("📋 Copy").size(12))
                        .on_press(Message::CopyToClipboard)
                        .padding([5, 10]),
                ]
                .spacing(10)
                .align_y(iced::Alignment::Center),
                container(text_input("", &self.output).size(14))
                    .style(container::rounded_box)
                    .padding(10)
                    .width(Length::Fill),
            ]
            .spacing(5)
        } else {
            column![
                text("Hash Result").size(16),
                container(text("Hash will appear here...").size(14).style(
|_theme| iced::widget::text::Style {
                        color: Some(iced::Color::from_rgb(0.6, 0.6, 0.6))
                    }
                ))
                .style(container::rounded_box)
                .padding(10)
                .width(Length::Fill),
            ]
            .spacing(5)
        };

        let content = Column::new()
            .spacing(20)
            .push(text("Hash / Checksum Generator").size(24))
            .push(input_section)
            .push(hash_type_picker)
            .push(buttons)
            .push(output_section);

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(20)
            .into()
    }
}
