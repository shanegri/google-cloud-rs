mod bounding_box;
mod builder;
mod client;
mod face;
mod image;
mod label;
mod likelihood;
mod text;
mod web;
mod api {
    pub mod rpc {
        include!("api/google.rpc.rs");
    }
    #[allow(clippy::module_inception)]
    pub mod api {
        include!("api/google.api.rs");
    }
    pub mod longrunning {
        include!("api/google.longrunning.rs");
    }
    pub mod protobuf {
        include!("api/google.protobuf.rs");
    }
    pub mod r#type {
        include!("api/google.r#type.rs");
    }
    pub mod cloud {
        pub mod vision {
            pub mod v1 {
                include!("api/google.cloud.vision.v1.rs");
            }
        }
    }
    pub use self::cloud::vision::v1::*;
    pub use self::r#type::*;
}

pub use self::bounding_box::*;
pub use self::builder::*;
pub use self::client::*;
pub use self::face::*;
pub use self::image::*;
pub use self::label::*;
pub use self::likelihood::*;
pub use self::text::*;
pub use self::web::*;

/// The error type for the Cloud Vision module.
pub type Error = crate::error::Error;
