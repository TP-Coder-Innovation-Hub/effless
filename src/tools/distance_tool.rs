use iced::{
    widget::{button, column, container, row, text, text_input, Column},
    Element, Length,
};

#[derive(Debug, Clone)]
pub enum Message {
    Lat1Changed(String),
    Lon1Changed(String),
    Lat2Changed(String),
    Lon2Changed(String),
    Calculate,
    Clear,
}

#[derive(Default)]
pub struct DistanceTool {
    lat1: String,
    lon1: String,
    lat2: String,
    lon2: String,
    result: String,
    error: Option<String>,
}

impl DistanceTool {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::Lat1Changed(value) => {
                self.lat1 = value;
                self.error = None;
            }
            Message::Lon1Changed(value) => {
                self.lon1 = value;
                self.error = None;
            }
            Message::Lat2Changed(value) => {
                self.lat2 = value;
                self.error = None;
            }
            Message::Lon2Changed(value) => {
                self.lon2 = value;
                self.error = None;
            }
            Message::Calculate => {
                match self.calculate_distance() {
                    Ok(distance) => {
                        self.result = format!("{:.2} km ({:.2} miles)", distance, distance * 0.621371);
                        self.error = None;
                    }
                    Err(e) => {
                        self.error = Some(e);
                        self.result.clear();
                    }
                }
            }
            Message::Clear => {
                self.lat1.clear();
                self.lon1.clear();
                self.lat2.clear();
                self.lon2.clear();
                self.result.clear();
                self.error = None;
            }
        }
    }

    fn calculate_distance(&self) -> Result<f64, String> {
        let lat1: f64 = self.lat1.parse().map_err(|_| "Invalid latitude 1")?;
        let lon1: f64 = self.lon1.parse().map_err(|_| "Invalid longitude 1")?;
        let lat2: f64 = self.lat2.parse().map_err(|_| "Invalid latitude 2")?;
        let lon2: f64 = self.lon2.parse().map_err(|_| "Invalid longitude 2")?;

        // Haversine formula
        let r = 6371.0; // Earth's radius in km
        let dlat = (lat2 - lat1).to_radians();
        let dlon = (lon2 - lon1).to_radians();
        let a = (dlat / 2.0).sin().powi(2) 
            + lat1.to_radians().cos() * lat2.to_radians().cos() * (dlon / 2.0).sin().powi(2);
        let c = 2.0 * a.sqrt().asin();
        
        Ok(r * c)
    }

    pub fn view(&self) -> Element<Message> {
        let point1_section = column![
            text("Point 1").size(16),
            row![
                text_input("Latitude", &self.lat1)
                    .on_input(Message::Lat1Changed)
                    .size(14)
                    .padding(10),
                text_input("Longitude", &self.lon1)
                    .on_input(Message::Lon1Changed)
                    .size(14)
                    .padding(10),
            ]
            .spacing(10),
        ]
        .spacing(5);

        let point2_section = column![
            text("Point 2").size(16),
            row![
                text_input("Latitude", &self.lat2)
                    .on_input(Message::Lat2Changed)
                    .size(14)
                    .padding(10),
                text_input("Longitude", &self.lon2)
                    .on_input(Message::Lon2Changed)
                    .size(14)
                    .padding(10),
            ]
            .spacing(10),
        ]
        .spacing(5);

        let buttons = row![
            button(text("Calculate").size(14))
                .on_press(Message::Calculate)
                .padding(10),
            button(text("Clear").size(14))
                .on_press(Message::Clear)
                .padding(10),
        ]
        .spacing(10);

        let result_section = if !self.result.is_empty() {
            column![
                text("Distance").size(16),
                container(
                    text_input("", &self.result)
                        .size(14)
                )
                .style(iced::theme::Container::Box)
                .padding(10)
                .width(Length::Fill),
            ]
            .spacing(5)
        } else {
            column![]
        };

        let mut content = Column::new()
            .spacing(20)
            .push(text("Haversine Distance Calculator").size(24))
            .push(text("Calculate the great-circle distance between two points on Earth").size(14))
            .push(point1_section)
            .push(point2_section)
            .push(buttons)
            .push(result_section);

        if let Some(error) = &self.error {
            content = content.push(
                text(error)
                    .size(14)
                    .style(iced::theme::Text::Color(iced::Color::from_rgb(0.8, 0.2, 0.2)))
            );
        }

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(20)
            .into()
    }
}