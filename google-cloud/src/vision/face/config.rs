use crate::vision::{api, FeatureConfig};

/// Represents the text detection's configuration.
#[derive(Clone)]
pub struct FaceDetectionConfig {
    pub(crate) max_results: i32,
}

impl FaceDetectionConfig {
    /// Add a language hint for text detection.
    /// Language detection is automatic if none specified.
    pub fn max_results(mut self, max_results: i32) -> FaceDetectionConfig {
        self.max_results = max_results;
        self
    }
}

impl Default for FaceDetectionConfig {
    fn default() -> FaceDetectionConfig {
        FaceDetectionConfig { max_results: 10 }
    }
}

impl FeatureConfig for FaceDetectionConfig {
    fn feature_type(&self) -> api::feature::Type {
        api::feature::Type::FaceDetection
    }
}

impl Into<api::Feature> for FaceDetectionConfig {
    fn into(self) -> api::Feature {
        api::Feature {
            r#type: api::feature::Type::FaceDetection as i32,
            max_results: self.max_results,
            model: String::from("builtin/stable"),
        }
    }
}
