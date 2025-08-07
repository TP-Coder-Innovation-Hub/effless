use iced::{widget::{button, container, row, scrollable, text, text_input, Column}, Application, Command, Element, Length, Settings, Subscription, Theme};

mod tools;

use tools::{Tool, ToolType};

#[derive(Debug, Clone)]
pub enum Message {
    SelectTool(ToolType),
    ToolMessage(tools::Message),
    SearchChanged(String),
}

#[derive(Default)]
pub struct DevTools {
    selected_tool: ToolType,
    current_tool: Tool,
    search_query: String,
}

impl Application for DevTools {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        (
            Self {
                // List availability tools
                selected_tool: ToolType::Base64,
                // Current selection
                current_tool: Tool::new(ToolType::Base64),
                // Search box value
                search_query: String::new(),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        "Effless - Developer Tools ⚒️ by TP Coder".to_string()
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::SelectTool(tool_type) => {
                self.selected_tool = tool_type;
                self.current_tool = Tool::new(tool_type);
                Command::none()
            }
            Message::ToolMessage(tool_msg) => {
                self.current_tool.update(tool_msg);
                Command::none()
            }
            Message::SearchChanged(query) => {
                self.search_query = query;
                Command::none()
            }
        }
    }

    fn view(&self) -> Element<Message> {
        let sidebar = self.sidebar();
        let tool_area = self.tool_area();
        // Separate layout into 2 columns
        row![sidebar, tool_area].into()
    }
}

impl DevTools {
    fn sidebar(&self) -> Element<Message> {
        let all_tools = vec![
            (ToolType::Base64, "Base64", "Converters"),
            (ToolType::Json, "JSON", "Converters"),
            (ToolType::Base64, "Base64 Text", "Encoders / Decoders"),
            (ToolType::Url, "URL", "Encoders / Decoders"),
            (ToolType::Hash, "Hash / Checksum", "Generators"),
            (ToolType::Uuid, "UUID", "Generators"),
            (ToolType::Ulid, "ULID", "Generators"),
            (ToolType::QrCode, "QR Code", "Generators"),
            (ToolType::Distance, "Haversine Distance", "Calculators"),
            (ToolType::SystemDesign, "System Estimator", "System Design"),
        ];

        // Filter tools based on search query
        let filtered_tools: Vec<_> = if self.search_query.is_empty() {
            all_tools
        } else {
            all_tools
                .into_iter()
                .filter(|(_, tool_name, category)| {
                    tool_name.to_lowercase().contains(&self.search_query.to_lowercase()) ||
                    category.to_lowercase().contains(&self.search_query.to_lowercase())
                })
                .collect()
        };

        let mut sidebar_content = Column::new().spacing(10);

        // Add search input
        sidebar_content = sidebar_content.push(
            text_input("🔍 Search tools...", &self.search_query)
                .on_input(Message::SearchChanged)
                .size(14)
                .padding(8)
        );

        // Group filtered tools by category
        let mut categories_map: std::collections::HashMap<&str, Vec<(ToolType, &str)>> = std::collections::HashMap::new();
        
        for (tool_type, tool_name, category) in filtered_tools {
            categories_map.entry(category).or_insert_with(Vec::new).push((tool_type, tool_name));
        }

        // Sort categories for consistent display
        let mut categories: Vec<_> = categories_map.iter().collect();
        categories.sort_by_key(|(name, _)| *name);

        for (category_name, tools) in categories {
            if !tools.is_empty() {
                sidebar_content = sidebar_content.push(
                    text(*category_name)
                        .size(16)
                        .style(iced::theme::Text::Color(iced::Color::from_rgb(0.7, 0.7, 0.7)))
                );

                for (tool_type, tool_name) in tools {
                    let is_selected = self.selected_tool == *tool_type;
                    let button_style = if is_selected {
                        iced::theme::Button::Primary
                    } else {
                        iced::theme::Button::Secondary
                    };

                    sidebar_content = sidebar_content.push(
                        button(text(*tool_name).size(14))
                            .width(Length::Fill)
                            .style(button_style)
                            .on_press(Message::SelectTool(*tool_type))
                    );
                }
            }
        }

        container(scrollable(sidebar_content.padding(20)))
            .width(250)
            .height(Length::Fill)
            .style(iced::theme::Container::Box)
            .into()
    }

    fn tool_area(&self) -> Element<Message> {
        container(
            self.current_tool.view().map(Message::ToolMessage)
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .padding(20)
        .into()
    }
}

fn main() -> iced::Result {
    #[cfg(not(target_arch = "wasm32"))]
    tracing_subscriber::fmt::init();
    DevTools::run(Settings::default())
}
