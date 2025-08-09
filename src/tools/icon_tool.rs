#![allow(non_snake_case)]

use dioxus::prelude::*;
use arboard::Clipboard;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IconShape {
    Circle,
    Square,
}

impl std::fmt::Display for IconShape {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IconShape::Circle => write!(f, "Circle"),
            IconShape::Square => write!(f, "Square"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExportFormat {
    Png,
    Ico,
}

impl std::fmt::Display for ExportFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ExportFormat::Png => write!(f, "PNG"),
            ExportFormat::Ico => write!(f, "ICO"),
        }
    }
}

pub struct IconTool;

impl Default for IconShape {
    fn default() -> Self {
        IconShape::Circle
    }
}

impl Default for ExportFormat {
    fn default() -> Self {
        ExportFormat::Png
    }
}

impl IconTool {
    pub fn new() -> Self {
        Self
    }

    pub fn view(&self) -> Element {
        rsx! { IconToolView {} }
    }
}

fn generate_preview_svg(text: &str, shape: IconShape, size: u32, bg_color: &str, text_color: &str) -> String {
        let display_text = if text.len() > 3 {
            &text[..3]
        } else {
            text
        };

        let font_size = match display_text.len() {
            0 => size / 4,
            1 => size / 2,
            2 => size / 3,
            3 => size / 4,
            _ => size / 4,
        };

        let shape_element = match shape {
            IconShape::Circle => {
                let radius = size / 2;
                format!("<circle cx=\"{}\" cy=\"{}\" r=\"{}\" fill=\"{}\"/>", 
                    radius, radius, radius, bg_color)
            },
            IconShape::Square => {
                format!("<rect width=\"{}\" height=\"{}\" fill=\"{}\"/>", size, size, bg_color)
            }
        };

        let text_element = if !display_text.is_empty() {
            format!(
                "<text x=\"50%\" y=\"50%\" text-anchor=\"middle\" dominant-baseline=\"middle\" fill=\"{}\" font-family=\"Arial, sans-serif\" font-size=\"{}\" font-weight=\"bold\">{}</text>",
                text_color, font_size, display_text
            )
        } else {
            String::new()
        };

        format!(
            "<svg width=\"{}\" height=\"{}\" xmlns=\"http://www.w3.org/2000/svg\">{}{}</svg>",
            size, size, shape_element, text_element
        )
    }

fn simple_base64_encode(input: &str) -> String {
        // Simple base64 encoding without external dependencies
        
        let chars = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
        let mut result = String::new();
        let bytes = input.as_bytes();
        
        for chunk in bytes.chunks(3) {
            let mut buffer = [0u8; 3];
            for (i, &byte) in chunk.iter().enumerate() {
                buffer[i] = byte;
            }
            
            let combined = ((buffer[0] as u32) << 16) | ((buffer[1] as u32) << 8) | (buffer[2] as u32);
            
            result.push(chars.chars().nth(((combined >> 18) & 0x3F) as usize).unwrap());
            result.push(chars.chars().nth(((combined >> 12) & 0x3F) as usize).unwrap());
            result.push(if chunk.len() > 1 { chars.chars().nth(((combined >> 6) & 0x3F) as usize).unwrap() } else { '=' });
            result.push(if chunk.len() > 2 { chars.chars().nth((combined & 0x3F) as usize).unwrap() } else { '=' });
        }
        
        result
    }

#[component]
pub fn IconToolView() -> Element {
    let mut text = use_signal(String::new);
    let mut shape = use_signal(|| IconShape::Circle);
    let mut size = use_signal(|| 128u32);
    let mut background_color = use_signal(|| "#3498db".to_string());
    let mut text_color = use_signal(|| "#ffffff".to_string());
    let mut export_format = use_signal(|| ExportFormat::Png);
    let mut preview_data = use_signal(String::new);
    let mut status = use_signal(|| None::<String>);

    // Generate preview whenever inputs change
    let mut generate_preview = move || {
        let svg = generate_preview_svg(&text.read(), shape(), size(), &background_color.read(), &text_color.read());
        let data_url = format!("data:image/svg+xml;base64,{}", simple_base64_encode(&svg));
        preview_data.set(data_url);
    };

    let copy_base64 = move |_| {
        let svg = generate_preview_svg(&text.read(), shape(), size(), &background_color.read(), &text_color.read());
        let data_url = format!("data:image/svg+xml;base64,{}", simple_base64_encode(&svg));
        
        if let Ok(mut clipboard) = Clipboard::new() {
            let _ = clipboard.set_text(data_url);
            status.set(Some("Base64 data copied to clipboard!".to_string()));
        } else {
            status.set(Some("Failed to copy to clipboard".to_string()));
        }
    };

    let download_icon = move |_| {
        status.set(Some("Download functionality not implemented yet - this would save the icon as a file".to_string()));
    };

    let clear = move |_| {
        text.set(String::new());
        shape.set(IconShape::Circle);
        size.set(128);
        background_color.set("#3498db".to_string());
        text_color.set("#ffffff".to_string());
        export_format.set(ExportFormat::Png);
        preview_data.set(String::new());
        status.set(None);
    };

    // Generate initial preview
    use_effect(move || {
        generate_preview();
    });

    rsx! {
        div {
            style: "padding: 20px; height: 100%; display: flex; flex-direction: column; box-sizing: border-box; overflow: hidden;",
                
                h1 {
                    style: "font-size: 24px; margin-bottom: 20px; color: #2c3e50;",
                    "Icon Generator"
                }
                
                div {
                    style: "display: flex; gap: 20px;",
                    
                    // Left column - Controls
                    div {
                        style: "flex: 1;",
                        
                        // Text input
                        div {
                            style: "margin-bottom: 15px;",
                            
                            h3 {
                                style: "font-size: 16px; margin-bottom: 5px; color: #2c3e50;",
                                "Text (max 3 characters)"
                            }
                            
                            input {
                                style: "width: 100%; padding: 10px; border: 1px solid #bdc3c7; border-radius: 4px; font-size: 14px;",
                                placeholder: "Enter text (max 3 chars)...",
                                value: "{text.read()}",
                                maxlength: "3",
                                oninput: move |event| {
                                    let new_text = event.value();
                                    if new_text.len() <= 3 {
                                        text.set(new_text);
                                        generate_preview();
                                        status.set(None);
                                    }
                                }
                            }
                        }
                        
                        // Shape selection
                        div {
                            style: "margin-bottom: 15px;",
                            
                            h3 {
                                style: "font-size: 16px; margin-bottom: 5px; color: #2c3e50;",
                                "Shape"
                            }
                            
                            select {
                                style: "width: 100%; padding: 10px; border: 1px solid #bdc3c7; border-radius: 4px; font-size: 14px; background-color: white;",
                                onchange: move |event| {
                                    let selected_shape = match event.value().as_str() {
                                        "Circle" => IconShape::Circle,
                                        "Square" => IconShape::Square,
                                        _ => IconShape::Circle,
                                    };
                                    shape.set(selected_shape);
                                    generate_preview();
                                },
                                
                                option {
                                    value: "Circle",
                                    selected: matches!(shape(), IconShape::Circle),
                                    "Circle"
                                }
                                option {
                                    value: "Square",
                                    selected: matches!(shape(), IconShape::Square),
                                    "Square"
                                }
                            }
                        }
                        
                        // Size input
                        div {
                            style: "margin-bottom: 15px;",
                            
                            h3 {
                                style: "font-size: 16px; margin-bottom: 5px; color: #2c3e50;",
                                "Size (pixels)"
                            }
                            
                            input {
                                r#type: "number",
                                style: "width: 100%; padding: 10px; border: 1px solid #bdc3c7; border-radius: 4px; font-size: 14px;",
                                value: "{size()}",
                                min: "16",
                                max: "512",
                                oninput: move |event| {
                                    if let Ok(new_size) = event.value().parse::<u32>() {
                                        if new_size >= 16 && new_size <= 512 {
                                            size.set(new_size);
                                            generate_preview();
                                        }
                                    }
                                }
                            }
                        }
                        
                        // Background color
                        div {
                            style: "margin-bottom: 15px;",
                            
                            h3 {
                                style: "font-size: 16px; margin-bottom: 5px; color: #2c3e50;",
                                "Background Color"
                            }
                            
                            div {
                                style: "display: flex; gap: 10px; align-items: center;",
                                
                                input {
                                    r#type: "color",
                                    style: "width: 50px; height: 40px; border: 1px solid #bdc3c7; border-radius: 4px; cursor: pointer;",
                                    value: "{background_color.read()}",
                                    oninput: move |event| {
                                        background_color.set(event.value());
                                        generate_preview();
                                    }
                                }
                                
                                input {
                                    r#type: "text",
                                    style: "flex: 1; padding: 10px; border: 1px solid #bdc3c7; border-radius: 4px; font-size: 14px;",
                                    value: "{background_color.read()}",
                                    placeholder: "#3498db",
                                    oninput: move |event| {
                                        let value = event.value();
                                        if value.starts_with('#') && (value.len() == 7 || value.len() == 4) {
                                            background_color.set(value);
                                            generate_preview();
                                        }
                                    }
                                }
                            }
                        }
                        
                        // Text color
                        div {
                            style: "margin-bottom: 15px;",
                            
                            h3 {
                                style: "font-size: 16px; margin-bottom: 5px; color: #2c3e50;",
                                "Text Color"
                            }
                            
                            div {
                                style: "display: flex; gap: 10px; align-items: center;",
                                
                                input {
                                    r#type: "color",
                                    style: "width: 50px; height: 40px; border: 1px solid #bdc3c7; border-radius: 4px; cursor: pointer;",
                                    value: "{text_color.read()}",
                                    oninput: move |event| {
                                        text_color.set(event.value());
                                        generate_preview();
                                    }
                                }
                                
                                input {
                                    r#type: "text",
                                    style: "flex: 1; padding: 10px; border: 1px solid #bdc3c7; border-radius: 4px; font-size: 14px;",
                                    value: "{text_color.read()}",
                                    placeholder: "#ffffff",
                                    oninput: move |event| {
                                        let value = event.value();
                                        if value.starts_with('#') && (value.len() == 7 || value.len() == 4) {
                                            text_color.set(value);
                                            generate_preview();
                                        }
                                    }
                                }
                            }
                        }
                        
                        // Export format
                        div {
                            style: "margin-bottom: 15px;",
                            
                            h3 {
                                style: "font-size: 16px; margin-bottom: 5px; color: #2c3e50;",
                                "Export Format"
                            }
                            
                            select {
                                style: "width: 100%; padding: 10px; border: 1px solid #bdc3c7; border-radius: 4px; font-size: 14px; background-color: white;",
                                onchange: move |event| {
                                    let selected_format = match event.value().as_str() {
                                        "PNG" => ExportFormat::Png,
                                        "ICO" => ExportFormat::Ico,
                                        _ => ExportFormat::Png,
                                    };
                                    export_format.set(selected_format);
                                },
                                
                                option {
                                    value: "PNG",
                                    selected: matches!(export_format(), ExportFormat::Png),
                                    "PNG"
                                }
                                option {
                                    value: "ICO",
                                    selected: matches!(export_format(), ExportFormat::Ico),
                                    "ICO"
                                }
                            }
                        }
                        
                        // Buttons
                        div {
                            style: "display: flex; flex-direction: column; gap: 10px;",
                            
                            button {
                                style: "padding: 10px 20px; background-color: #3498db; color: white; border: none; border-radius: 4px; cursor: pointer; font-size: 14px;",
                                onclick: download_icon,
                                "📥 Download Icon"
                            }
                            
                            button {
                                style: "padding: 10px 20px; background-color: #2ecc71; color: white; border: none; border-radius: 4px; cursor: pointer; font-size: 14px;",
                                onclick: copy_base64,
                                "📋 Copy Base64"
                            }
                            
                            button {
                                style: "padding: 10px 20px; background-color: #95a5a6; color: white; border: none; border-radius: 4px; cursor: pointer; font-size: 14px;",
                                onclick: clear,
                                "Clear"
                            }
                        }
                    }
                    
                    // Right column - Preview
                    div {
                        style: "flex: 1; display: flex; flex-direction: column; align-items: center;",
                        
                        h3 {
                            style: "font-size: 16px; margin-bottom: 15px; color: #2c3e50;",
                            "Live Preview"
                        }
                        
                        div {
                            style: "padding: 20px; border: 2px dashed #bdc3c7; border-radius: 8px; background-color: #f8f9fa; display: flex; justify-content: center; align-items: center; min-height: 200px;",
                            
                            if !preview_data.read().is_empty() {
                                img {
                                    src: "{preview_data.read()}",
                                    alt: "Icon Preview",
                                    style: "max-width: 128px; max-height: 128px; border: 1px solid #dee2e6; border-radius: 4px;",
                                }
                            } else {
                                div {
                                    style: "color: #95a5a6; text-align: center;",
                                    p { "Preview will appear here" }
                                    p { 
                                        style: "font-size: 12px;",
                                        "Enter text and customize settings"
                                    }
                                }
                            }
                        }
                        
                        div {
                            style: "margin-top: 15px; text-align: center; font-size: 12px; color: #95a5a6;",
                            p { "Size: {size()}x{size()} pixels" }
                            p { "Format: {export_format()}" }
                            if !text.read().is_empty() {
                                p { "Text: \"{text.read()}\"" }
                            }
                        }
                    }
                }
                
            // Status message
            if let Some(msg) = status.read().as_ref() {
                div {
                    style: "margin-top: 20px; padding: 10px; background-color: #e8f5e8; border: 1px solid #4caf50; border-radius: 4px; color: #2e7d32; font-size: 14px;",
                    "{msg}"
                }
            }
        }
    }
}