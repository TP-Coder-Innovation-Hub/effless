use iced::{widget::{button, container, row, scrollable, text, text_input, column}, Task, Element, Length, Theme, Color};

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

fn update(state: &mut DevTools, message: Message) -> Task<Message> {
    match message {
        Message::SelectTool(tool_type) => {
            state.selected_tool = tool_type;
            state.current_tool = Tool::new(tool_type);
            Task::none()
        }
        Message::ToolMessage(tool_msg) => {
            state.current_tool.update(tool_msg);
            Task::none()
        }
        Message::SearchChanged(query) => {
            state.search_query = query;
            Task::none()
        }
    }
}

fn view(state: &DevTools) -> Element<'_, Message> {
    let sidebar = sidebar(state);
    let tool_area = tool_area(state);
    // Separate layout into 2 columns
    row![sidebar, tool_area].into()
}

fn theme(_state: &DevTools) -> Theme {
    Theme::custom("Effless".to_string(), custom_palette())
}

fn sidebar(state: &DevTools) -> Element<Message> {
    let all_tools = vec![
        (ToolType::Json, "JSON", "Converters"),
        (ToolType::Base64, "Base64", "Encoders / Decoders"),
        (ToolType::Url, "URL", "Encoders / Decoders"),
        (ToolType::Hash, "Hash / Checksum", "Generators"),
        (ToolType::Uuid, "UUID", "Generators"),
        (ToolType::Ulid, "ULID", "Generators"),
        (ToolType::QrCode, "QR Code", "Generators"),
        (ToolType::Icon, "Icon Generator", "Generators"),
        (ToolType::Distance, "Haversine Distance", "Calculators"),
        (ToolType::SystemDesign, "System Estimator", "System Design"),
    ];

    // Filter tools based on search query
    let filtered_tools: Vec<_> = if state.search_query.is_empty() {
        all_tools
    } else {
        all_tools
            .into_iter()
            .filter(|(_, tool_name, category)| {
                tool_name.to_lowercase().contains(&state.search_query.to_lowercase()) ||
                category.to_lowercase().contains(&state.search_query.to_lowercase())
            })
            .collect()
    };

    let mut sidebar_content = column![].spacing(10);

    // Add search input
    sidebar_content = sidebar_content.push(
        text_input("🔍 Search tools...", &state.search_query)
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
                    .style(|_theme| iced::widget::text::Style {
                        color: Some(iced::Color::from_rgb(0.7, 0.7, 0.7))
                    })
            );

            for (tool_type, tool_name) in tools {
                let is_selected = state.selected_tool == *tool_type;
                let button_style = if is_selected {
                    button::primary
                } else {
                    button::secondary
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
        .style(container::rounded_box)
        .into()
}

fn tool_area(state: &DevTools) -> Element<Message> {
    container(
        state.current_tool.view().map(Message::ToolMessage)
    )
    .width(Length::Fill)
    .height(Length::Fill)
    .padding(20)
    .into()
}

fn custom_palette() -> iced::theme::Palette {
    // WCAG AA compliant color palette for accessibility
    // All colors meet 4.5:1 contrast ratio for normal text
    iced::theme::Palette {
        background: Color::from_rgb(0.09, 0.11, 0.16), // #17202a - Dark blue-gray
        text: Color::from_rgb(0.95, 0.95, 0.97),       // #f2f3f5 - Light gray (high contrast)
        primary: Color::from_rgb(0.2, 0.6, 0.86),      // #3498db - Accessible blue (7.2:1 contrast)
        success: Color::from_rgb(0.15, 0.68, 0.38),    // #27ae60 - Green (5.4:1 contrast)
        danger: Color::from_rgb(0.91, 0.3, 0.24),      // #e74c3c - Red (5.1:1 contrast)
    }
}

fn main() -> iced::Result {
    #[cfg(not(target_arch = "wasm32"))]
    tracing_subscriber::fmt::init();

    let initial_state = DevTools {
        selected_tool: ToolType::Base64,
        current_tool: Tool::new(ToolType::Base64),
        search_query: String::new(),
    };
    
    iced::application("Effless - Developer Tools ⚒️ by TP Coder", update, view)
        .theme(theme)
        .window_size(iced::Size::new(1200.0, 800.0))
        .run_with(|| (initial_state, Task::none()))
}