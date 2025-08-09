#![allow(non_snake_case)]

use dioxus::prelude::*;
use uuid::Uuid;
use arboard::Clipboard;

#[derive(Default)]
pub struct UuidTool;

impl UuidTool {
    pub fn new() -> Self {
        Self
    }

    pub fn view(&self) -> Element {
        rsx! { UuidToolView {} }
    }
}

#[component]
pub fn UuidToolView() -> Element {
    let mut generated_uuid = use_signal(String::new);
    let mut count = use_signal(|| 0u32);

    let generate = move |_| {
        let uuid = Uuid::new_v4();
        generated_uuid.set(uuid.to_string());
        count.set(count() + 1);
    };

    let clear = move |_| {
        generated_uuid.set(String::new());
        count.set(0);
    };

    let copy_to_clipboard = move |_| {
        if !generated_uuid.read().is_empty() {
            if let Ok(mut clipboard) = Clipboard::new() {
                let _ = clipboard.set_text(&*generated_uuid.read());
            }
        }
    };

    rsx! {
        div {
            style: "padding: 20px; height: 100%; display: flex; flex-direction: column; box-sizing: border-box; overflow: hidden;",
            
            h1 {
                style: "font-size: 24px; margin-bottom: 5px; color: #2c3e50; margin-top: 0; flex-shrink: 0;",
                "UUID v4 Generator"
            }
            
            p {
                style: "font-size: 14px; margin-bottom: 20px; color: #2c3e50; flex-shrink: 0;",
                "Generates random UUIDs using version 4 (random)"
            }
            
            // Buttons
            div {
                style: "margin-bottom: 20px; display: flex; gap: 10px; flex-shrink: 0;",
                
                button {
                    style: "padding: 10px 20px; background-color: #3498db; color: white; border: none; border-radius: 4px; cursor: pointer; font-size: 14px;",
                    onclick: generate,
                    "Generate UUID"
                }
                
                button {
                    style: "padding: 10px 20px; background-color: #95a5a6; color: white; border: none; border-radius: 4px; cursor: pointer; font-size: 14px;",
                    onclick: clear,
                    "Clear"
                }
            }
            
            // Output section
            div {
                style: "flex: 1; display: flex; flex-direction: column; overflow: hidden;",
                
                div {
                    style: "display: flex; align-items: center; gap: 10px; margin-bottom: 5px;",
                    
                    h3 {
                        style: "font-size: 16px; color: #2c3e50; margin: 0;",
                        "Generated UUID"
                    }
                    
                    if !generated_uuid.read().is_empty() {
                        button {
                            style: "padding: 5px 10px; background-color: #34495e; color: white; border: none; border-radius: 4px; cursor: pointer; font-size: 12px;",
                            onclick: copy_to_clipboard,
                            "📋 Copy"
                        }
                    }
                }
                
                if generated_uuid.read().is_empty() {
                    div {
                        style: "padding: 10px; border: 1px solid #bdc3c7; border-radius: 4px; background-color: #f8f9fa; display: flex; align-items: center; justify-content: center;",
                        span {
                            style: "color: #95a5a6; font-size: 14px;",
                            "Click 'Generate UUID' to create a new UUID"
                        }
                    }
                } else {
                    input {
                        style: "width: calc(100% - 20px); padding: 10px; border: 1px solid #bdc3c7; border-radius: 4px; font-size: 14px; font-family: monospace; background-color: #f8f9fa; box-sizing: border-box;",
                        readonly: true,
                        value: "{generated_uuid.read()}"
                    }
                }
                
                if count() > 0 {
                    p {
                        style: "margin-top: 5px; font-size: 12px; color: #95a5a6; margin-bottom: 0;",
                        "Total generated: {count()}"
                    }
                }
            }
        }
    }
}