#![allow(non_snake_case)]

use dioxus::prelude::*;

pub struct DistanceTool;

impl DistanceTool {
    pub fn new() -> Self {
        Self
    }

    pub fn view(&self) -> Element {
        rsx! { DistanceToolView {} }
    }
}

#[component]
pub fn DistanceToolView() -> Element {
    let mut lat1 = use_signal(String::new);
    let mut lon1 = use_signal(String::new);
    let mut lat2 = use_signal(String::new);
    let mut lon2 = use_signal(String::new);
    let mut result = use_signal(String::new);
    let mut error = use_signal(|| None::<String>);

    let calculate = move |_| {
        use crate::logic::distance_logic::DistanceLogic;
        
        let lat1_val = lat1.read().parse::<f64>();
        let lon1_val = lon1.read().parse::<f64>();
        let lat2_val = lat2.read().parse::<f64>();
        let lon2_val = lon2.read().parse::<f64>();
        
        match (lat1_val, lon1_val, lat2_val, lon2_val) {
            (Ok(lat1), Ok(lon1), Ok(lat2), Ok(lon2)) => {
                match DistanceLogic::calculate_with_validation(lat1, lon1, lat2, lon2) {
                    Ok(distance) => {
                        result.set(format!("{:.2} km ({:.2} miles)", distance.km, distance.miles));
                        error.set(None);
                    }
                    Err(err) => {
                        error.set(Some(format!("{:?}", err)));
                        result.set(String::new());
                    }
                }
            }
            _ => {
                error.set(Some("Please enter valid numeric coordinates".to_string()));
                result.set(String::new());
            }
        }
    };

    let clear = move |_| {
        lat1.set(String::new());
        lon1.set(String::new());
        lat2.set(String::new());
        lon2.set(String::new());
        result.set(String::new());
        error.set(None);
    };

    rsx! {
        div {
            style: "padding: 20px; height: 100%; display: flex; flex-direction: column; box-sizing: border-box; overflow: hidden;",
            
            h1 {
                style: "font-size: 24px; margin-bottom: 5px; color: #2c3e50; margin-top: 0; flex-shrink: 0;",
                "Haversine Distance Calculator"
            }
            
            p {
                style: "font-size: 14px; margin-bottom: 20px; color: #2c3e50; flex-shrink: 0;",
                "Calculate the great-circle distance between two points on Earth"
            }
            
            // Point 1 section
            div {
                style: "margin-bottom: 20px; flex-shrink: 0;",
                
                h3 {
                    style: "font-size: 16px; margin-bottom: 5px; color: #2c3e50; margin-top: 0;",
                    "Point 1"
                }
                
                div {
                    style: "display: flex; gap: 10px;",
                    
                    input {
                        style: "flex: 1; padding: 10px; border: 1px solid #bdc3c7; border-radius: 4px; font-size: 14px; box-sizing: border-box;",
                        placeholder: "Latitude",
                        value: "{lat1.read()}",
                        oninput: move |event| {
                            lat1.set(event.value());
                            error.set(None);
                        }
                    }
                    
                    input {
                        style: "flex: 1; padding: 10px; border: 1px solid #bdc3c7; border-radius: 4px; font-size: 14px; box-sizing: border-box;",
                        placeholder: "Longitude",
                        value: "{lon1.read()}",
                        oninput: move |event| {
                            lon1.set(event.value());
                            error.set(None);
                        }
                    }
                }
            }
            
            // Point 2 section
            div {
                style: "margin-bottom: 20px; flex-shrink: 0;",
                
                h3 {
                    style: "font-size: 16px; margin-bottom: 5px; color: #2c3e50; margin-top: 0;",
                    "Point 2"
                }
                
                div {
                    style: "display: flex; gap: 10px;",
                    
                    input {
                        style: "flex: 1; padding: 10px; border: 1px solid #bdc3c7; border-radius: 4px; font-size: 14px; box-sizing: border-box;",
                        placeholder: "Latitude",
                        value: "{lat2.read()}",
                        oninput: move |event| {
                            lat2.set(event.value());
                            error.set(None);
                        }
                    }
                    
                    input {
                        style: "flex: 1; padding: 10px; border: 1px solid #bdc3c7; border-radius: 4px; font-size: 14px; box-sizing: border-box;",
                        placeholder: "Longitude",
                        value: "{lon2.read()}",
                        oninput: move |event| {
                            lon2.set(event.value());
                            error.set(None);
                        }
                    }
                }
            }
            
            // Buttons
            div {
                style: "margin-bottom: 20px; display: flex; gap: 10px; flex-shrink: 0;",
                
                button {
                    style: "padding: 10px 20px; background-color: #3498db; color: white; border: none; border-radius: 4px; cursor: pointer; font-size: 14px;",
                    onclick: calculate,
                    "Calculate"
                }
                
                button {
                    style: "padding: 10px 20px; background-color: #95a5a6; color: white; border: none; border-radius: 4px; cursor: pointer; font-size: 14px;",
                    onclick: clear,
                    "Clear"
                }
            }
            
            // Result section
            div {
                style: "flex: 1; display: flex; flex-direction: column; overflow: hidden;",
                
                if !result.read().is_empty() {
                    div {
                        h3 {
                            style: "font-size: 16px; margin-bottom: 5px; color: #2c3e50; margin-top: 0;",
                            "Distance"
                        }
                        
                        input {
                            style: "width: calc(100% - 20px); padding: 10px; border: 1px solid #bdc3c7; border-radius: 4px; font-size: 14px; background-color: #f8f9fa; box-sizing: border-box;",
                            readonly: true,
                            value: "{result.read()}"
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
}