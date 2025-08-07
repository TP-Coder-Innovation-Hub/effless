use iced::Element;

pub mod base64_tool;
pub mod distance_tool;
pub mod hash_tool;
pub mod icon;
pub mod json_tool;
pub mod qr_tool;
pub mod system_design_tool;
pub mod ulid_tool;
pub mod url_tool;
pub mod uuid_tool;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ToolType {
    Base64,
    Json,
    Uuid,
    Hash,
    Ulid,
    QrCode,
    Distance,
    SystemDesign,
    Url,
    Icon,
}

impl Default for ToolType {
    fn default() -> Self {
        ToolType::Base64
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    Base64(base64_tool::Message),
    Json(json_tool::Message),
    Uuid(uuid_tool::Message),
    Hash(hash_tool::Message),
    Ulid(ulid_tool::Message),
    QrCode(qr_tool::Message),
    Distance(distance_tool::Message),
    SystemDesign(system_design_tool::Message),
    Url(url_tool::Message),
    Icon(icon::Message),
}

pub enum Tool {
    Base64(base64_tool::Base64Tool),
    Json(json_tool::JsonTool),
    Uuid(uuid_tool::UuidTool),
    Hash(hash_tool::HashTool),
    Ulid(ulid_tool::UlidTool),
    QrCode(qr_tool::QrTool),
    Distance(distance_tool::DistanceTool),
    SystemDesign(system_design_tool::SystemDesignTool),
    Url(url_tool::UrlTool),
    Icon(icon::IconTool),
}

impl Tool {
    pub fn new(tool_type: ToolType) -> Self {
        match tool_type {
            ToolType::Base64 => Tool::Base64(base64_tool::Base64Tool::new()),
            ToolType::Json => Tool::Json(json_tool::JsonTool::new()),
            ToolType::Uuid => Tool::Uuid(uuid_tool::UuidTool::new()),
            ToolType::Hash => Tool::Hash(hash_tool::HashTool::new()),
            ToolType::Ulid => Tool::Ulid(ulid_tool::UlidTool::new()),
            ToolType::QrCode => Tool::QrCode(qr_tool::QrTool::new()),
            ToolType::Distance => Tool::Distance(distance_tool::DistanceTool::new()),
            ToolType::SystemDesign => {
                Tool::SystemDesign(system_design_tool::SystemDesignTool::new())
            }
            ToolType::Url => Tool::Url(url_tool::UrlTool::new()),
            ToolType::Icon => Tool::Icon(icon::IconTool::new()),
        }
    }

    pub fn update(&mut self, message: Message) {
        match (self, message) {
            (Tool::Base64(tool), Message::Base64(msg)) => tool.update(msg),
            (Tool::Json(tool), Message::Json(msg)) => tool.update(msg),
            (Tool::Uuid(tool), Message::Uuid(msg)) => tool.update(msg),
            (Tool::Hash(tool), Message::Hash(msg)) => tool.update(msg),
            (Tool::Ulid(tool), Message::Ulid(msg)) => tool.update(msg),
            (Tool::QrCode(tool), Message::QrCode(msg)) => tool.update(msg),
            (Tool::Distance(tool), Message::Distance(msg)) => tool.update(msg),
            (Tool::SystemDesign(tool), Message::SystemDesign(msg)) => tool.update(msg),
            (Tool::Url(tool), Message::Url(msg)) => tool.update(msg),
            (Tool::Icon(tool), Message::Icon(msg)) => tool.update(msg),
            _ => {}
        }
    }

    pub fn view(&self) -> Element<Message> {
        match self {
            Tool::Base64(tool) => tool.view().map(Message::Base64),
            Tool::Json(tool) => tool.view().map(Message::Json),
            Tool::Uuid(tool) => tool.view().map(Message::Uuid),
            Tool::Hash(tool) => tool.view().map(Message::Hash),
            Tool::Ulid(tool) => tool.view().map(Message::Ulid),
            Tool::QrCode(tool) => tool.view().map(Message::QrCode),
            Tool::Distance(tool) => tool.view().map(Message::Distance),
            Tool::SystemDesign(tool) => tool.view().map(Message::SystemDesign),
            Tool::Url(tool) => tool.view().map(Message::Url),
            Tool::Icon(tool) => tool.view().map(Message::Icon),
        }
    }
}

impl Default for Tool {
    fn default() -> Self {
        Tool::new(ToolType::default())
    }
}
