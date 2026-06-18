// Domain logic for core Photorg functionality

/// Represents a rating value for an image
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Rating {
    Rejected = 0,
    Keep = 1,
    Good = 2,
    Great = 3,
}

impl Rating {
    pub fn from_score(score: u32) -> Option<Self> {
        match score {
            0 => Some(Rating::Rejected),
            1 => Some(Rating::Keep),
            2 => Some(Rating::Good),
            3 => Some(Rating::Great),
            _ => None,
        }
    }

    pub fn to_score(&self) -> u32 {
        *self as u32
    }
}

/// Represents a pair of RAW and JPEG images
#[derive(Debug, Clone)]
pub struct ImagePair {
    raw_path: String,
    jpeg_path: Option<String>,
}

impl ImagePair {
    pub fn new(raw_path: String, jpeg_path: Option<String>) -> Self {
        ImagePair { raw_path, jpeg_path }
    }

    pub fn is_valid(&self) -> bool {
        !self.raw_path.is_empty()
    }

    pub fn has_jpeg(&self) -> bool {
        self.jpeg_path.is_some()
    }

    pub fn raw_path(&self) -> &str {
        &self.raw_path
    }

    pub fn jpeg_path(&self) -> Option<&str> {
        self.jpeg_path.as_deref()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rating_from_score() {
        assert_eq!(Rating::from_score(0), Some(Rating::Rejected));
        assert_eq!(Rating::from_score(1), Some(Rating::Keep));
        assert_eq!(Rating::from_score(2), Some(Rating::Good));
        assert_eq!(Rating::from_score(3), Some(Rating::Great));
        assert_eq!(Rating::from_score(4), None);
    }

    #[test]
    fn test_rating_to_score() {
        assert_eq!(Rating::Rejected.to_score(), 0);
        assert_eq!(Rating::Keep.to_score(), 1);
        assert_eq!(Rating::Good.to_score(), 2);
        assert_eq!(Rating::Great.to_score(), 3);
    }

    #[test]
    fn test_rating_ordering() {
        assert!(Rating::Rejected < Rating::Keep);
        assert!(Rating::Keep < Rating::Good);
        assert!(Rating::Good < Rating::Great);
    }

    #[test]
    fn test_image_pair_valid() {
        let pair = ImagePair::new("test.raw".to_string(), Some("test.jpg".to_string()));
        assert!(pair.is_valid());
    }

    #[test]
    fn test_image_pair_invalid_empty_raw() {
        let pair = ImagePair::new("".to_string(), Some("test.jpg".to_string()));
        assert!(!pair.is_valid());
    }

    #[test]
    fn test_image_pair_raw_only() {
        let pair = ImagePair::new("test.raw".to_string(), None);
        assert!(pair.is_valid());
        assert!(!pair.has_jpeg());
    }

    #[test]
    fn test_image_pair_with_jpeg() {
        let pair = ImagePair::new("test.raw".to_string(), Some("test.jpg".to_string()));
        assert!(pair.has_jpeg());
        assert_eq!(pair.raw_path(), "test.raw");
        assert_eq!(pair.jpeg_path(), Some("test.jpg"));
    }
}
