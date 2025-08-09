#![allow(non_snake_case)]

use dioxus::prelude::*;
use ulid::Ulid;
use arboard::Clipboard;

pub struct UlidTool;

impl UlidTool {
    pub fn new() -> Self {
        Self
    }

    pub fn view(&self) -> Element {
        rsx! { UlidToolView {} }
    }
}

#[component]
pub fn UlidToolView() -> Element {
    let mut generated_ulid = use_signal(String::new);
    let mut count = use_signal(|| 0u32);

    let generate = move |_| {
        let ulid = Ulid::new();
        generated_ulid.set(ulid.to_string());
        count.set(count() + 1);
    };

    let clear = move |_| {
        generated_ulid.set(String::new());
        count.set(0);
    };

    let copy_to_clipboard = move |_| {
        if !generated_ulid.read().is_empty() {
            if let Ok(mut clipboard) = Clipboard::new() {
                let _ = clipboard.set_text(&*generated_ulid.read());
            }
        }
    };

    rsx! {
        div {
            style: "padding: 20px; height: 100%; display: flex; flex-direction: column; box-sizing: border-box; overflow: hidden;",
            
            h1 {
                style: "font-size: 24px; margin-bottom: 5px; color: #2c3e50; margin-top: 0; flex-shrink: 0;",
                "ULID Generator"
            }
            
            p {
                style: "font-size: 14px; margin-bottom: 5px; color: #2c3e50; flex-shrink: 0;",
                "Generates Universally Unique Lexicographically Sortable Identifiers"
            }
            
            p {
                style: "font-size: 12px; margin-bottom: 20px; color: #95a5a6; flex-shrink: 0;",
                "ULIDs are timestamp-sortable and URL-safe"
            }
            
            // Buttons
            div {
                style: "margin-bottom: 20px; display: flex; gap: 10px; flex-shrink: 0;",
                
                button {
                    style: "padding: 10px 20px; background-color: #3498db; color: white; border: none; border-radius: 4px; cursor: pointer; font-size: 14px;",
                    onclick: generate,
                    "Generate ULID"
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
                        "Generated ULID"
                    }
                    
                    if !generated_ulid.read().is_empty() {
                        button {
                            style: "padding: 5px 10px; background-color: #34495e; color: white; border: none; border-radius: 4px; cursor: pointer; font-size: 12px;",
                            onclick: copy_to_clipboard,
                            "📋 Copy"
                        }
                    }
                }
                
                if generated_ulid.read().is_empty() {
                    div {
                        style: "padding: 10px; border: 1px solid #bdc3c7; border-radius: 4px; background-color: #f8f9fa; display: flex; align-items: center; justify-content: center;",
                        span {
                            style: "color: #95a5a6; font-size: 14px;",
                            "Click 'Generate ULID' to create a new ULID"
                        }
                    }
                } else {
                    input {
                        style: "width: calc(100% - 20px); padding: 10px; border: 1px solid #bdc3c7; border-radius: 4px; font-size: 14px; font-family: monospace; background-color: #f8f9fa; box-sizing: border-box;",
                        readonly: true,
                        value: "{generated_ulid.read()}"
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