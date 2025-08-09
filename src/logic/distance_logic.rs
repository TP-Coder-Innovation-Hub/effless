use std::f64::consts::PI;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Coordinates {
    pub lat: f64,
    pub lon: f64,
}

#[derive(Debug, PartialEq)]
pub struct Distance {
    pub km: f64,
    pub miles: f64,
}

#[derive(Debug, PartialEq)]
pub enum DistanceError {
    InvalidLatitude(f64),
    InvalidLongitude(f64),
}

pub struct DistanceLogic;

impl DistanceLogic {
    const EARTH_RADIUS_KM: f64 = 6371.0;
    const KM_TO_MILES: f64 = 0.621371;
    
    /// Validate coordinates
    pub fn validate_coordinates(lat: f64, lon: f64) -> Result<Coordinates, DistanceError> {
        if lat < -90.0 || lat > 90.0 {
            return Err(DistanceError::InvalidLatitude(lat));
        }
        if lon < -180.0 || lon > 180.0 {
            return Err(DistanceError::InvalidLongitude(lon));
        }
        Ok(Coordinates { lat, lon })
    }
    
    /// Calculate Haversine distance between two points
    pub fn calculate_distance(point1: Coordinates, point2: Coordinates) -> Distance {
        let lat1_rad = point1.lat * PI / 180.0;
        let lat2_rad = point2.lat * PI / 180.0;
        let delta_lat = (point2.lat - point1.lat) * PI / 180.0;
        let delta_lon = (point2.lon - point1.lon) * PI / 180.0;

        let a = (delta_lat / 2.0).sin().powi(2) +
                lat1_rad.cos() * lat2_rad.cos() *
                (delta_lon / 2.0).sin().powi(2);
                
        let c = 2.0 * a.sqrt().atan2((1.0 - a).sqrt());
        let km = Self::EARTH_RADIUS_KM * c;
        let miles = km * Self::KM_TO_MILES;
        
        Distance { km, miles }
    }
    
    /// Calculate distance with validation
    pub fn calculate_with_validation(lat1: f64, lon1: f64, lat2: f64, lon2: f64) 
        -> Result<Distance, DistanceError> {
        let point1 = Self::validate_coordinates(lat1, lon1)?;
        let point2 = Self::validate_coordinates(lat2, lon2)?;
        Ok(Self::calculate_distance(point1, point2))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_valid_coordinates() {
        let result = DistanceLogic::validate_coordinates(40.7128, -74.0060);
        assert!(result.is_ok());
        let coords = result.unwrap();
        assert_eq!(coords.lat, 40.7128);
        assert_eq!(coords.lon, -74.0060);
    }

    #[test]
    fn test_validate_invalid_latitude() {
        let result = DistanceLogic::validate_coordinates(91.0, -74.0060);
        assert_eq!(result, Err(DistanceError::InvalidLatitude(91.0)));
    }

    #[test]
    fn test_validate_invalid_longitude() {
        let result = DistanceLogic::validate_coordinates(40.7128, 181.0);
        assert_eq!(result, Err(DistanceError::InvalidLongitude(181.0)));
    }

    #[test]
    fn test_calculate_distance_nyc_to_la() {
        // New York to Los Angeles
        let nyc = Coordinates { lat: 40.7128, lon: -74.0060 };
        let la = Coordinates { lat: 34.0522, lon: -118.2437 };
        
        let distance = DistanceLogic::calculate_distance(nyc, la);
        
        // Expected distance is approximately 3944 km / 2451 miles
        assert!((distance.km - 3944.0).abs() < 50.0);  // Within 50km tolerance
        assert!((distance.miles - 2451.0).abs() < 50.0); // Within 50 miles tolerance
    }

    #[test]
    fn test_calculate_same_point() {
        let point = Coordinates { lat: 40.7128, lon: -74.0060 };
        let distance = DistanceLogic::calculate_distance(point, point);
        
        assert!((distance.km).abs() < 0.001); // Should be very close to 0
        assert!((distance.miles).abs() < 0.001);
    }

    #[test]
    fn test_calculate_with_validation_success() {
        let result = DistanceLogic::calculate_with_validation(40.7128, -74.0060, 34.0522, -118.2437);
        assert!(result.is_ok());
        let distance = result.unwrap();
        assert!(distance.km > 3000.0); // Should be a substantial distance
    }

    #[test]
    fn test_calculate_with_validation_failure() {
        let result = DistanceLogic::calculate_with_validation(91.0, -74.0060, 34.0522, -118.2437);
        assert_eq!(result, Err(DistanceError::InvalidLatitude(91.0)));
    }
}