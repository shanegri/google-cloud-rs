use crate::vision::{api, FeatureConfig};

// TODO: underlying api is missing imageContext.textDetectionParams.
//       https://cloud.google.com/vision/docs/reference/rest/v1/ImageContext#TextDetectionParams
//       - add or regen: https://github.com/google-apis-rs/generator

/// Restrict text to text/document detections.
pub enum TextDetectionType {
    /// Optimized for areas of text within a larger image.
    /// If the image is a document, use DocumentDetection instead.
    TextDetection,
    /// Run dense text document OCR.
    DocumentDetection,
}

impl Into<api::feature::Type> for TextDetectionType {
    fn into(self) -> api::feature::Type {
        match self {
            TextDetectionType::DocumentDetection => api::feature::Type::DocumentTextDetection,
            TextDetectionType::TextDetection => api::feature::Type::TextDetection,
        }
    }
}

/// Represents the text detection's configuration.
#[derive(Clone)]
pub struct TextDetectionConfig {
    pub(crate) language_hints: Vec<String>,
    pub(crate) detection_type: api::feature::Type,
}

impl TextDetectionConfig {
    /// Add a language hint for text detection.
    /// Language detection is automatic if none specified.
    pub fn language_hint(mut self, lang: impl Into<String>) -> TextDetectionConfig {
        self.language_hints.push(lang.into());
        self
    }
}

impl TextDetectionConfig {
    /// Create a new OCR detection.
    pub fn new(detection_type: TextDetectionType) -> TextDetectionConfig {
        TextDetectionConfig {
            language_hints: Vec::new(),
            detection_type: detection_type.into(),
        }
    }
}

impl FeatureConfig for TextDetectionConfig {
    fn feature_type(&self) -> api::feature::Type {
        self.detection_type
    }
    fn update_context(&self, context: api::ImageContext) -> api::ImageContext {
        api::ImageContext {
            language_hints: self.language_hints.to_owned(),
            ..context
        }
    }
}

impl Into<api::Feature> for TextDetectionConfig {
    fn into(self) -> api::Feature {
        api::Feature {
            r#type: self.feature_type() as i32,
            max_results: 0, // Does not apply for TEXT_DETECTION/DOCUMENT_DETECTION.
            model: String::from("builtin/stable"),
        }
    }
}
