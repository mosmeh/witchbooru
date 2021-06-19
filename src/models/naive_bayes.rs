use crate::Result;

use ndarray_npy::NpzReader;
use std::io::{Read, Seek};
use tract_onnx::tract_core::ndarray::{Array1, Array2, ArrayView1};

pub struct NaiveBayes {
    array_a: Array2<f32>,
    array_b: Array1<f32>,
}

impl NaiveBayes {
    pub fn new<R: Read + Seek>(reader: R) -> Result<Self> {
        let mut npz = NpzReader::new(reader)?;
        Ok(Self {
            array_a: npz.by_name("a.npy")?,
            array_b: npz.by_name("b.npy")?,
        })
    }

    pub fn predict(&self, probs: ArrayView1<f32>) -> Array1<f32> {
        probs.dot(&self.array_a) + &self.array_b
    }
}
