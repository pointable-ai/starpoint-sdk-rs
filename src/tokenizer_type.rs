use serde::{Deserialize, Serialize};

#[derive(Default, Clone, Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum TokenizerType {
    Llama2,
    #[default]
    EnStem,
    Naive,
}
