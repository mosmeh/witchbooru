mod classifier;
pub mod models;
mod topk;

pub use classifier::{Classifier, Params, Prediction};
pub use image;
pub use tract_onnx::tract_core::ndarray;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Tract(#[from] tract_onnx::tract_core::TractError),

    #[error(transparent)]
    NdArrayShape(#[from] tract_onnx::tract_core::ndarray::ShapeError),

    #[error(transparent)]
    ReadNpz(#[from] ndarray_npy::ReadNpzError),
}

pub type Result<T> = std::result::Result<T, Error>;
