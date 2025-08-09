#![allow(non_snake_case)]

use dioxus::prelude::*;

pub struct QrTool;

impl QrTool {
    pub fn new() -> Self {
        Self
    }

    pub fn view(&self) -> Element {
        rsx! { QrToolView {} }
    }
}

#[component]
pub fn QrToolView() -> Element {
    let mut input = use_signal(String::new);
    let mut status = use_signal(String::new);

    let generate = move |_| {
        if !input.read().is_empty() {
            status.set(format!("QR Code would be generated for: '{}'", input.read()));
        } else {
            status.set("Please enter text to generate QR code".to_string());
        }
    };

    let clear = move |_| {
        input.set(String::new());
        status.set(String::new());
    };

    rsx! {
        div {
            style: "padding: 20px; height: 100%; display: flex; flex-direction: column; box-sizing: border-box; overflow: hidden;",
            
            h1 {
                style: "font-size: 24px; margin-bottom: 5px; color: #2c3e50; margin-top: 0; flex-shrink: 0;",
                "QR Code Generator"
            }
            
            p {
                style: "font-size: 12px; margin-bottom: 20px; color: #95a5a6; flex-shrink: 0;",
                "Note: QR code generation UI is a placeholder"
            }
            
            // Input section
            div {
                style: "margin-bottom: 20px; flex-shrink: 0;",
                
                h3 {
                    style: "font-size: 16px; margin-bottom: 5px; color: #2c3e50; margin-top: 0;",
                    "Text to encode"
                }
                
                input {
                    style: "width: calc(100% - 20px); padding: 10px; border: 1px solid #bdc3c7; border-radius: 4px; font-size: 14px; box-sizing: border-box;",
                    placeholder: "Enter text for QR code...",
                    value: "{input.read()}",
                    oninput: move |event| {
                        input.set(event.value());
                        status.set(String::new());
                    }
                }
            }
            
            // Buttons
            div {
                style: "margin-bottom: 20px; display: flex; gap: 10px; flex-shrink: 0;",
                
                button {
                    style: "padding: 10px 20px; background-color: #3498db; color: white; border: none; border-radius: 4px; cursor: pointer; font-size: 14px;",
                    onclick: generate,
                    "Generate QR Code"
                }
                
                button {
                    style: "padding: 10px 20px; background-color: #95a5a6; color: white; border: none; border-radius: 4px; cursor: pointer; font-size: 14px;",
                    onclick: clear,
                    "Clear"
                }
            }
            
            // Status section
            div {
                style: "flex: 1; display: flex; flex-direction: column; overflow: hidden;",
                
                if !status.read().is_empty() {
                    div {
                        h3 {
                            style: "font-size: 16px; margin-bottom: 5px; color: #2c3e50; margin-top: 0;",
                            "Status"
                        }
                        p {
                            style: "font-size: 14px; color: #2c3e50;",
                            "{status.read()}"
                        }
                    }
                }
            }
        }
    }
}