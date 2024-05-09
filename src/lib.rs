pub mod layer;
pub mod network;

pub mod prelude {
    pub use crate::{
        layer::DenseLayer,
        network::Network,
    };
}