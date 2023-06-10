use crate::vision::{api, FeatureConfig};

/// Configuration for a label detections request
#[derive(Clone)]
pub struct LabelDetectionConfig {
    /// Maximum number of web results to return
    pub(crate) max_results: i32,
}

impl LabelDetectionConfig {
    /// Create new config
    pub fn new(max_results: i32) -> Self {
        LabelDetectionConfig { max_results }
    }
}

impl Default for LabelDetectionConfig {
    fn default() -> Self {
        LabelDetectionConfig { max_results: 10 }
    }
}

impl FeatureConfig for LabelDetectionConfig {
    fn feature_type(&self) -> api::feature::Type {
        api::feature::Type::LabelDetection
    }
}

impl Into<api::Feature> for LabelDetectionConfig {
    fn into(self) -> api::Feature {
        api::Feature {
            r#type: self.feature_type() as i32,
            max_results: self.max_results,
            model: String::from("builtin/stable"),
        }
    }
}
