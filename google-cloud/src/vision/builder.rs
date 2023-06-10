use std::{collections::HashMap, vec};

use crate::error::BatchError;

use super::{api, Error};

/// Config that can be converted to a feature.
pub trait FeatureConfig: Into<api::Feature> + Clone {
    /// https://cloud.google.com/vision/docs/reference/rest/v1/Feature#Type
    fn feature_type(&self) -> api::feature::Type;

    /// Updates context with configuration for this feature.
    /// https://cloud.google.com/vision/docs/reference/rest/v1/ImageContext
    fn update_context(&self, context: api::ImageContext) -> api::ImageContext {
        context
    }
}

/// A constructed request.
pub struct BatchRequest {
    pub(crate) requests: Vec<api::AnnotateImageRequest>,
}

/// Data for a batched vision request.
pub struct Builder {
    features: HashMap<api::feature::Type, api::Feature>,
    images: Vec<api::Image>,
    context: api::ImageContext,
}

impl Builder {
    /// Create a new builder.
    pub fn new() -> Self {
        Builder {
            features: HashMap::new(),
            images: vec![],
            context: api::ImageContext {
                lat_long_rect: None,
                crop_hints_params: None,
                product_search_params: None,
                web_detection_params: None,
                language_hints: vec![],
            },
        }
    }

    /// Add an image to the request.
    pub fn add_image(&mut self, image: api::Image) -> &mut Self {
        self.images.push(image);
        self
    }

    /// Add a feature to the request.
    pub fn add_feature<T: FeatureConfig>(&mut self, config: &T) -> &mut Self {
        let feature: api::Feature = config.clone().into();
        self.context = config.update_context(self.context.to_owned());
        self.features.insert(config.feature_type().clone(), feature);
        self
    }

    /// Create a request from provided images/features.
    /// At least one image and feature are required.
    pub fn build(&mut self) -> Result<BatchRequest, Error> {
        if self.images.is_empty() {
            return Err(Error::Batch(BatchError::ImagesRequired));
        }
        if self.features.is_empty() {
            return Err(Error::Batch(BatchError::FeaturesRequired));
        }
        let mut requests: Vec<api::AnnotateImageRequest> = vec![];

        let features: Vec<api::Feature> = self.features.values().cloned().collect();

        for image in self.images.drain(..) {
            requests.push(api::AnnotateImageRequest {
                image: Some(image.into()),
                features: features.to_owned(),
                image_context: Some(self.context.to_owned()),
            })
        }
        Ok(BatchRequest { requests })
    }
}
