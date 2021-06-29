use crate::{
    models::{NaiveBayes, NeuralNet},
    Result,
};

use image::DynamicImage;
use serde::Serialize;
use std::cmp::Reverse;
use tract_onnx::tract_core::{ndarray::ArrayView1, tract_data::itertools::Itertools};

pub struct Params {
    pub neural_net: NeuralNet,
    pub naive_bayes: NaiveBayes,
    pub general_tags: Vec<String>,
    pub character_tags: Vec<String>,
    pub topk: usize,
}

pub struct Prediction<'a> {
    general_tags: Vec<Tag<'a>>,
    character_tags: Vec<Tag<'a>>,
}

impl Prediction<'_> {
    pub fn general(&self) -> &[Tag] {
        &self.general_tags
    }

    pub fn character(&self) -> &[Tag] {
        &self.character_tags
    }
}

#[derive(Serialize)]
pub struct Tag<'a> {
    pub name: &'a str,
    pub score: f32,
}

pub struct Classifier {
    neural_net: NeuralNet,
    naive_bayes: NaiveBayes,
    general_tags: Vec<String>,
    character_tags: Vec<String>,
    topk: usize,
}

impl Classifier {
    pub fn new(params: Params) -> Result<Self> {
        Ok(Self {
            neural_net: params.neural_net,
            naive_bayes: params.naive_bayes,
            general_tags: params.general_tags,
            character_tags: params.character_tags,
            topk: params.topk,
        })
    }

    pub fn predict(&self, img: DynamicImage) -> Result<Prediction> {
        let output = self.neural_net.predict(img)?;

        let general_tag_probs = &output.as_slice::<f32>()?[..self.general_tags.len()];
        let general_tag_probs =
            ArrayView1::from_shape((self.general_tags.len(),), general_tag_probs)?;

        let general_tags = self
            .general_tags
            .iter()
            .zip(general_tag_probs.iter())
            .filter(|(_, prob)| **prob > 0.5)
            .map(|(name, prob)| {
                Reverse(ScoreCmp(Tag {
                    name: name.as_str(),
                    score: *prob,
                }))
            })
            .k_smallest(self.topk)
            .map(|Reverse(ScoreCmp(x))| x)
            .collect();

        let character_logits = self.naive_bayes.predict(general_tag_probs);
        let character_tags = self
            .character_tags
            .iter()
            .zip(character_logits.iter())
            .map(|(name, logit)| {
                Reverse(ScoreCmp(Tag {
                    name: name.as_str(),
                    score: *logit,
                }))
            })
            .k_smallest(self.topk)
            .map(|Reverse(ScoreCmp(Tag { name, score: logit }))| Tag {
                name,
                score: sigmoid(logit),
            })
            .collect();

        Ok(Prediction {
            general_tags,
            character_tags,
        })
    }
}

fn sigmoid(x: f32) -> f32 {
    1.0 / (1.0 + f32::exp(-x))
}

struct ScoreCmp<'a>(Tag<'a>);

impl PartialEq for ScoreCmp<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.0.score.eq(&other.0.score)
    }
}

impl Eq for ScoreCmp<'_> {}

impl PartialOrd for ScoreCmp<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for ScoreCmp<'_> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // from nightly standard library

        let mut left = self.0.score.to_bits() as i32;
        let mut right = other.0.score.to_bits() as i32;

        left ^= (((left >> 31) as u32) >> 1) as i32;
        right ^= (((right >> 31) as u32) >> 1) as i32;

        left.cmp(&right)
    }
}
