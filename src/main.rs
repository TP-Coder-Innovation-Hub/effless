#![allow(non_snake_case)]

use dioxus::prelude::*;
use std::collections::HashMap;

mod tools;
mod logic;

use tools::{Tool, ToolType};

fn main() {
    dioxus::launch(App);
}

fn App() -> Element {
    let selected_tool = use_signal(|| ToolType::Base64);
    let current_tool = use_signal(|| Tool::new(ToolType::Base64));
    let mut search_query = use_signal(String::new);

    rsx! {
        div {
            style: "display: flex; height: 100%; font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif; overflow: hidden; position: absolute; top: 0; left: 0; right: 0; bottom: 0;",
            
            // Sidebar
            div {
                style: "width: 250px; background-color: #2c3e50; padding: 20px; overflow-y: auto; flex-shrink: 0; box-sizing: border-box;",
                
                // Search input
                input {
                    style: "width: calc(100% - 16px); padding: 8px; margin-bottom: 20px; border: none; border-radius: 4px; background-color: #34495e; color: #ecf0f1; font-size: 14px;",
                    placeholder: "🔍 Search tools...",
                    value: "{search_query.read()}",
                    oninput: move |event| {
                        search_query.set(event.value());
                    }
                }
                
                // Tool categories and buttons
                {render_sidebar(selected_tool, current_tool, &search_query.read())}
            }
            
            // Main tool area
            div {
                style: "flex: 1; background-color: #ecf0f1; display: flex; flex-direction: column; min-height: 0; box-sizing: border-box; overflow: hidden;",
                {current_tool.read().view()}
            }
        }
    }
}

fn render_sidebar(
    mut selected_tool: Signal<ToolType>,
    mut current_tool: Signal<Tool>,
    search_query: &str
) -> Element {
    let all_tools = vec![
        (ToolType::Base64, "Base64", "Encoders / Decoders"),
        (ToolType::Uuid, "UUID", "Generators"),
        (ToolType::Ulid, "ULID", "Generators"),
        (ToolType::QrCode, "QR Code", "Generators"),
        (ToolType::Icon, "Icon Generator", "Generators"),
        (ToolType::Distance, "Haversine Distance", "Calculators"),
        (ToolType::SystemDesign, "System Estimator", "System Design"),
    ];

    // Filter tools based on search query
    let filtered_tools: Vec<_> = if search_query.is_empty() {
        all_tools
    } else {
        all_tools
            .into_iter()
            .filter(|(_, tool_name, category)| {
                tool_name.to_lowercase().contains(&search_query.to_lowercase()) ||
                category.to_lowercase().contains(&search_query.to_lowercase())
            })
            .collect()
    };

    // Group filtered tools by category
    let mut categories_map: HashMap<String, Vec<(ToolType, String)>> = HashMap::new();
    
    for (tool_type, tool_name, category) in filtered_tools {
        categories_map.entry(category.to_string()).or_insert_with(Vec::new).push((tool_type, tool_name.to_string()));
    }

    // Sort categories for consistent display
    let mut categories: Vec<_> = categories_map.iter().collect();
    categories.sort_by_key(|(name, _)| name.as_str());

    rsx! {
        div {
            {categories.into_iter().map(|(category_name, tools)| {
                if tools.is_empty() {
                    return rsx! { div {} };
                }
                
                let category_key = format!("category_{}", category_name);
                
                rsx! {
                    div {
                        key: "{category_key}",
                        style: "margin-bottom: 15px;",
                        
                        // Category header
                        div {
                            style: "color: #bdc3c7; font-size: 16px; margin-bottom: 8px; font-weight: 500;",
                            "{category_name}"
                        }
                        
                        // Tool buttons in this category
                        {tools.iter().map(|(tool_type, tool_name)| {
                            let is_selected = selected_tool.read().clone() == *tool_type;
                            let button_style = if is_selected {
                                "width: 100%; padding: 8px 12px; margin-bottom: 4px; border: none; border-radius: 4px; background-color: #3498db; color: white; cursor: pointer; text-align: left; font-size: 14px;"
                            } else {
                                "width: 100%; padding: 8px 12px; margin-bottom: 4px; border: none; border-radius: 4px; background-color: #34495e; color: #ecf0f1; cursor: pointer; text-align: left; font-size: 14px; transition: background-color 0.2s;"
                            };
                            
                            let tool_key = format!("tool_{}_{}", category_name, tool_name);
                            let tool_type_val = *tool_type;
                            
                            rsx! {
                                button {
                                    key: "{tool_key}",
                                    style: "{button_style}",
                                    onclick: move |_| {
                                        selected_tool.set(tool_type_val);
                                        current_tool.set(Tool::new(tool_type_val));
                                    },
                                    "{tool_name}"
                                }
                            }
                        })}
                    }
                }
            })}
        }
    }
}