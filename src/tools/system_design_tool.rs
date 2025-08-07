use iced::{
    widget::{button, column, container, pick_list, row, scrollable, text, text_input, Column},
    Element, Length,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScaleType {
    Small,
    Medium,
    Large,
    Enterprise,
}

impl ScaleType {
    const ALL: [ScaleType; 4] = [
        ScaleType::Small,
        ScaleType::Medium,
        ScaleType::Large,
        ScaleType::Enterprise,
    ];
}

impl std::fmt::Display for ScaleType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ScaleType::Small => write!(f, "Small (< 1K users)"),
            ScaleType::Medium => write!(f, "Medium (1K - 100K users)"),
            ScaleType::Large => write!(f, "Large (100K - 1M users)"),
            ScaleType::Enterprise => write!(f, "Enterprise (> 1M users)"),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    UsersChanged(String),
    ScaleSelected(ScaleType),
    Calculate,
    Clear,
}

pub struct SystemDesignTool {
    users: String,
    scale: ScaleType,
    estimates: String,
}

impl Default for SystemDesignTool {
    fn default() -> Self {
        Self {
            users: String::new(),
            scale: ScaleType::Small,
            estimates: String::new(),
        }
    }
}

impl SystemDesignTool {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::UsersChanged(value) => {
                self.users = value;
            }
            Message::ScaleSelected(scale) => {
                self.scale = scale;
            }
            Message::Calculate => {
                self.estimates = self.generate_estimates();
            }
            Message::Clear => {
                self.users.clear();
                self.estimates.clear();
            }
        }
    }

    fn generate_estimates(&self) -> String {
        let user_count: u64 = self.users.parse().unwrap_or(1000);
        
        let (servers, db_size, bandwidth, storage) = match self.scale {
            ScaleType::Small => (
                (user_count / 500).max(1),
                user_count * 100, // 100KB per user
                user_count / 10,  // 0.1 MB/s per user
                user_count * 1,   // 1MB per user
            ),
            ScaleType::Medium => (
                (user_count / 1000).max(2),
                user_count * 500,
                user_count / 5,
                user_count * 10,
            ),
            ScaleType::Large => (
                (user_count / 2000).max(5),
                user_count * 1000,
                user_count / 2,
                user_count * 50,
            ),
            ScaleType::Enterprise => (
                (user_count / 5000).max(10),
                user_count * 2000,
                user_count,
                user_count * 100,
            ),
        };

        format!(
            "Estimated Resources for {} users:\n\n\
            • Application Servers: {}\n\
            • Database Size: {} MB\n\
            • Bandwidth: {} MB/s\n\
            • Storage: {} MB\n\
            • CDN: {} (recommended for global users)\n\
            • Load Balancer: {} (recommended for high availability)",
            user_count,
            servers,
            db_size / 1024,
            bandwidth,
            storage / 1024,
            if user_count > 10000 { "Yes" } else { "Optional" },
            if servers > 2 { "Yes" } else { "Optional" }
        )
    }

    pub fn view(&self) -> Element<Message> {
        let input_section = column![
            text("Number of Users").size(16),
            text_input("Enter expected number of users...", &self.users)
                .on_input(Message::UsersChanged)
                .size(14)
                .padding(10),
        ]
        .spacing(5);

        let scale_picker = column![
            text("System Scale").size(16),
            pick_list(&ScaleType::ALL[..], Some(self.scale), Message::ScaleSelected)
                .padding(10)
                .text_size(14),
        ]
        .spacing(5);

        let buttons = row![
            button(text("Calculate").size(14))
                .on_press(Message::Calculate)
                .padding(10),
            button(text("Clear").size(14))
                .on_press(Message::Clear)
                .padding(10),
        ]
        .spacing(10);

        let estimates_section = if !self.estimates.is_empty() {
            column![
                text("Resource Estimates").size(16),
                container(
                    scrollable(
                        text_input("", &self.estimates)
                            .size(14)
                    )
                    .height(Length::Fixed(200.0))
                )
                .style(iced::theme::Container::Box)
                .padding(10)
                .width(Length::Fill),
            ]
            .spacing(5)
        } else {
            column![]
        };

        let content = Column::new()
            .spacing(20)
            .push(text("System Design Estimator").size(24))
            .push(text("Rough estimates for system resources based on user count").size(14))
            .push(input_section)
            .push(scale_picker)
            .push(buttons)
            .push(estimates_section);

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(20)
            .into()
    }
}