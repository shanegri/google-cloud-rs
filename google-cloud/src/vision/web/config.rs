use crate::vision::api;

/// Configuration for a web detections request
pub struct WebDetectionConfig {
    /// Maximum number of web results to return
    pub(crate) max_results: i32,

    /// Use the image's geo metadata in search
    pub(crate) include_geo_results: bool,
}

impl WebDetectionConfig {
    /// Create new config 
    pub fn new(max_results: i32, include_geo_results: bool) -> Self {
        WebDetectionConfig { max_results, include_geo_results }
    }
}

impl Default for WebDetectionConfig {
    fn default() -> Self {
        WebDetectionConfig { max_results: 10, include_geo_results: false }
    }
}

impl From<WebDetectionConfig> for api::ImageContext {
    fn from(config: WebDetectionConfig) -> api::ImageContext {
        api::ImageContext {
            lat_long_rect: None,
            crop_hints_params: None,
            product_search_params: None,
            web_detection_params: Some(api::WebDetectionParams {
                include_geo_results: config.include_geo_results
            }),
            language_hints: vec![],
        }
    }
}
