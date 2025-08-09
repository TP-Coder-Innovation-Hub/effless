#![allow(non_snake_case)]

use dioxus::prelude::*;
use arboard::Clipboard;

const SECOND_IN_MINUTE: u64 = 60;
const MINUTE_IN_HOUR: u64 = 60;
const HOUR_IN_DAY: u64 = 24;
const SECOND_IN_DAY: u64 = SECOND_IN_MINUTE * MINUTE_IN_HOUR * HOUR_IN_DAY; // 86400
const DAY_IN_YEAR: u64 = 365; // Assume 365 days in a year

pub struct SystemDesignTool;

impl SystemDesignTool {
    pub fn new() -> Self {
        Self
    }

    fn calculate_back_of_envelope(dau_str: &str, ratio_str: &str, size_str: &str) -> Result<(f64, f64, f64), String> {
        let dau = dau_str.parse::<u64>()
            .map_err(|_| "Invalid Daily Active User number".to_string())?;
        
        let size = size_str.parse::<f64>()
            .map_err(|_| "Invalid data size".to_string())?;

        let ratio: Vec<&str> = ratio_str.split(':').collect();
        if ratio.len() != 2 {
            return Err("Invalid ratio format. Use format like '1:1' or '10:1'".to_string());
        }

        let read_ratio = ratio[0].parse::<f64>()
            .map_err(|_| "Invalid read ratio".to_string())?;
        let write_ratio = ratio[1].parse::<f64>()
            .map_err(|_| "Invalid write ratio".to_string())?;

        let read_per_second = (dau as f64 * read_ratio) / SECOND_IN_DAY as f64;
        let write_per_second = (dau as f64 * write_ratio) / SECOND_IN_DAY as f64;
        let storage_used_per_year = (size * write_per_second) * DAY_IN_YEAR as f64;

        Ok((read_per_second, write_per_second, storage_used_per_year))
    }

    fn format_results(read_per_second: f64, write_per_second: f64, storage_used_per_year: u64) -> String {
        if read_per_second == 0.0 && write_per_second == 0.0 {
            return String::new();
        }

        let kb = storage_used_per_year / 1024;
        let mb = kb / 1024;
        let gb = mb / 1024;
        let tb = gb / 1024;
        let pb = tb / 1024;

        format!(
            "Back of the Envelope Calculations Results:\n\n\
            Read per second: {:.6} rps\n\
            Write per second: {:.6} tps\n\n\
            Storage used per year (roughly calculated from Write per second):\n\
            {} Byte\n\
            {} KB\n\
            {} MB\n\
            {} GB\n\
            {} TB\n\
            {} PB\n\n\
            The rest, Sum/Multiply them by yourself, you already got foundation value",
            read_per_second,
            write_per_second,
            storage_used_per_year,
            kb,
            mb,
            gb,
            tb,
            pb
        )
    }

    pub fn view(&self) -> Element {
        rsx! { SystemDesignToolView {} }
    }
}

fn calculate_back_of_envelope(dau_str: &str, ratio_str: &str, size_str: &str) -> Result<(f64, f64, f64), String> {
    let dau = dau_str.parse::<u64>()
        .map_err(|_| "Invalid Daily Active User number".to_string())?;
    
    let size = size_str.parse::<f64>()
        .map_err(|_| "Invalid data size".to_string())?;

    let ratio: Vec<&str> = ratio_str.split(':').collect();
    if ratio.len() != 2 {
        return Err("Invalid ratio format. Use format like '1:1' or '10:1'".to_string());
    }

    let read_ratio = ratio[0].parse::<f64>()
        .map_err(|_| "Invalid read ratio".to_string())?;
    let write_ratio = ratio[1].parse::<f64>()
        .map_err(|_| "Invalid write ratio".to_string())?;

    let read_per_second = (dau as f64 * read_ratio) / SECOND_IN_DAY as f64;
    let write_per_second = (dau as f64 * write_ratio) / SECOND_IN_DAY as f64;
    let storage_used_per_year = (size * write_per_second) * DAY_IN_YEAR as f64;

    Ok((read_per_second, write_per_second, storage_used_per_year))
}

fn format_results(read_per_second: f64, write_per_second: f64, storage_used_per_year: u64) -> String {
    if read_per_second == 0.0 && write_per_second == 0.0 {
        return String::new();
    }

    let kb = storage_used_per_year / 1024;
    let mb = kb / 1024;
    let gb = mb / 1024;
    let tb = gb / 1024;
    let pb = tb / 1024;

    format!(
        "Back of the Envelope Calculations Results:\n\n\
        Read per second: {:.6} rps\n\
        Write per second: {:.6} tps\n\n\
        Storage used per year (roughly calculated from Write per second):\n\
        {} Byte\n\
        {} KB\n\
        {} MB\n\
        {} GB\n\
        {} TB\n\
        {} PB\n\n\
        The rest, Sum/Multiply them by yourself, you already got foundation value",
        read_per_second,
        write_per_second,
        storage_used_per_year,
        kb,
        mb,
        gb,
        tb,
        pb
    )
}

#[component]
pub fn SystemDesignToolView() -> Element {
    let mut daily_active_user = use_signal(String::new);
    let mut read_write_ratio = use_signal(String::new);
    let mut data_size = use_signal(String::new);
    let mut read_per_second = use_signal(|| 0.0f64);
    let mut write_per_second = use_signal(|| 0.0f64);
    let mut storage_used_per_year = use_signal(|| 0u64);
    let mut error = use_signal(|| None::<String>);

    let calculate = move |_| {
        match calculate_back_of_envelope(&daily_active_user.read(), &read_write_ratio.read(), &data_size.read()) {
            Ok((rps, wps, storage)) => {
                read_per_second.set(rps);
                write_per_second.set(wps);
                storage_used_per_year.set(storage as u64);
                error.set(None);
            }
            Err(e) => {
                error.set(Some(e));
            }
        }
    };

    let clear = move |_| {
        daily_active_user.set(String::new());
        read_write_ratio.set(String::new());
        data_size.set(String::new());
        read_per_second.set(0.0);
        write_per_second.set(0.0);
        storage_used_per_year.set(0);
        error.set(None);
    };

    let copy_to_clipboard = move |_| {
        let results = format_results(read_per_second(), write_per_second(), storage_used_per_year());
        if !results.is_empty() {
            if let Ok(mut clipboard) = Clipboard::new() {
                let _ = clipboard.set_text(results);
            }
        }
    };

    rsx! {
        div {
            style: "padding: 20px; height: 100%; display: flex; flex-direction: column; box-sizing: border-box; overflow: hidden;",
                
                h1 {
                    style: "font-size: 24px; margin-bottom: 5px; color: #2c3e50;",
                    "Back of the envelope calculations"
                }
                
                div {
                    style: "margin-bottom: 20px;",
                    
                    h3 {
                        style: "font-size: 16px; margin-bottom: 5px; color: #2c3e50;",
                        "Assumptions:"
                    }
                    
                    ul {
                        style: "margin: 0; padding-left: 20px; font-size: 12px; color: #2c3e50;",
                        li { "Assume DAU (Daily Active User)" }
                        li { "Adjust read:write ratio - one of them need to be 1 for based calculation" }
                        li { "Adjust number you want to calculate read/write per seconds" }
                        li { "Assume data size of interest payload" }
                    }
                }
                
                div {
                    style: "display: flex; gap: 20px;",
                    
                    // Left column - inputs
                    div {
                        style: "flex: 1;",
                        
                        div {
                            style: "margin-bottom: 10px;",
                            
                            h3 {
                                style: "font-size: 16px; margin-bottom: 5px; color: #2c3e50;",
                                "Daily Active User"
                            }
                            
                            input {
                                style: "width: 100%; padding: 10px; border: 1px solid #bdc3c7; border-radius: 4px; font-size: 14px;",
                                placeholder: "Daily Active User",
                                value: "{daily_active_user.read()}",
                                oninput: move |event| {
                                    daily_active_user.set(event.value());
                                    error.set(None);
                                }
                            }
                        }
                        
                        div {
                            style: "margin-bottom: 10px;",
                            
                            h3 {
                                style: "font-size: 16px; margin-bottom: 5px; color: #2c3e50;",
                                "Read:Write Ratio"
                            }
                            
                            input {
                                style: "width: 100%; padding: 10px; border: 1px solid #bdc3c7; border-radius: 4px; font-size: 14px;",
                                placeholder: "Read:Write Ratio (e.g., 1:1, 10:1)",
                                value: "{read_write_ratio.read()}",
                                oninput: move |event| {
                                    read_write_ratio.set(event.value());
                                    error.set(None);
                                }
                            }
                        }
                        
                        div {
                            style: "margin-bottom: 10px;",
                            
                            h3 {
                                style: "font-size: 16px; margin-bottom: 5px; color: #2c3e50;",
                                "Data size of interest payload (Byte)"
                            }
                            
                            input {
                                style: "width: 100%; padding: 10px; border: 1px solid #bdc3c7; border-radius: 4px; font-size: 14px;",
                                placeholder: "Data size of interest payload in byte",
                                value: "{data_size.read()}",
                                oninput: move |event| {
                                    data_size.set(event.value());
                                    error.set(None);
                                }
                            }
                        }
                        
                        div {
                            style: "display: flex; gap: 10px;",
                            
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
                    }
                    
                    // Right column - results
                    if read_per_second() != 0.0 || write_per_second() != 0.0 {
                        div {
                            style: "flex: 1;",
                            
                            div {
                                style: "display: flex; align-items: center; gap: 10px; margin-bottom: 10px;",
                                
                                h3 {
                                    style: "font-size: 16px; color: #2c3e50; margin: 0;",
                                    "Results"
                                }
                                
                                button {
                                    style: "padding: 5px 10px; background-color: #34495e; color: white; border: none; border-radius: 4px; cursor: pointer; font-size: 12px;",
                                    onclick: copy_to_clipboard,
                                    "📋 Copy"
                                }
                            }
                            
                            div {
                                style: "margin-bottom: 10px;",
                                p {
                                    style: "font-size: 14px; margin: 2px 0; color: #2c3e50;",
                                    "Read per second"
                                }
                                p {
                                    style: "font-size: 14px; margin: 2px 0; color: #2c3e50; font-weight: bold;",
                                    "{read_per_second():.6} rps"
                                }
                            }
                            
                            div {
                                style: "margin-bottom: 10px;",
                                p {
                                    style: "font-size: 14px; margin: 2px 0; color: #2c3e50;",
                                    "Write per second"
                                }
                                p {
                                    style: "font-size: 14px; margin: 2px 0; color: #2c3e50; font-weight: bold;",
                                    "{write_per_second():.6} tps"
                                }
                            }
                            
                            div {
                                style: "margin-bottom: 10px;",
                                p {
                                    style: "font-size: 14px; margin: 2px 0; color: #2c3e50;",
                                    "Storage used per year (roughly calculated from Write per second)"
                                }
                                p {
                                    style: "font-size: 12px; margin: 1px 0; color: #2c3e50;",
                                    "{storage_used_per_year()} Byte"
                                }
                                p {
                                    style: "font-size: 12px; margin: 1px 0; color: #2c3e50;",
                                    "{storage_used_per_year() / 1024} KB"
                                }
                                p {
                                    style: "font-size: 12px; margin: 1px 0; color: #2c3e50;",
                                    "{storage_used_per_year() / 1024 / 1024} MB"
                                }
                                p {
                                    style: "font-size: 12px; margin: 1px 0; color: #2c3e50;",
                                    "{storage_used_per_year() / 1024 / 1024 / 1024} GB"
                                }
                                p {
                                    style: "font-size: 12px; margin: 1px 0; color: #2c3e50;",
                                    "{storage_used_per_year() / 1024 / 1024 / 1024 / 1024} TB"
                                }
                                p {
                                    style: "font-size: 12px; margin: 1px 0; color: #2c3e50;",
                                    "{storage_used_per_year() / 1024 / 1024 / 1024 / 1024 / 1024} PB"
                                }
                            }
                            
                            p {
                                style: "font-size: 14px; color: #2c3e50; margin-top: 10px;",
                                "The rest, Sum/Multiply them by yourself, you already got foundation value"
                            }
                        }
                    }
                }
                
            // Error message
            if let Some(err) = error.read().as_ref() {
                div {
                    style: "margin-top: 20px; padding: 10px; background-color: #ffebee; border: 1px solid #f44336; border-radius: 4px; color: #c62828; font-size: 14px;",
                    "{err}"
                }
            }
        }
    }
}