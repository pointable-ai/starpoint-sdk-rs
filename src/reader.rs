pub use progenitor_client::{ByteStream, Error, ResponseValue};
#[allow(unused_imports)]
use progenitor_client::{encode_path, RequestBuilderExt};
#[allow(unused_imports)]
use reqwest::header::{HeaderMap, HeaderValue};
pub mod types {
    use serde::{Deserialize, Serialize};
    #[allow(unused_imports)]
    use std::convert::TryFrom;
    ///InferSchemaBody
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /**{
  "type": "object",
  "properties": {
    "collection_id": {
      "type": [
        "string",
        "null"
      ],
      "format": "uuid"
    },
    "collection_name": {
      "type": [
        "string",
        "null"
      ]
    }
  }
}*/
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct InferSchemaBody {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub collection_id: Option<uuid::Uuid>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub collection_name: Option<String>,
    }
    impl From<&InferSchemaBody> for InferSchemaBody {
        fn from(value: &InferSchemaBody) -> Self {
            value.clone()
        }
    }
    ///InferSchemaResponse
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /**{
  "type": "object",
  "required": [
    "inferred_schema"
  ],
  "properties": {
    "inferred_schema": {
      "type": "object",
      "required": [
        "nullability",
        "types"
      ],
      "properties": {
        "nullability": {
          "type": "object",
          "additionalProperties": {
            "type": "boolean"
          }
        },
        "types": {
          "type": "object",
          "additionalProperties": {
            "type": "array",
            "items": {
              "type": "string",
              "enum": [
                "String",
                "Number",
                "Boolean",
                "Array",
                "Object"
              ]
            },
            "uniqueItems": true
          }
        }
      }
    }
  }
}*/
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct InferSchemaResponse {
        pub inferred_schema: InferSchemaResponseInferredSchema,
    }
    impl From<&InferSchemaResponse> for InferSchemaResponse {
        fn from(value: &InferSchemaResponse) -> Self {
            value.clone()
        }
    }
    ///InferSchemaResponseInferredSchema
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /**{
  "type": "object",
  "required": [
    "nullability",
    "types"
  ],
  "properties": {
    "nullability": {
      "type": "object",
      "additionalProperties": {
        "type": "boolean"
      }
    },
    "types": {
      "type": "object",
      "additionalProperties": {
        "type": "array",
        "items": {
          "type": "string",
          "enum": [
            "String",
            "Number",
            "Boolean",
            "Array",
            "Object"
          ]
        },
        "uniqueItems": true
      }
    }
  }
}*/
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct InferSchemaResponseInferredSchema {
        pub nullability: std::collections::HashMap<String, bool>,
        pub types: std::collections::HashMap<
            String,
            Vec<InferSchemaResponseInferredSchemaTypesValueItem>,
        >,
    }
    impl From<&InferSchemaResponseInferredSchema> for InferSchemaResponseInferredSchema {
        fn from(value: &InferSchemaResponseInferredSchema) -> Self {
            value.clone()
        }
    }
    ///InferSchemaResponseInferredSchemaTypesValueItem
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /**{
  "type": "string",
  "enum": [
    "String",
    "Number",
    "Boolean",
    "Array",
    "Object"
  ]
}*/
    /// ```
    /// </details>
    #[derive(
        Clone,
        Copy,
        Debug,
        Deserialize,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
        Serialize
    )]
    pub enum InferSchemaResponseInferredSchemaTypesValueItem {
        String,
        Number,
        Boolean,
        Array,
        Object,
    }
    impl From<&InferSchemaResponseInferredSchemaTypesValueItem>
    for InferSchemaResponseInferredSchemaTypesValueItem {
        fn from(value: &InferSchemaResponseInferredSchemaTypesValueItem) -> Self {
            value.clone()
        }
    }
    impl ToString for InferSchemaResponseInferredSchemaTypesValueItem {
        fn to_string(&self) -> String {
            match *self {
                Self::String => "String".to_string(),
                Self::Number => "Number".to_string(),
                Self::Boolean => "Boolean".to_string(),
                Self::Array => "Array".to_string(),
                Self::Object => "Object".to_string(),
            }
        }
    }
    impl std::str::FromStr for InferSchemaResponseInferredSchemaTypesValueItem {
        type Err = &'static str;
        fn from_str(value: &str) -> Result<Self, &'static str> {
            match value {
                "String" => Ok(Self::String),
                "Number" => Ok(Self::Number),
                "Boolean" => Ok(Self::Boolean),
                "Array" => Ok(Self::Array),
                "Object" => Ok(Self::Object),
                _ => Err("invalid value"),
            }
        }
    }
    impl std::convert::TryFrom<&str>
    for InferSchemaResponseInferredSchemaTypesValueItem {
        type Error = &'static str;
        fn try_from(value: &str) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<&String>
    for InferSchemaResponseInferredSchemaTypesValueItem {
        type Error = &'static str;
        fn try_from(value: &String) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<String>
    for InferSchemaResponseInferredSchemaTypesValueItem {
        type Error = &'static str;
        fn try_from(value: String) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    ///QueryDocumentsBody
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /**{
  "type": "object",
  "properties": {
    "collection_id": {
      "type": [
        "string",
        "null"
      ],
      "format": "uuid"
    },
    "collection_name": {
      "type": [
        "string",
        "null"
      ]
    },
    "distance_metric": {
      "allOf": [
        {
          "type": "string",
          "enum": [
            "L2",
            "EUCLIDEAN",
            "COSINE",
            "DOT"
          ]
        }
      ]
    },
    "params": {
      "type": [
        "array",
        "null"
      ],
      "items": {}
    },
    "query_document_id": {
      "type": [
        "string",
        "null"
      ]
    },
    "query_embedding": {
      "allOf": [
        {
          "type": "object",
          "required": [
            "dimensionality",
            "values"
          ],
          "properties": {
            "dimensionality": {
              "type": "integer",
              "minimum": 0.0
            },
            "values": {
              "type": "array",
              "items": {
                "type": "number",
                "format": "float"
              }
            }
          }
        }
      ]
    },
    "sql": {
      "type": [
        "string",
        "null"
      ]
    },
    "text_search_query": {
      "type": [
        "array",
        "null"
      ],
      "items": {
        "type": "string"
      }
    },
    "text_search_weight": {
      "type": [
        "number",
        "null"
      ],
      "format": "float"
    },
    "tokenizer_type": {
      "allOf": [
        {
          "type": "string",
          "enum": [
            "llama2",
            "en_stem",
            "naive"
          ]
        }
      ]
    }
  }
}*/
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct QueryDocumentsBody {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub collection_id: Option<uuid::Uuid>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub collection_name: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub distance_metric: Option<QueryDocumentsBodyDistanceMetric>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub params: Option<Vec<serde_json::Value>>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub query_document_id: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub query_embedding: Option<QueryDocumentsBodyQueryEmbedding>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub sql: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub text_search_query: Option<Vec<String>>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub text_search_weight: Option<f32>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub tokenizer_type: Option<QueryDocumentsBodyTokenizerType>,
    }
    impl From<&QueryDocumentsBody> for QueryDocumentsBody {
        fn from(value: &QueryDocumentsBody) -> Self {
            value.clone()
        }
    }
    ///QueryDocumentsBodyDistanceMetric
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /**{
  "type": "string",
  "enum": [
    "L2",
    "EUCLIDEAN",
    "COSINE",
    "DOT"
  ]
}*/
    /// ```
    /// </details>
    #[derive(
        Clone,
        Copy,
        Debug,
        Deserialize,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
        Serialize
    )]
    pub enum QueryDocumentsBodyDistanceMetric {
        L2,
        #[serde(rename = "EUCLIDEAN")]
        Euclidean,
        #[serde(rename = "COSINE")]
        Cosine,
        #[serde(rename = "DOT")]
        Dot,
    }
    impl From<&QueryDocumentsBodyDistanceMetric> for QueryDocumentsBodyDistanceMetric {
        fn from(value: &QueryDocumentsBodyDistanceMetric) -> Self {
            value.clone()
        }
    }
    impl ToString for QueryDocumentsBodyDistanceMetric {
        fn to_string(&self) -> String {
            match *self {
                Self::L2 => "L2".to_string(),
                Self::Euclidean => "EUCLIDEAN".to_string(),
                Self::Cosine => "COSINE".to_string(),
                Self::Dot => "DOT".to_string(),
            }
        }
    }
    impl std::str::FromStr for QueryDocumentsBodyDistanceMetric {
        type Err = &'static str;
        fn from_str(value: &str) -> Result<Self, &'static str> {
            match value {
                "L2" => Ok(Self::L2),
                "EUCLIDEAN" => Ok(Self::Euclidean),
                "COSINE" => Ok(Self::Cosine),
                "DOT" => Ok(Self::Dot),
                _ => Err("invalid value"),
            }
        }
    }
    impl std::convert::TryFrom<&str> for QueryDocumentsBodyDistanceMetric {
        type Error = &'static str;
        fn try_from(value: &str) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<&String> for QueryDocumentsBodyDistanceMetric {
        type Error = &'static str;
        fn try_from(value: &String) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<String> for QueryDocumentsBodyDistanceMetric {
        type Error = &'static str;
        fn try_from(value: String) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    ///QueryDocumentsBodyQueryEmbedding
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /**{
  "type": "object",
  "required": [
    "dimensionality",
    "values"
  ],
  "properties": {
    "dimensionality": {
      "type": "integer",
      "minimum": 0.0
    },
    "values": {
      "type": "array",
      "items": {
        "type": "number",
        "format": "float"
      }
    }
  }
}*/
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct QueryDocumentsBodyQueryEmbedding {
        pub dimensionality: u64,
        pub values: Vec<f32>,
    }
    impl From<&QueryDocumentsBodyQueryEmbedding> for QueryDocumentsBodyQueryEmbedding {
        fn from(value: &QueryDocumentsBodyQueryEmbedding) -> Self {
            value.clone()
        }
    }
    ///QueryDocumentsBodyTokenizerType
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /**{
  "type": "string",
  "enum": [
    "llama2",
    "en_stem",
    "naive"
  ]
}*/
    /// ```
    /// </details>
    #[derive(
        Clone,
        Copy,
        Debug,
        Deserialize,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
        Serialize
    )]
    pub enum QueryDocumentsBodyTokenizerType {
        #[serde(rename = "llama2")]
        Llama2,
        #[serde(rename = "en_stem")]
        EnStem,
        #[serde(rename = "naive")]
        Naive,
    }
    impl From<&QueryDocumentsBodyTokenizerType> for QueryDocumentsBodyTokenizerType {
        fn from(value: &QueryDocumentsBodyTokenizerType) -> Self {
            value.clone()
        }
    }
    impl ToString for QueryDocumentsBodyTokenizerType {
        fn to_string(&self) -> String {
            match *self {
                Self::Llama2 => "llama2".to_string(),
                Self::EnStem => "en_stem".to_string(),
                Self::Naive => "naive".to_string(),
            }
        }
    }
    impl std::str::FromStr for QueryDocumentsBodyTokenizerType {
        type Err = &'static str;
        fn from_str(value: &str) -> Result<Self, &'static str> {
            match value {
                "llama2" => Ok(Self::Llama2),
                "en_stem" => Ok(Self::EnStem),
                "naive" => Ok(Self::Naive),
                _ => Err("invalid value"),
            }
        }
    }
    impl std::convert::TryFrom<&str> for QueryDocumentsBodyTokenizerType {
        type Error = &'static str;
        fn try_from(value: &str) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<&String> for QueryDocumentsBodyTokenizerType {
        type Error = &'static str;
        fn try_from(value: &String) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<String> for QueryDocumentsBodyTokenizerType {
        type Error = &'static str;
        fn try_from(value: String) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    ///QueryDocumentsResponse
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /**{
  "type": "object",
  "required": [
    "collection_id",
    "distance_metric",
    "result_count",
    "results"
  ],
  "properties": {
    "collection_id": {
      "type": "string",
      "format": "uuid"
    },
    "distance_metric": {
      "type": "string",
      "enum": [
        "L2",
        "EUCLIDEAN",
        "COSINE",
        "DOT"
      ]
    },
    "result_count": {
      "type": "integer",
      "minimum": 0.0
    },
    "results": {
      "type": "array",
      "items": {}
    },
    "sql": {
      "type": [
        "string",
        "null"
      ]
    }
  }
}*/
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct QueryDocumentsResponse {
        pub collection_id: uuid::Uuid,
        pub distance_metric: QueryDocumentsResponseDistanceMetric,
        pub result_count: u64,
        pub results: Vec<serde_json::Value>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub sql: Option<String>,
    }
    impl From<&QueryDocumentsResponse> for QueryDocumentsResponse {
        fn from(value: &QueryDocumentsResponse) -> Self {
            value.clone()
        }
    }
    ///QueryDocumentsResponseDistanceMetric
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /**{
  "type": "string",
  "enum": [
    "L2",
    "EUCLIDEAN",
    "COSINE",
    "DOT"
  ]
}*/
    /// ```
    /// </details>
    #[derive(
        Clone,
        Copy,
        Debug,
        Deserialize,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
        Serialize
    )]
    pub enum QueryDocumentsResponseDistanceMetric {
        L2,
        #[serde(rename = "EUCLIDEAN")]
        Euclidean,
        #[serde(rename = "COSINE")]
        Cosine,
        #[serde(rename = "DOT")]
        Dot,
    }
    impl From<&QueryDocumentsResponseDistanceMetric>
    for QueryDocumentsResponseDistanceMetric {
        fn from(value: &QueryDocumentsResponseDistanceMetric) -> Self {
            value.clone()
        }
    }
    impl ToString for QueryDocumentsResponseDistanceMetric {
        fn to_string(&self) -> String {
            match *self {
                Self::L2 => "L2".to_string(),
                Self::Euclidean => "EUCLIDEAN".to_string(),
                Self::Cosine => "COSINE".to_string(),
                Self::Dot => "DOT".to_string(),
            }
        }
    }
    impl std::str::FromStr for QueryDocumentsResponseDistanceMetric {
        type Err = &'static str;
        fn from_str(value: &str) -> Result<Self, &'static str> {
            match value {
                "L2" => Ok(Self::L2),
                "EUCLIDEAN" => Ok(Self::Euclidean),
                "COSINE" => Ok(Self::Cosine),
                "DOT" => Ok(Self::Dot),
                _ => Err("invalid value"),
            }
        }
    }
    impl std::convert::TryFrom<&str> for QueryDocumentsResponseDistanceMetric {
        type Error = &'static str;
        fn try_from(value: &str) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<&String> for QueryDocumentsResponseDistanceMetric {
        type Error = &'static str;
        fn try_from(value: &String) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<String> for QueryDocumentsResponseDistanceMetric {
        type Error = &'static str;
        fn try_from(value: String) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    ///QueryRequest
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /**{
  "type": "object",
  "properties": {
    "collection_id": {
      "type": [
        "string",
        "null"
      ],
      "format": "uuid"
    },
    "collection_name": {
      "type": [
        "string",
        "null"
      ]
    },
    "distance_metric": {
      "allOf": [
        {
          "type": "string",
          "enum": [
            "L2",
            "EUCLIDEAN",
            "COSINE",
            "DOT"
          ]
        }
      ]
    },
    "params": {
      "type": [
        "array",
        "null"
      ],
      "items": {}
    },
    "query_document_id": {
      "type": [
        "string",
        "null"
      ]
    },
    "query_embedding": {
      "allOf": [
        {
          "type": "object",
          "required": [
            "dimensionality",
            "values"
          ],
          "properties": {
            "dimensionality": {
              "type": "integer",
              "minimum": 0.0
            },
            "values": {
              "type": "array",
              "items": {
                "type": "number",
                "format": "float"
              }
            }
          }
        }
      ]
    },
    "sql": {
      "type": [
        "string",
        "null"
      ]
    },
    "text_search_query": {
      "type": [
        "array",
        "null"
      ],
      "items": {
        "type": "string"
      }
    },
    "text_search_weight": {
      "type": [
        "number",
        "null"
      ],
      "format": "float"
    },
    "tokenizer_type": {
      "allOf": [
        {
          "type": "string",
          "enum": [
            "llama2",
            "en_stem",
            "naive"
          ]
        }
      ]
    }
  }
}*/
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct QueryRequest {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub collection_id: Option<uuid::Uuid>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub collection_name: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub distance_metric: Option<QueryRequestDistanceMetric>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub params: Option<Vec<serde_json::Value>>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub query_document_id: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub query_embedding: Option<QueryRequestQueryEmbedding>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub sql: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub text_search_query: Option<Vec<String>>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub text_search_weight: Option<f32>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub tokenizer_type: Option<QueryRequestTokenizerType>,
    }
    impl From<&QueryRequest> for QueryRequest {
        fn from(value: &QueryRequest) -> Self {
            value.clone()
        }
    }
    ///QueryRequestDistanceMetric
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /**{
  "type": "string",
  "enum": [
    "L2",
    "EUCLIDEAN",
    "COSINE",
    "DOT"
  ]
}*/
    /// ```
    /// </details>
    #[derive(
        Clone,
        Copy,
        Debug,
        Deserialize,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
        Serialize
    )]
    pub enum QueryRequestDistanceMetric {
        L2,
        #[serde(rename = "EUCLIDEAN")]
        Euclidean,
        #[serde(rename = "COSINE")]
        Cosine,
        #[serde(rename = "DOT")]
        Dot,
    }
    impl From<&QueryRequestDistanceMetric> for QueryRequestDistanceMetric {
        fn from(value: &QueryRequestDistanceMetric) -> Self {
            value.clone()
        }
    }
    impl ToString for QueryRequestDistanceMetric {
        fn to_string(&self) -> String {
            match *self {
                Self::L2 => "L2".to_string(),
                Self::Euclidean => "EUCLIDEAN".to_string(),
                Self::Cosine => "COSINE".to_string(),
                Self::Dot => "DOT".to_string(),
            }
        }
    }
    impl std::str::FromStr for QueryRequestDistanceMetric {
        type Err = &'static str;
        fn from_str(value: &str) -> Result<Self, &'static str> {
            match value {
                "L2" => Ok(Self::L2),
                "EUCLIDEAN" => Ok(Self::Euclidean),
                "COSINE" => Ok(Self::Cosine),
                "DOT" => Ok(Self::Dot),
                _ => Err("invalid value"),
            }
        }
    }
    impl std::convert::TryFrom<&str> for QueryRequestDistanceMetric {
        type Error = &'static str;
        fn try_from(value: &str) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<&String> for QueryRequestDistanceMetric {
        type Error = &'static str;
        fn try_from(value: &String) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<String> for QueryRequestDistanceMetric {
        type Error = &'static str;
        fn try_from(value: String) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    ///QueryRequestQueryEmbedding
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /**{
  "type": "object",
  "required": [
    "dimensionality",
    "values"
  ],
  "properties": {
    "dimensionality": {
      "type": "integer",
      "minimum": 0.0
    },
    "values": {
      "type": "array",
      "items": {
        "type": "number",
        "format": "float"
      }
    }
  }
}*/
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct QueryRequestQueryEmbedding {
        pub dimensionality: u64,
        pub values: Vec<f32>,
    }
    impl From<&QueryRequestQueryEmbedding> for QueryRequestQueryEmbedding {
        fn from(value: &QueryRequestQueryEmbedding) -> Self {
            value.clone()
        }
    }
    ///QueryRequestTokenizerType
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /**{
  "type": "string",
  "enum": [
    "llama2",
    "en_stem",
    "naive"
  ]
}*/
    /// ```
    /// </details>
    #[derive(
        Clone,
        Copy,
        Debug,
        Deserialize,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
        Serialize
    )]
    pub enum QueryRequestTokenizerType {
        #[serde(rename = "llama2")]
        Llama2,
        #[serde(rename = "en_stem")]
        EnStem,
        #[serde(rename = "naive")]
        Naive,
    }
    impl From<&QueryRequestTokenizerType> for QueryRequestTokenizerType {
        fn from(value: &QueryRequestTokenizerType) -> Self {
            value.clone()
        }
    }
    impl ToString for QueryRequestTokenizerType {
        fn to_string(&self) -> String {
            match *self {
                Self::Llama2 => "llama2".to_string(),
                Self::EnStem => "en_stem".to_string(),
                Self::Naive => "naive".to_string(),
            }
        }
    }
    impl std::str::FromStr for QueryRequestTokenizerType {
        type Err = &'static str;
        fn from_str(value: &str) -> Result<Self, &'static str> {
            match value {
                "llama2" => Ok(Self::Llama2),
                "en_stem" => Ok(Self::EnStem),
                "naive" => Ok(Self::Naive),
                _ => Err("invalid value"),
            }
        }
    }
    impl std::convert::TryFrom<&str> for QueryRequestTokenizerType {
        type Error = &'static str;
        fn try_from(value: &str) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<&String> for QueryRequestTokenizerType {
        type Error = &'static str;
        fn try_from(value: &String) -> Result<Self, &'static str> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<String> for QueryRequestTokenizerType {
        type Error = &'static str;
        fn try_from(value: String) -> Result<Self, &'static str> {
            value.parse()
        }
    }
}
#[derive(Clone, Debug)]
/**Client for grimoire



Version: 0.1.0*/
pub struct Client {
    pub(crate) baseurl: String,
    pub(crate) client: reqwest::Client,
}
impl Client {
    /// Create a new client.
    ///
    /// `baseurl` is the base URL provided to the internal
    /// `reqwest::Client`, and should include a scheme and hostname,
    /// as well as port and a path stem if applicable.
    pub fn new(baseurl: &str) -> Self {
        #[cfg(not(target_arch = "wasm32"))]
        let client = {
            let dur = std::time::Duration::from_secs(15);
            reqwest::ClientBuilder::new().connect_timeout(dur).timeout(dur)
        };
        #[cfg(target_arch = "wasm32")]
        let client = reqwest::ClientBuilder::new();
        Self::new_with_client(baseurl, client.build().unwrap())
    }
    /// Construct a new client with an existing `reqwest::Client`,
    /// allowing more control over its configuration.
    ///
    /// `baseurl` is the base URL provided to the internal
    /// `reqwest::Client`, and should include a scheme and hostname,
    /// as well as port and a path stem if applicable.
    pub fn new_with_client(baseurl: &str, client: reqwest::Client) -> Self {
        Self {
            baseurl: baseurl.to_string(),
            client,
        }
    }
    /// Get the base URL to which requests are made.
    pub fn baseurl(&self) -> &String {
        &self.baseurl
    }
    /// Get the internal `reqwest::Client` used to make requests.
    pub fn client(&self) -> &reqwest::Client {
        &self.client
    }
    /// Get the version of this API.
    ///
    /// This string is pulled directly from the source OpenAPI
    /// document and may be in any format the API selects.
    pub fn api_version(&self) -> &'static str {
        "0.1.0"
    }
}
impl Client {
    /**Sends a `POST` request to `/api/v1/infer_schema`

*/
    pub async fn infer_schema<'a>(
        &'a self,
        body: &'a types::InferSchemaBody,
    ) -> Result<ResponseValue<types::InferSchemaResponse>, Error<()>> {
        let url = format!("{}/api/v1/infer_schema", self.baseurl,);
        let request = self
            .client
            .post(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Sends a `POST` request to `/api/v1/query`

*/
    pub async fn query_documents<'a>(
        &'a self,
        body: &'a types::QueryDocumentsBody,
    ) -> Result<ResponseValue<types::QueryDocumentsResponse>, Error<()>> {
        let url = format!("{}/api/v1/query", self.baseurl,);
        let request = self
            .client
            .post(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
}
pub mod prelude {
    pub use super::Client;
}
