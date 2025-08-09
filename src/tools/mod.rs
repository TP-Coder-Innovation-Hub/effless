use dioxus::prelude::*;

pub mod base64_tool;
pub mod uuid_tool;
pub mod ulid_tool;
pub mod qr_tool;
pub mod distance_tool;
pub mod system_design_tool;
pub mod icon_tool;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ToolType {
    Base64,
    Uuid,
    Ulid,
    QrCode,
    Distance,
    SystemDesign,
    Icon,
}

impl Default for ToolType {
    fn default() -> Self {
        ToolType::Base64
    }
}

pub enum Tool {
    Base64(base64_tool::Base64Tool),
    Uuid(uuid_tool::UuidTool),
    Ulid(ulid_tool::UlidTool),
    QrCode(qr_tool::QrTool),
    Distance(distance_tool::DistanceTool),
    SystemDesign(system_design_tool::SystemDesignTool),
    Icon(icon_tool::IconTool),
}

impl Tool {
    pub fn new(tool_type: ToolType) -> Self {
        match tool_type {
            ToolType::Base64 => Tool::Base64(base64_tool::Base64Tool::new()),
            ToolType::Uuid => Tool::Uuid(uuid_tool::UuidTool::new()),
            ToolType::Ulid => Tool::Ulid(ulid_tool::UlidTool::new()),
            ToolType::QrCode => Tool::QrCode(qr_tool::QrTool::new()),
            ToolType::Distance => Tool::Distance(distance_tool::DistanceTool::new()),
            ToolType::SystemDesign => Tool::SystemDesign(system_design_tool::SystemDesignTool::new()),
            ToolType::Icon => Tool::Icon(icon_tool::IconTool::new()),
        }
    }

    pub fn view(&self) -> Element {
        match self {
            Tool::Base64(tool) => tool.view(),
            Tool::Uuid(tool) => tool.view(),
            Tool::Ulid(tool) => tool.view(),
            Tool::QrCode(tool) => tool.view(),
            Tool::Distance(tool) => tool.view(),
            Tool::SystemDesign(tool) => tool.view(),
            Tool::Icon(tool) => tool.view(),
        }
    }
}

impl Default for Tool {
    fn default() -> Self {
        Tool::new(ToolType::default())
    }
}