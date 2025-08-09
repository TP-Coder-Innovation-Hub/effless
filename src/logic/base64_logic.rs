use base64::{Engine as _, engine::general_purpose};

#[derive(Debug, PartialEq)]
pub enum Base64Error {
    InvalidBase64,
    InvalidUtf8,
}

pub struct Base64Logic;

impl Base64Logic {
    /// Encode text to Base64
    pub fn encode(input: &str) -> String {
        general_purpose::STANDARD.encode(input.as_bytes())
    }
    
    /// Decode Base64 to text
    pub fn decode(input: &str) -> Result<String, Base64Error> {
        let decoded = general_purpose::STANDARD.decode(input)
            .map_err(|_| Base64Error::InvalidBase64)?;
        
        String::from_utf8(decoded)
            .map_err(|_| Base64Error::InvalidUtf8)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_basic() {
        let result = Base64Logic::encode("Hello World");
        assert_eq!(result, "SGVsbG8gV29ybGQ=");
    }

    #[test]
    fn test_encode_empty() {
        let result = Base64Logic::encode("");
        assert_eq!(result, "");
    }

    #[test]
    fn test_decode_basic() {
        let result = Base64Logic::decode("SGVsbG8gV29ybGQ=").unwrap();
        assert_eq!(result, "Hello World");
    }

    #[test]
    fn test_decode_invalid_base64() {
        let result = Base64Logic::decode("invalid!!!base64");
        assert_eq!(result, Err(Base64Error::InvalidBase64));
    }

    #[test]
    fn test_round_trip() {
        let original = "The quick brown fox jumps over the lazy dog";
        let encoded = Base64Logic::encode(original);
        let decoded = Base64Logic::decode(&encoded).unwrap();
        assert_eq!(original, decoded);
    }
}