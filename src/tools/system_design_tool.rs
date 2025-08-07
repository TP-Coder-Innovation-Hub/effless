use iced::{
    widget::{button, column, container, row, text, text_input, Column},
    Element, Length,
};
use arboard::Clipboard;

const SECOND_IN_MINUTE: u64 = 60;
const MINUTE_IN_HOUR: u64 = 60;
const HOUR_IN_DAY: u64 = 24;
const SECOND_IN_DAY: u64 = SECOND_IN_MINUTE * MINUTE_IN_HOUR * HOUR_IN_DAY; // 86400
const DAY_IN_YEAR: u64 = 365; // Assume 365 days in a year

#[derive(Debug, Clone)]
pub enum Message {
    DailyActiveUserChanged(String),
    ReadWriteRatioChanged(String),
    DataSizeChanged(String),
    Calculate,
    Clear,
    CopyToClipboard,
}

pub struct SystemDesignTool {
    daily_active_user: String,
    read_write_ratio: String,
    data_size: String,
    read_per_second: f64,
    write_per_second: f64,
    storage_used_per_year: u64,
    error: Option<String>,
}

impl Default for SystemDesignTool {
    fn default() -> Self {
        Self {
            daily_active_user: String::new(),
            read_write_ratio: String::new(),
            data_size: String::new(),
            read_per_second: 0.0,
            write_per_second: 0.0,
            storage_used_per_year: 0,
            error: None,
        }
    }
}

impl SystemDesignTool {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::DailyActiveUserChanged(value) => {
                self.daily_active_user = value;
                self.error = None;
            }
            Message::ReadWriteRatioChanged(value) => {
                self.read_write_ratio = value;
                self.error = None;
            }
            Message::DataSizeChanged(value) => {
                self.data_size = value;
                self.error = None;
            }
            Message::Calculate => {
                match self.calculate_back_of_envelope() {
                    Ok((read_per_second, write_per_second, storage_used_per_year)) => {
                        self.read_per_second = read_per_second;
                        self.write_per_second = write_per_second;
                        self.storage_used_per_year = storage_used_per_year as u64;
                        self.error = None;
                    }
                    Err(error) => {
                        self.error = Some(error);
                    }
                }
            }
            Message::Clear => {
                self.daily_active_user.clear();
                self.read_write_ratio.clear();
                self.data_size.clear();
                self.read_per_second = 0.0;
                self.write_per_second = 0.0;
                self.storage_used_per_year = 0;
                self.error = None;
            }
            Message::CopyToClipboard => {
                let results = self.format_results();
                if !results.is_empty() {
                    if let Ok(mut clipboard) = Clipboard::new() {
                        let _ = clipboard.set_text(&results);
                    }
                }
            }
        }
    }

    fn calculate_back_of_envelope(&self) -> Result<(f64, f64, f64), String> {
        let dau = self.daily_active_user.parse::<u64>()
            .map_err(|_| "Invalid Daily Active User number".to_string())?;
        
        let size = self.data_size.parse::<f64>()
            .map_err(|_| "Invalid data size".to_string())?;

        let ratio: Vec<&str> = self.read_write_ratio.split(':').collect();
        if ratio.len() != 2 {
            return Err("Invalid ratio format. Use format like '1:1' or '10:1'".to_string());
        }

        let read_ratio = ratio[0].parse::<f64>()
            .map_err(|_| "Invalid read ratio".to_string())?;
        let write_ratio = ratio[1].parse::<f64>()
            .map_err(|_| "Invalid write ratio".to_string())?;

        let read_per_second = (dau as f64 * read_ratio) / SECOND_IN_DAY as f64;
        let write_per_second = (dau as f64 * write_ratio) / SECOND_IN_DAY as f64;
        let storage_used_per_year = (size * write_per_second) * DAY_IN_YEAR as f64;

        Ok((read_per_second, write_per_second, storage_used_per_year))
    }

    fn format_results(&self) -> String {
        if self.read_per_second == 0.0 && self.write_per_second == 0.0 {
            return String::new();
        }

        let kb = self.storage_used_per_year / 1024;
        let mb = kb / 1024;
        let gb = mb / 1024;
        let tb = gb / 1024;
        let pb = tb / 1024;

        format!(
            "Back of the Envelope Calculations Results:\n\n\
            Read per second: {:.6} rps\n\
            Write per second: {:.6} tps\n\n\
            Storage used per year (roughly calculated from Write per second):\n\
            {} Byte\n\
            {} KB\n\
            {} MB\n\
            {} GB\n\
            {} TB\n\
            {} PB\n\n\
            The rest, Sum/Multiply them by yourself, you already got foundation value",
            self.read_per_second,
            self.write_per_second,
            self.storage_used_per_year,
            kb,
            mb,
            gb,
            tb,
            pb
        )
    }

    pub fn view(&self) -> Element<Message> {
        let description = column![
            text("Back of the envelope calculations").size(24),
            text("Assumptions:").size(16),
            text("• Assume DAU (Daily Active User)").size(12),
            text("• Adjust read:write ratio - one of them need to be 1 for based calculation").size(12),
            text("• Adjust number you want to calculate read/write per seconds").size(12),
            text("• Assume data size of interest payload").size(12),
        ]
        .spacing(5);

        let left_column = column![
            text("Daily Active User").size(16),
            text_input("Daily Active User", &self.daily_active_user)
                .on_input(Message::DailyActiveUserChanged)
                .size(14)
                .padding(10),
            text("Read:Write Ratio").size(16),
            text_input("Read:Write Ratio (e.g., 1:1, 10:1)", &self.read_write_ratio)
                .on_input(Message::ReadWriteRatioChanged)
                .size(14)
                .padding(10),
            text("Data size of interest payload (Byte)").size(16),
            text_input("Data size of interest payload in byte", &self.data_size)
                .on_input(Message::DataSizeChanged)
                .size(14)
                .padding(10),
            row![
                button(text("Calculate").size(14))
                    .on_press(Message::Calculate)
                    .padding(10),
                button(text("Clear").size(14))
                    .on_press(Message::Clear)
                    .padding(10),
            ]
            .spacing(10),
        ]
        .spacing(10)
        .width(Length::FillPortion(1));

        let right_column = if self.read_per_second != 0.0 || self.write_per_second != 0.0 {
            let kb = self.storage_used_per_year / 1024;
            let mb = kb / 1024;
            let gb = mb / 1024;
            let tb = gb / 1024;
            let pb = tb / 1024;

            column![
                row![
                    text("Results").size(16),
                    button(text("📋 Copy").size(12))
                        .on_press(Message::CopyToClipboard)
                        .padding([5, 10]),
                ]
                .spacing(10)
                .align_items(iced::Alignment::Center),
                text("Read per second").size(14),
                text(format!("{:.6} rps", self.read_per_second)).size(14),
                text("Write per second").size(14),
                text(format!("{:.6} tps", self.write_per_second)).size(14),
                text("Storage used per year (roughly calculated from Write per second)").size(14),
                text(format!("{} Byte", self.storage_used_per_year)).size(12),
                text(format!("{} KB", kb)).size(12),
                text(format!("{} MB", mb)).size(12),
                text(format!("{} GB", gb)).size(12),
                text(format!("{} TB", tb)).size(12),
                text(format!("{} PB", pb)).size(12),
                text("The rest, Sum/Multiply them by yourself, you already got foundation value").size(14),
            ]
            .spacing(10)
            .width(Length::FillPortion(1))
        } else {
            column![]
                .width(Length::FillPortion(1))
        };

        let mut content = Column::new()
            .spacing(20)
            .push(description)
            .push(row![left_column, right_column].spacing(20));

        if let Some(error) = &self.error {
            content = content.push(
                text(error)
                    .size(14)
                    .style(iced::theme::Text::Color(iced::Color::from_rgb(0.8, 0.2, 0.2)))
            );
        }

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(20)
            .into()
    }
}