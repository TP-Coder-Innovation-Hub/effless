#![allow(non_snake_case)]

use dioxus::prelude::*;
use arboard::Clipboard;

pub struct Base64Tool;

impl Base64Tool {
    pub fn new() -> Self {
        Self
    }

    pub fn view(&self) -> Element {
        rsx! { Base64ToolView {} }
    }
}

#[component]
pub fn Base64ToolView() -> Element {
    let mut input = use_signal(String::new);
    let mut output = use_signal(String::new);
    let mut error = use_signal(|| None::<String>);

    let encode = move |_| {
        use crate::logic::base64_logic::Base64Logic;
        let encoded = Base64Logic::encode(&input.read());
        output.set(encoded);
        error.set(None);
    };

    let decode = move |_| {
        use crate::logic::base64_logic::Base64Logic;
        match Base64Logic::decode(&input.read()) {
            Ok(decoded) => {
                output.set(decoded);
                error.set(None);
            }
            Err(err) => {
                error.set(Some(format!("{:?}", err)));
            }
        }
    };

    let clear = move |_| {
        input.set(String::new());
        output.set(String::new());
        error.set(None);
    };

    let copy_to_clipboard = move |_| {
        if !output.read().is_empty() {
            if let Ok(mut clipboard) = Clipboard::new() {
                let _ = clipboard.set_text(&*output.read());
            }
        }
    };

    rsx! {
        div {
            style: "padding: 20px; height: 100%; display: flex; flex-direction: column; box-sizing: border-box; overflow: hidden;",
            
            h1 {
                style: "font-size: 24px; margin-bottom: 15px; color: #2c3e50; margin-top: 0; flex-shrink: 0;",
                "Base64 Encoder/Decoder"
            }
            
            // Input section
            div {
                style: "margin-bottom: 15px; flex-shrink: 0;",
                
                h3 {
                    style: "font-size: 16px; margin-bottom: 5px; color: #2c3e50; margin-top: 0;",
                    "Input"
                }
                
                textarea {
                    style: "width: calc(100% - 20px); height: 60px; padding: 10px; border: 1px solid #bdc3c7; border-radius: 4px; font-size: 14px; font-family: monospace; resize: none; box-sizing: border-box;",
                    placeholder: "Enter text to encode/decode...",
                    value: "{input.read()}",
                    oninput: move |event| {
                        input.set(event.value());
                        error.set(None);
                    }
                }
            }
            
            // Buttons
            div {
                style: "margin-bottom: 15px; display: flex; gap: 10px; flex-shrink: 0;",
                
                button {
                    style: "padding: 8px 16px; background-color: #3498db; color: white; border: none; border-radius: 4px; cursor: pointer; font-size: 14px;",
                    onclick: encode,
                    "Encode"
                }
                
                button {
                    style: "padding: 8px 16px; background-color: #2ecc71; color: white; border: none; border-radius: 4px; cursor: pointer; font-size: 14px;",
                    onclick: decode,
                    "Decode"
                }
                
                button {
                    style: "padding: 8px 16px; background-color: #95a5a6; color: white; border: none; border-radius: 4px; cursor: pointer; font-size: 14px;",
                    onclick: clear,
                    "Clear"
                }
            }
            
            // Output section
            div {
                style: "flex: 1; display: flex; flex-direction: column; min-height: 0; overflow: hidden;",
                
                div {
                    style: "display: flex; align-items: center; gap: 10px; margin-bottom: 5px;",
                    
                    h3 {
                        style: "font-size: 16px; color: #2c3e50; margin: 0;",
                        "Output"
                    }
                    
                    if !output.read().is_empty() {
                        button {
                            style: "padding: 4px 8px; background-color: #34495e; color: white; border: none; border-radius: 4px; cursor: pointer; font-size: 12px;",
                            onclick: copy_to_clipboard,
                            "📋 Copy"
                        }
                    }
                }
                
                if output.read().is_empty() {
                    div {
                        style: "flex: 1; padding: 10px; border: 1px solid #bdc3c7; border-radius: 4px; background-color: #f8f9fa; display: flex; align-items: center; justify-content: center;",
                        span {
                            style: "color: #95a5a6; font-size: 14px;",
                            "Result will appear here..."
                        }
                    }
                } else {
                    textarea {
                        style: "flex: 1; width: calc(100% - 20px); padding: 10px; border: 1px solid #bdc3c7; border-radius: 4px; font-size: 14px; font-family: monospace; background-color: #f8f9fa; resize: none; box-sizing: border-box; min-height: 0;",
                        readonly: true,
                        value: "{output.read()}"
                    }
                }
            }
            
            // Error message
            if let Some(err) = error.read().as_ref() {
                div {
                    style: "margin-top: 10px; padding: 10px; background-color: #ffebee; border: 1px solid #f44336; border-radius: 4px; color: #c62828; font-size: 14px; flex-shrink: 0;",
                    "{err}"
                }
            }
        }
    }
}