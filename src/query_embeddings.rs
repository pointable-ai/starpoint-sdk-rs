use serde::{Deserialize, Serialize};
use serde_valid::{validation, Validate};

fn validate_embedding(vectors: &[f32], dim: &usize) -> Result<(), validation::Error> {
    if vectors.len() % dim != 0 {
        return Err(validation::Error::Custom(
            "dimensionality mismatch".to_string(),
        ));
    }
    Ok(())
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[rule(validate_embedding(values, dimensionality))]

pub struct QueryEmbeddings {
    pub values: Vec<f32>,
    pub dimensionality: usize,
}

impl QueryEmbeddings {
    pub fn is_some(&self) -> bool {
        !self.values.is_empty()
    }

    pub fn matches_dim(&self, dim: usize) -> bool {
        self.dimensionality == dim && self.values.len() % dim == 0
    }

    pub fn none() -> Self {
        Self {
            values: vec![],
            dimensionality: 0,
        }
    }

    pub fn from_single_vec(vectors: Vec<f32>) -> Self {
        let dim = vectors.len();
        Self {
            values: vectors,
            dimensionality: dim,
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = &[f32]> {
        self.values.chunks(self.dimensionality)
    }
}
