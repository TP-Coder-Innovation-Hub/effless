use arboard::Clipboard;
use base64::{Engine as _, engine::general_purpose};
use iced::{
    Element, Length,
    widget::{button, column, container, pick_list, row, text, text_input, Image},
};
use image::{ImageBuffer, Rgb, RgbImage};
use chrono;

#[derive(Debug, Clone)]
pub enum Message {
    TextChanged(String),
    ShapeChanged(Shape),
    SizeChanged(String),
    BackgroundColorChanged(String),
    TextColorChanged(String),
    GenerateIcon,
    CopyToClipboard,
    DownloadIcon(String), // Parameter is the file format: "png" or "ico"
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Shape {
    Circle,
    Square,
}

impl std::fmt::Display for Shape {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Shape::Circle => write!(f, "Circle"),
            Shape::Square => write!(f, "Square"),
        }
    }
}

pub struct IconTool {
    text: String,
    shape: Shape,
    size: String,
    background_color: String,
    text_color: String,
    generated_icon: Option<Vec<u8>>,
    icon_handle: Option<iced::advanced::image::Handle>,
    status_message: String,
}

impl IconTool {
    pub fn new() -> Self {
        Self {
            text: String::new(),
            shape: Shape::Circle,
            size: "128".to_string(),
            background_color: "#3B82F6".to_string(),
            text_color: "#FFFFFF".to_string(),
            generated_icon: None,
            icon_handle: None,
            status_message: String::new(),
        }
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::TextChanged(value) => {
                // Limit to 3 characters maximum
                self.text = value.chars().take(3).collect();
            }
            Message::ShapeChanged(shape) => {
                self.shape = shape;
            }
            Message::SizeChanged(value) => {
                self.size = value;
            }
            Message::BackgroundColorChanged(value) => {
                self.background_color = value;
            }
            Message::TextColorChanged(value) => {
                self.text_color = value;
            }
            Message::GenerateIcon => {
                self.generate_icon();
            }
            Message::CopyToClipboard => {
                self.copy_to_clipboard();
            }
            Message::DownloadIcon(format) => {
                self.download_icon(&format);
            }
        }
    }

    fn generate_icon(&mut self) {
        // Reset previous state
        self.icon_handle = None;
        self.generated_icon = None;
        
        if self.text.trim().is_empty() {
            self.status_message = "Please enter text for the icon".to_string();
            return;
        }

        let size = match self.size.parse::<u32>() {
            Ok(s) if s >= 16 && s <= 1024 => s,
            _ => {
                self.status_message = "Size must be a number between 16 and 1024".to_string();
                return;
            }
        };

        let bg_color = match self.parse_hex_color(&self.background_color) {
            Ok(color) => color,
            Err(e) => {
                self.status_message = format!("Invalid background color: {e}");
                return;
            }
        };

        let text_color = match self.parse_hex_color(&self.text_color) {
            Ok(color) => color,
            Err(e) => {
                self.status_message = format!("Invalid text color: {e}");
                return;
            }
        };

        // Additional validation
        if bg_color == text_color {
            self.status_message = "Background and text colors should be different for better visibility".to_string();
            return;
        }

        // Wrap entire icon generation in panic handler
        let generation_result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            self.create_icon_image(size, bg_color, text_color)
        }));

        match generation_result {
            Ok(Ok(image_data)) => {
                if image_data.is_empty() {
                    self.status_message = "Failed to generate icon: Empty image data".to_string();
                    return;
                }
                
                // Create image handle for preview with error handling
                match std::panic::catch_unwind(|| {
                    iced::advanced::image::Handle::from_bytes(image_data.clone())
                }) {
                    Ok(handle) => {
                        self.icon_handle = Some(handle);
                        self.generated_icon = Some(image_data);
                        self.status_message = "Icon generated successfully!".to_string();
                    }
                    Err(_) => {
                        self.status_message = "Failed to create image preview".to_string();
                        // Still save the raw data for download
                        self.generated_icon = Some(image_data);
                    }
                }
            }
            Ok(Err(e)) => {
                self.status_message = format!("Failed to generate icon: {e}");
                self.icon_handle = None;
                self.generated_icon = None;
            }
            Err(_) => {
                self.status_message = "Icon generation failed due to internal error. Try different settings.".to_string();
                self.icon_handle = None;
                self.generated_icon = None;
            }
        }
    }

    fn parse_hex_color(&self, hex: &str) -> Result<(u8, u8, u8), String> {
        let hex = hex.trim_start_matches('#');
        if hex.len() != 6 {
            return Err("Color must be in #RRGGBB format".to_string());
        }

        let r = u8::from_str_radix(&hex[0..2], 16).map_err(|_| "Invalid red component")?;
        let g = u8::from_str_radix(&hex[2..4], 16).map_err(|_| "Invalid green component")?;
        let b = u8::from_str_radix(&hex[4..6], 16).map_err(|_| "Invalid blue component")?;

        Ok((r, g, b))
    }

    fn create_icon_image(
        &self,
        size: u32,
        bg_color: (u8, u8, u8),
        text_color: (u8, u8, u8),
    ) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        if size == 0 || size > 2048 {
            return Err("Invalid image size".into());
        }
        
        let mut img: RgbImage = match ImageBuffer::new(size, size) {
            img if img.width() == size && img.height() == size => img,
            _ => return Err("Failed to create image buffer".into()),
        };

        let center = size as f32 / 2.0;
        let radius = (center - 2.0).max(1.0); // Ensure positive radius

        for (x, y, pixel) in img.enumerate_pixels_mut() {
            let should_color = match self.shape {
                Shape::Circle => {
                    let dx = x as f32 - center;
                    let dy = y as f32 - center;
                    dx * dx + dy * dy <= radius * radius
                }
                Shape::Square => true,
            };

            if should_color {
                *pixel = Rgb([bg_color.0, bg_color.1, bg_color.2]);
            } else {
                *pixel = Rgb([255, 255, 255]); // White background for transparency
            }
        }

        let text_chars: Vec<char> = self.text.chars().collect();
        let display_text = if text_chars.len() > 3 {
            text_chars
                .into_iter()
                .take(3)
                .collect::<String>()
                .to_uppercase()
        } else {
            self.text.to_uppercase()
        };
        
        // Dynamic font sizing based on character count and icon size
        let char_count = display_text.len() as u32;
        let base_font_size = match char_count {
            1 => (size as f32 * 0.6) as u32,  // Larger for single character
            2 => (size as f32 * 0.45) as u32, // Medium for two characters  
            3 => (size as f32 * 0.35) as u32, // Smaller for three characters
            _ => (size as f32 * 0.3) as u32,  // Fallback
        };
        let font_size = std::cmp::max(8, base_font_size);

        if !display_text.is_empty() {
            // Calculate text dimensions for proper centering
            let char_count = display_text.len() as u32;
            
            // Character width is based on our 5-pixel wide bitmap patterns
            let char_pixel_width = 5u32.saturating_mul(std::cmp::max(1, font_size / 8));
            let char_spacing = match char_count {
                1 => 0,
                2 => std::cmp::max(char_pixel_width / 2, 6),  // Better spacing for readability
                3 => std::cmp::max(char_pixel_width / 3, 4),  // Tighter spacing for 3 chars  
                _ => std::cmp::max(char_pixel_width / 4, 3),
            };
            
            // Calculate total width including spacing
            let total_text_width = (char_count * char_pixel_width) + ((char_count.saturating_sub(1)) * char_spacing);
            
            // For circles, use a smaller effective area (85% of size)
            let effective_size = match self.shape {
                Shape::Circle => (size as f32 * 0.85) as u32,
                Shape::Square => (size as f32 * 0.9) as u32,
            };
            
            let text_x = if total_text_width < effective_size {
                (size.saturating_sub(total_text_width)) / 2
            } else {
                // If text is too wide, position it at a safe margin
                std::cmp::max(size / 10, 2)
            };
            
            // Vertical centering - adjust for our 7-row character patterns
            let char_pattern_height = 7; // Our character patterns are 7 rows
            let pixel_size = std::cmp::max(1, std::cmp::min(font_size / 8, 8));
            let total_char_height = char_pattern_height * pixel_size;
            let text_y = if total_char_height < size {
                (size.saturating_sub(total_char_height)) / 2
            } else {
                std::cmp::max(size / 8, 2)  // Safe fallback positioning
            };

            self.draw_simple_text(
                &mut img,
                &display_text,
                text_x,
                text_y,
                font_size,
                text_color,
            )?;
        }

        let mut buffer = Vec::new();
        {
            let mut cursor = std::io::Cursor::new(&mut buffer);
            img.write_to(&mut cursor, image::ImageFormat::Png)?;
        }

        Ok(buffer)
    }

    fn draw_simple_text(
        &self,
        img: &mut RgbImage,
        text: &str,
        start_x: u32,
        start_y: u32,
        size: u32,
        color: (u8, u8, u8),
    ) -> Result<(), Box<dyn std::error::Error>> {
        let pixel_size = std::cmp::max(1, std::cmp::min(size / 8, 8)); // Limit pixel size to prevent overflow
        
        if text.is_empty() {
            return Ok(());
        }

        let char_count = text.len() as u32;
        // Character width based on our 5-pixel bitmap patterns
        let char_pixel_width = 5u32.saturating_mul(pixel_size);
        let char_spacing = match char_count {
            1 => 0,
            2 => std::cmp::max(char_pixel_width / 2, 6),  // Better spacing for readability
            3 => std::cmp::max(char_pixel_width / 3, 4),  // Tighter spacing for 3 chars
            _ => std::cmp::max(char_pixel_width / 4, 3),
        };

        for (char_idx, ch) in text.chars().enumerate() {
            // Calculate character position with safer arithmetic
            let char_offset = (char_idx as u32).saturating_mul(char_pixel_width.saturating_add(char_spacing));
            let char_x = start_x.saturating_add(char_offset);
            
            // Skip if character would be completely outside image bounds
            if char_x >= img.width() || start_y >= img.height() {
                break;
            }

            let pattern = self.get_char_pattern(ch);
            if pattern.is_empty() {
                continue;
            }

            for (row, line) in pattern.iter().enumerate() {
                if line.is_empty() {
                    continue;
                }
                
                for (col, &bit) in line.iter().enumerate() {
                    if bit == 1 {
                        // Calculate the pixel block position with safer arithmetic
                        let col_offset = (col as u32).saturating_mul(pixel_size);
                        let row_offset = (row as u32).saturating_mul(pixel_size);
                        let block_x = char_x.saturating_add(col_offset);
                        let block_y = start_y.saturating_add(row_offset);
                        
                        // Skip if block is completely outside bounds
                        if block_x >= img.width() || block_y >= img.height() {
                            continue;
                        }
                        
                        // Draw the pixel block with strict bounds checking
                        for py in 0..pixel_size {
                            for px in 0..pixel_size {
                                let x = block_x.saturating_add(px);
                                let y = block_y.saturating_add(py);

                                // Double-check bounds before pixel access
                                if x < img.width() && y < img.height() {
                                    // Safely set pixel with panic protection
                                    if let Some(pixel) = img.get_pixel_mut_checked(x, y) {
                                        *pixel = Rgb([color.0, color.1, color.2]);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        Ok(())
    }

    fn get_char_pattern(&self, ch: char) -> Vec<Vec<u8>> {
        match ch {
            'A' => vec![
                vec![0, 1, 1, 1, 0],
                vec![1, 0, 0, 0, 1],
                vec![1, 0, 0, 0, 1],
                vec![1, 1, 1, 1, 1],
                vec![1, 0, 0, 0, 1],
                vec![1, 0, 0, 0, 1],
                vec![1, 0, 0, 0, 1],
            ],
            'B' => vec![
                vec![1, 1, 1, 1, 0],
                vec![1, 0, 0, 0, 1],
                vec![1, 0, 0, 0, 1],
                vec![1, 1, 1, 1, 0],
                vec![1, 0, 0, 0, 1],
                vec![1, 0, 0, 0, 1],
                vec![1, 1, 1, 1, 0],
            ],
            'C' => vec![
                vec![0, 1, 1, 1, 0],
                vec![1, 0, 0, 0, 1],
                vec![1, 0, 0, 0, 0],
                vec![1, 0, 0, 0, 0],
                vec![1, 0, 0, 0, 0],
                vec![1, 0, 0, 0, 1],
                vec![0, 1, 1, 1, 0],
            ],
            'D' => vec![
                vec![1, 1, 1, 1, 0],
                vec![1, 0, 0, 0, 1],
                vec![1, 0, 0, 0, 1],
                vec![1, 0, 0, 0, 1],
                vec![1, 0, 0, 0, 1],
                vec![1, 0, 0, 0, 1],
                vec![1, 1, 1, 1, 0],
            ],
            'E' => vec![
                vec![1, 1, 1, 1, 1],
                vec![1, 0, 0, 0, 0],
                vec![1, 0, 0, 0, 0],
                vec![1, 1, 1, 1, 0],
                vec![1, 0, 0, 0, 0],
                vec![1, 0, 0, 0, 0],
                vec![1, 1, 1, 1, 1],
            ],
            'F' => vec![
                vec![1, 1, 1, 1, 1],
                vec![1, 0, 0, 0, 0],
                vec![1, 0, 0, 0, 0],
                vec![1, 1, 1, 1, 0],
                vec![1, 0, 0, 0, 0],
                vec![1, 0, 0, 0, 0],
                vec![1, 0, 0, 0, 0],
            ],
            'G' => vec![
                vec![0, 1, 1, 1, 0],
                vec![1, 0, 0, 0, 1],
                vec![1, 0, 0, 0, 0],
                vec![1, 0, 1, 1, 1],
                vec![1, 0, 0, 0, 1],
                vec![1, 0, 0, 0, 1],
                vec![0, 1, 1, 1, 0],
            ],
            'H' => vec![
                vec![1, 0, 0, 0, 1],
                vec![1, 0, 0, 0, 1],
                vec![1, 0, 0, 0, 1],
                vec![1, 1, 1, 1, 1],
                vec![1, 0, 0, 0, 1],
                vec![1, 0, 0, 0, 1],
                vec![1, 0, 0, 0, 1],
            ],
            'I' => vec![
                vec![1, 1, 1, 1, 1],
                vec![0, 0, 1, 0, 0],
                vec![0, 0, 1, 0, 0],
                vec![0, 0, 1, 0, 0],
                vec![0, 0, 1, 0, 0],
                vec![0, 0, 1, 0, 0],
                vec![1, 1, 1, 1, 1],
            ],
            'J' => vec![
                vec![0, 0, 0, 0, 1],
                vec![0, 0, 0, 0, 1],
                vec![0, 0, 0, 0, 1],
                vec![0, 0, 0, 0, 1],
                vec![1, 0, 0, 0, 1],
                vec![1, 0, 0, 0, 1],
                vec![0, 1, 1, 1, 0],
            ],
            'K' => vec![
                vec![1, 0, 0, 0, 1],
                vec![1, 0, 0, 1, 0],
                vec![1, 0, 1, 0, 0],
                vec![1, 1, 0, 0, 0],
                vec![1, 0, 1, 0, 0],
                vec![1, 0, 0, 1, 0],
                vec![1, 0, 0, 0, 1],
            ],
            'L' => vec![
                vec![1, 0, 0, 0, 0],
                vec![1, 0, 0, 0, 0],
                vec![1, 0, 0, 0, 0],
                vec![1, 0, 0, 0, 0],
                vec![1, 0, 0, 0, 0],
                vec![1, 0, 0, 0, 0],
                vec![1, 1, 1, 1, 1],
            ],
            'M' => vec![
                vec![1, 0, 0, 0, 1],
                vec![1, 1, 0, 1, 1],
                vec![1, 0, 1, 0, 1],
                vec![1, 0, 0, 0, 1],
                vec![1, 0, 0, 0, 1],
                vec![1, 0, 0, 0, 1],
                vec![1, 0, 0, 0, 1],
            ],
            'N' => vec![
                vec![1, 0, 0, 0, 1],
                vec![1, 1, 0, 0, 1],
                vec![1, 0, 1, 0, 1],
                vec![1, 0, 0, 1, 1],
                vec![1, 0, 0, 0, 1],
                vec![1, 0, 0, 0, 1],
                vec![1, 0, 0, 0, 1],
            ],
            'O' => vec![
                vec![0, 1, 1, 1, 0],
                vec![1, 0, 0, 0, 1],
                vec![1, 0, 0, 0, 1],
                vec![1, 0, 0, 0, 1],
                vec![1, 0, 0, 0, 1],
                vec![1, 0, 0, 0, 1],
                vec![0, 1, 1, 1, 0],
            ],
            'P' => vec![
                vec![1, 1, 1, 1, 0],
                vec![1, 0, 0, 0, 1],
                vec![1, 0, 0, 0, 1],
                vec![1, 1, 1, 1, 0],
                vec![1, 0, 0, 0, 0],
                vec![1, 0, 0, 0, 0],
                vec![1, 0, 0, 0, 0],
            ],
            'Q' => vec![
                vec![0, 1, 1, 1, 0],
                vec![1, 0, 0, 0, 1],
                vec![1, 0, 0, 0, 1],
                vec![1, 0, 0, 0, 1],
                vec![1, 0, 1, 0, 1],
                vec![1, 0, 0, 1, 0],
                vec![0, 1, 1, 0, 1],
            ],
            'R' => vec![
                vec![1, 1, 1, 1, 0],
                vec![1, 0, 0, 0, 1],
                vec![1, 0, 0, 0, 1],
                vec![1, 1, 1, 1, 0],
                vec![1, 0, 1, 0, 0],
                vec![1, 0, 0, 1, 0],
                vec![1, 0, 0, 0, 1],
            ],
            'S' => vec![
                vec![0, 1, 1, 1, 0],
                vec![1, 0, 0, 0, 1],
                vec![1, 0, 0, 0, 0],
                vec![0, 1, 1, 1, 0],
                vec![0, 0, 0, 0, 1],
                vec![1, 0, 0, 0, 1],
                vec![0, 1, 1, 1, 0],
            ],
            'T' => vec![
                vec![1, 1, 1, 1, 1],
                vec![0, 0, 1, 0, 0],
                vec![0, 0, 1, 0, 0],
                vec![0, 0, 1, 0, 0],
                vec![0, 0, 1, 0, 0],
                vec![0, 0, 1, 0, 0],
                vec![0, 0, 1, 0, 0],
            ],
            'U' => vec![
                vec![1, 0, 0, 0, 1],
                vec![1, 0, 0, 0, 1],
                vec![1, 0, 0, 0, 1],
                vec![1, 0, 0, 0, 1],
                vec![1, 0, 0, 0, 1],
                vec![1, 0, 0, 0, 1],
                vec![0, 1, 1, 1, 0],
            ],
            'V' => vec![
                vec![1, 0, 0, 0, 1],
                vec![1, 0, 0, 0, 1],
                vec![1, 0, 0, 0, 1],
                vec![1, 0, 0, 0, 1],
                vec![0, 1, 0, 1, 0],
                vec![0, 1, 0, 1, 0],
                vec![0, 0, 1, 0, 0],
            ],
            'W' => vec![
                vec![1, 0, 0, 0, 1],
                vec![1, 0, 0, 0, 1],
                vec![1, 0, 0, 0, 1],
                vec![1, 0, 1, 0, 1],
                vec![1, 0, 1, 0, 1],
                vec![1, 1, 0, 1, 1],
                vec![1, 0, 0, 0, 1],
            ],
            'X' => vec![
                vec![1, 0, 0, 0, 1],
                vec![0, 1, 0, 1, 0],
                vec![0, 0, 1, 0, 0],
                vec![0, 0, 1, 0, 0],
                vec![0, 0, 1, 0, 0],
                vec![0, 1, 0, 1, 0],
                vec![1, 0, 0, 0, 1],
            ],
            'Y' => vec![
                vec![1, 0, 0, 0, 1],
                vec![0, 1, 0, 1, 0],
                vec![0, 0, 1, 0, 0],
                vec![0, 0, 1, 0, 0],
                vec![0, 0, 1, 0, 0],
                vec![0, 0, 1, 0, 0],
                vec![0, 0, 1, 0, 0],
            ],
            'Z' => vec![
                vec![1, 1, 1, 1, 1],
                vec![0, 0, 0, 0, 1],
                vec![0, 0, 0, 1, 0],
                vec![0, 0, 1, 0, 0],
                vec![0, 1, 0, 0, 0],
                vec![1, 0, 0, 0, 0],
                vec![1, 1, 1, 1, 1],
            ],
            '0' => vec![
                vec![0, 1, 1, 1, 0],
                vec![1, 0, 0, 0, 1],
                vec![1, 0, 0, 1, 1],
                vec![1, 0, 1, 0, 1],
                vec![1, 1, 0, 0, 1],
                vec![1, 0, 0, 0, 1],
                vec![0, 1, 1, 1, 0],
            ],
            '1' => vec![
                vec![0, 0, 1, 0, 0],
                vec![0, 1, 1, 0, 0],
                vec![0, 0, 1, 0, 0],
                vec![0, 0, 1, 0, 0],
                vec![0, 0, 1, 0, 0],
                vec![0, 0, 1, 0, 0],
                vec![1, 1, 1, 1, 1],
            ],
            '2' => vec![
                vec![0, 1, 1, 1, 0],
                vec![1, 0, 0, 0, 1],
                vec![0, 0, 0, 0, 1],
                vec![0, 0, 0, 1, 0],
                vec![0, 0, 1, 0, 0],
                vec![0, 1, 0, 0, 0],
                vec![1, 1, 1, 1, 1],
            ],
            '3' => vec![
                vec![0, 1, 1, 1, 0],
                vec![1, 0, 0, 0, 1],
                vec![0, 0, 0, 0, 1],
                vec![0, 0, 1, 1, 0],
                vec![0, 0, 0, 0, 1],
                vec![1, 0, 0, 0, 1],
                vec![0, 1, 1, 1, 0],
            ],
            '4' => vec![
                vec![0, 0, 0, 1, 0],
                vec![0, 0, 1, 1, 0],
                vec![0, 1, 0, 1, 0],
                vec![1, 0, 0, 1, 0],
                vec![1, 1, 1, 1, 1],
                vec![0, 0, 0, 1, 0],
                vec![0, 0, 0, 1, 0],
            ],
            '5' => vec![
                vec![1, 1, 1, 1, 1],
                vec![1, 0, 0, 0, 0],
                vec![1, 1, 1, 1, 0],
                vec![0, 0, 0, 0, 1],
                vec![0, 0, 0, 0, 1],
                vec![1, 0, 0, 0, 1],
                vec![0, 1, 1, 1, 0],
            ],
            '6' => vec![
                vec![0, 0, 1, 1, 0],
                vec![0, 1, 0, 0, 0],
                vec![1, 0, 0, 0, 0],
                vec![1, 1, 1, 1, 0],
                vec![1, 0, 0, 0, 1],
                vec![1, 0, 0, 0, 1],
                vec![0, 1, 1, 1, 0],
            ],
            '7' => vec![
                vec![1, 1, 1, 1, 1],
                vec![0, 0, 0, 0, 1],
                vec![0, 0, 0, 1, 0],
                vec![0, 0, 1, 0, 0],
                vec![0, 1, 0, 0, 0],
                vec![0, 1, 0, 0, 0],
                vec![0, 1, 0, 0, 0],
            ],
            '8' => vec![
                vec![0, 1, 1, 1, 0],
                vec![1, 0, 0, 0, 1],
                vec![1, 0, 0, 0, 1],
                vec![0, 1, 1, 1, 0],
                vec![1, 0, 0, 0, 1],
                vec![1, 0, 0, 0, 1],
                vec![0, 1, 1, 1, 0],
            ],
            '9' => vec![
                vec![0, 1, 1, 1, 0],
                vec![1, 0, 0, 0, 1],
                vec![1, 0, 0, 0, 1],
                vec![0, 1, 1, 1, 1],
                vec![0, 0, 0, 0, 1],
                vec![0, 0, 0, 1, 0],
                vec![0, 1, 1, 0, 0],
            ],
            _ => vec![
                vec![0, 1, 1, 1, 0],
                vec![1, 0, 0, 0, 1],
                vec![1, 0, 0, 1, 1],
                vec![1, 0, 1, 0, 1],
                vec![1, 1, 0, 0, 1],
                vec![1, 0, 0, 0, 1],
                vec![0, 1, 1, 1, 0],
            ],
        }
    }

    fn copy_to_clipboard(&mut self) {
        if let Some(ref icon_data) = self.generated_icon {
            match Clipboard::new() {
                Ok(mut clipboard) => {
                    let base64_data = general_purpose::STANDARD.encode(icon_data);
                    match clipboard.set_text(base64_data) {
                        Ok(_) => {
                            self.status_message =
                                "Base64 icon data copied to clipboard!".to_string()
                        }
                        Err(_) => self.status_message = "Failed to copy to clipboard".to_string(),
                    }
                }
                Err(_) => self.status_message = "Failed to access clipboard".to_string(),
            }
        } else {
            self.status_message = "No icon to copy. Generate an icon first.".to_string();
        }
    }

    fn download_icon(&mut self, format: &str) {
        if let Some(ref icon_data) = self.generated_icon {
            let icon_data = icon_data.clone();
            let text = self.text.clone();
            let format = format.to_string();
            let size = self.size.clone();

            // Use blocking call directly without spawning async task
            let file_name = if text.is_empty() {
                format!("defaultName.{}", format)
            } else {
                let base_name = "img_generated";
                let current_date = chrono::Local::now();
                format!("{}_{}.{}", base_name, current_date, format)
            };

            let filter_name = match format.as_str() {
                "ico" => "ICO Icon",
                _ => "PNG Image",
            };
            let filter_ext = &[format.as_str()];

            if let Some(path) = rfd::FileDialog::new()
                .set_file_name(&file_name)
                .add_filter(filter_name, filter_ext)
                .save_file()
            {
                let result = if format == "ico" {
                    // Convert PNG data to ICO format
                    Self::convert_png_to_ico(&icon_data, &path, &size)
                } else {
                    // Save as PNG directly
                    std::fs::write(&path, &icon_data)
                        .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
                };

                match result {
                    Ok(_) => {
                        self.status_message = format!("Icon saved successfully as {}!", format.to_uppercase());
                    },
                    Err(e) => {
                        self.status_message = format!("Failed to save icon: {}", e);
                    },
                }
            } else {
                self.status_message = "File save dialog was cancelled".to_string();
            }
        } else {
            self.status_message = "No icon to download. Generate an icon first.".to_string();
        }
    }

    fn convert_png_to_ico(png_data: &[u8], path: &std::path::Path, size_str: &str) -> Result<(), Box<dyn std::error::Error>> {
        // Parse the PNG image
        let img = image::load_from_memory(png_data)?;
        let size: u32 = size_str.parse().unwrap_or(128);
        let rgba_img = img.resize_exact(size, size, image::imageops::FilterType::Lanczos3).to_rgba8();
        
        // Create ICO file
        let icon_image = ico::IconImage::from_rgba_data(size, size, rgba_img.into_raw());
        let icon_dir = ico::IconDir::new(ico::ResourceType::Icon);
        let mut icon_dir = icon_dir;
        icon_dir.add_entry(ico::IconDirEntry::encode(&icon_image)?);
        
        // Write ICO file
        let file = std::fs::File::create(path)?;
        icon_dir.write(file)?;
        
        Ok(())
    }

    pub fn view(&self) -> Element<Message> {
        let shapes = vec![Shape::Circle, Shape::Square];

        let controls = column![
            text("Text:").size(16),
            text_input("Enter text (max 3 chars)", &self.text)
                .on_input(Message::TextChanged)
                .padding(10),
            text("Shape:").size(16),
            pick_list(shapes, Some(self.shape.clone()), Message::ShapeChanged).padding(10),
            text("Size (pixels):").size(16),
            text_input("128", &self.size)
                .on_input(Message::SizeChanged)
                .padding(10),
            text("Background Color (#RRGGBB):").size(16),
            text_input("#3B82F6", &self.background_color)
                .on_input(Message::BackgroundColorChanged)
                .padding(10),
            text("Text Color (#RRGGBB):").size(16),
            text_input("#FFFFFF", &self.text_color)
                .on_input(Message::TextColorChanged)
                .padding(10),
            row![
                button(text("Generate Icon"))
                    .on_press(Message::GenerateIcon)
                    .padding(10),
                button(text("Copy Base64"))
                    .on_press(Message::CopyToClipboard)
                    .padding(10),
            ]
            .spacing(10),
            text(&self.status_message).size(14),
        ]
        .spacing(10);

        // Create preview section if icon is generated
        let preview_section = if let Some(ref handle) = self.icon_handle {
            column![
                text("Preview:").size(16),
                // Display the actual icon image
                container(
                    Image::new(handle.clone())
                        .width(Length::Fixed(128.0))
                        .height(Length::Fixed(128.0))
                )
                .style(container::rounded_box)
                .padding(10)
                .center_x(Length::Fill),
                // Download buttons
                column![
                    text("Download as:").size(14),
                    row![
                        button(text("Download PNG"))
                            .on_press(Message::DownloadIcon("png".to_string()))
                            .padding([5, 10])
                            .style(button::primary),
                        button(text("Download ICO"))
                            .on_press(Message::DownloadIcon("ico".to_string()))
                            .padding([5, 10])
                            .style(button::secondary),
                    ]
                    .spacing(10)
                    .align_y(iced::Alignment::Center),
                ]
                .spacing(5)
                .align_x(iced::Alignment::Center),
            ]
            .spacing(10)
            .align_x(iced::Alignment::Center)
        } else {
            column![
                text("Preview:").size(16),
                container(
                    text("Generate an icon to see preview")
                        .size(12)
                        .style(|_theme| iced::widget::text::Style {
                            color: Some(iced::Color::from_rgb(0.6, 0.6, 0.6))
                        })
                )
                .style(container::rounded_box)
                .padding(20)
                .width(Length::Fixed(200.0))
                .height(Length::Fixed(148.0))
                .center_x(Length::Fill)
                .center_y(Length::Fill),
            ]
            .spacing(10)
        };

        let content = row![
            container(controls).width(Length::FillPortion(1)).padding(20),
            container(preview_section).width(Length::FillPortion(1)).padding(20),
        ]
        .spacing(20);

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}
