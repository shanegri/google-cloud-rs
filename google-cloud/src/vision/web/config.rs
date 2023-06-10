use crate::vision::{api, FeatureConfig};

/// Configuration for a web detections request
#[derive(Clone)]
pub struct WebDetectionConfig {
    /// Maximum number of web results to return
    pub(crate) max_results: i32,

    /// Use the image's geo metadata in search
    pub(crate) include_geo_results: bool,
}

impl WebDetectionConfig {
    /// Create new config
    pub fn new(max_results: i32, include_geo_results: bool) -> Self {
        WebDetectionConfig {
            max_results,
            include_geo_results,
        }
    }
}

impl Default for WebDetectionConfig {
    fn default() -> Self {
        WebDetectionConfig {
            max_results: 10,
            include_geo_results: false,
        }
    }
}

impl FeatureConfig for WebDetectionConfig {
    fn feature_type(&self) -> api::feature::Type {
        api::feature::Type::TextDetection
    }
    fn update_context(&self, context: api::ImageContext) -> api::ImageContext {
        api::ImageContext {
            web_detection_params: Some(api::WebDetectionParams {
                include_geo_results: self.include_geo_results,
            }),
            ..context
        }
    }
}

impl Into<api::Feature> for WebDetectionConfig {
    fn into(self) -> api::Feature {
        api::Feature {
            r#type: api::feature::Type::WebDetection as i32,
            max_results: self.max_results,
            model: String::from("builtin/stable"),
        }
    }
}
