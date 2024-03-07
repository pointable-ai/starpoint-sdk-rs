pub use progenitor_client::{ByteStream, Error, ResponseValue};
#[allow(unused_imports)]
use progenitor_client::{encode_path, RequestBuilderExt};
#[allow(unused_imports)]
use reqwest::header::{HeaderMap, HeaderValue};
pub mod types {
    use serde::{Deserialize, Serialize};
    #[allow(unused_imports)]
    use std::convert::TryFrom;
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
    },
    "top_k": {
      "type": [
        "integer",
        "null"
      ],
      "minimum": 0.0
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
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub top_k: Option<u64>,
    }
    impl From<&QueryDocumentsBody> for QueryDocumentsBody {
        fn from(value: &QueryDocumentsBody) -> Self {
            value.clone()
        }
    }
    impl QueryDocumentsBody {
        pub fn builder() -> builder::QueryDocumentsBody {
            Default::default()
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
    impl QueryDocumentsBodyQueryEmbedding {
        pub fn builder() -> builder::QueryDocumentsBodyQueryEmbedding {
            Default::default()
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
      "items": {
        "type": "object",
        "required": [
          "id",
          "metadata",
          "score"
        ],
        "properties": {
          "id": {
            "type": "string"
          },
          "metadata": {
            "type": "object"
          },
          "score": {
            "type": "number",
            "format": "float"
          }
        }
      }
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
        pub results: Vec<QueryDocumentsResponseResultsItem>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub sql: Option<String>,
    }
    impl From<&QueryDocumentsResponse> for QueryDocumentsResponse {
        fn from(value: &QueryDocumentsResponse) -> Self {
            value.clone()
        }
    }
    impl QueryDocumentsResponse {
        pub fn builder() -> builder::QueryDocumentsResponse {
            Default::default()
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
    ///QueryDocumentsResponseResultsItem
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /**{
  "type": "object",
  "required": [
    "id",
    "metadata",
    "score"
  ],
  "properties": {
    "id": {
      "type": "string"
    },
    "metadata": {
      "type": "object"
    },
    "score": {
      "type": "number",
      "format": "float"
    }
  }
}*/
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct QueryDocumentsResponseResultsItem {
        pub id: String,
        pub metadata: serde_json::Map<String, serde_json::Value>,
        pub score: f32,
    }
    impl From<&QueryDocumentsResponseResultsItem> for QueryDocumentsResponseResultsItem {
        fn from(value: &QueryDocumentsResponseResultsItem) -> Self {
            value.clone()
        }
    }
    impl QueryDocumentsResponseResultsItem {
        pub fn builder() -> builder::QueryDocumentsResponseResultsItem {
            Default::default()
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
    },
    "top_k": {
      "type": [
        "integer",
        "null"
      ],
      "minimum": 0.0
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
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub top_k: Option<u64>,
    }
    impl From<&QueryRequest> for QueryRequest {
        fn from(value: &QueryRequest) -> Self {
            value.clone()
        }
    }
    impl QueryRequest {
        pub fn builder() -> builder::QueryRequest {
            Default::default()
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
    impl QueryRequestQueryEmbedding {
        pub fn builder() -> builder::QueryRequestQueryEmbedding {
            Default::default()
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
    pub mod builder {
        #[derive(Clone, Debug)]
        pub struct QueryDocumentsBody {
            collection_id: Result<Option<uuid::Uuid>, String>,
            collection_name: Result<Option<String>, String>,
            distance_metric: Result<
                Option<super::QueryDocumentsBodyDistanceMetric>,
                String,
            >,
            params: Result<Option<Vec<serde_json::Value>>, String>,
            query_document_id: Result<Option<String>, String>,
            query_embedding: Result<
                Option<super::QueryDocumentsBodyQueryEmbedding>,
                String,
            >,
            sql: Result<Option<String>, String>,
            text_search_query: Result<Option<Vec<String>>, String>,
            text_search_weight: Result<Option<f32>, String>,
            tokenizer_type: Result<
                Option<super::QueryDocumentsBodyTokenizerType>,
                String,
            >,
            top_k: Result<Option<u64>, String>,
        }
        impl Default for QueryDocumentsBody {
            fn default() -> Self {
                Self {
                    collection_id: Ok(Default::default()),
                    collection_name: Ok(Default::default()),
                    distance_metric: Ok(Default::default()),
                    params: Ok(Default::default()),
                    query_document_id: Ok(Default::default()),
                    query_embedding: Ok(Default::default()),
                    sql: Ok(Default::default()),
                    text_search_query: Ok(Default::default()),
                    text_search_weight: Ok(Default::default()),
                    tokenizer_type: Ok(Default::default()),
                    top_k: Ok(Default::default()),
                }
            }
        }
        impl QueryDocumentsBody {
            pub fn collection_id<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<uuid::Uuid>>,
                T::Error: std::fmt::Display,
            {
                self
                    .collection_id = value
                    .try_into()
                    .map_err(|e| {
                        format!(
                            "error converting supplied value for collection_id: {}", e
                        )
                    });
                self
            }
            pub fn collection_name<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self
                    .collection_name = value
                    .try_into()
                    .map_err(|e| {
                        format!(
                            "error converting supplied value for collection_name: {}", e
                        )
                    });
                self
            }
            pub fn distance_metric<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<
                    Option<super::QueryDocumentsBodyDistanceMetric>,
                >,
                T::Error: std::fmt::Display,
            {
                self
                    .distance_metric = value
                    .try_into()
                    .map_err(|e| {
                        format!(
                            "error converting supplied value for distance_metric: {}", e
                        )
                    });
                self
            }
            pub fn params<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<Vec<serde_json::Value>>>,
                T::Error: std::fmt::Display,
            {
                self
                    .params = value
                    .try_into()
                    .map_err(|e| {
                        format!("error converting supplied value for params: {}", e)
                    });
                self
            }
            pub fn query_document_id<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self
                    .query_document_id = value
                    .try_into()
                    .map_err(|e| {
                        format!(
                            "error converting supplied value for query_document_id: {}",
                            e
                        )
                    });
                self
            }
            pub fn query_embedding<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<
                    Option<super::QueryDocumentsBodyQueryEmbedding>,
                >,
                T::Error: std::fmt::Display,
            {
                self
                    .query_embedding = value
                    .try_into()
                    .map_err(|e| {
                        format!(
                            "error converting supplied value for query_embedding: {}", e
                        )
                    });
                self
            }
            pub fn sql<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self
                    .sql = value
                    .try_into()
                    .map_err(|e| {
                        format!("error converting supplied value for sql: {}", e)
                    });
                self
            }
            pub fn text_search_query<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<Vec<String>>>,
                T::Error: std::fmt::Display,
            {
                self
                    .text_search_query = value
                    .try_into()
                    .map_err(|e| {
                        format!(
                            "error converting supplied value for text_search_query: {}",
                            e
                        )
                    });
                self
            }
            pub fn text_search_weight<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<f32>>,
                T::Error: std::fmt::Display,
            {
                self
                    .text_search_weight = value
                    .try_into()
                    .map_err(|e| {
                        format!(
                            "error converting supplied value for text_search_weight: {}",
                            e
                        )
                    });
                self
            }
            pub fn tokenizer_type<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<super::QueryDocumentsBodyTokenizerType>>,
                T::Error: std::fmt::Display,
            {
                self
                    .tokenizer_type = value
                    .try_into()
                    .map_err(|e| {
                        format!(
                            "error converting supplied value for tokenizer_type: {}", e
                        )
                    });
                self
            }
            pub fn top_k<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<u64>>,
                T::Error: std::fmt::Display,
            {
                self
                    .top_k = value
                    .try_into()
                    .map_err(|e| {
                        format!("error converting supplied value for top_k: {}", e)
                    });
                self
            }
        }
        impl std::convert::TryFrom<QueryDocumentsBody> for super::QueryDocumentsBody {
            type Error = String;
            fn try_from(value: QueryDocumentsBody) -> Result<Self, String> {
                Ok(Self {
                    collection_id: value.collection_id?,
                    collection_name: value.collection_name?,
                    distance_metric: value.distance_metric?,
                    params: value.params?,
                    query_document_id: value.query_document_id?,
                    query_embedding: value.query_embedding?,
                    sql: value.sql?,
                    text_search_query: value.text_search_query?,
                    text_search_weight: value.text_search_weight?,
                    tokenizer_type: value.tokenizer_type?,
                    top_k: value.top_k?,
                })
            }
        }
        impl From<super::QueryDocumentsBody> for QueryDocumentsBody {
            fn from(value: super::QueryDocumentsBody) -> Self {
                Self {
                    collection_id: Ok(value.collection_id),
                    collection_name: Ok(value.collection_name),
                    distance_metric: Ok(value.distance_metric),
                    params: Ok(value.params),
                    query_document_id: Ok(value.query_document_id),
                    query_embedding: Ok(value.query_embedding),
                    sql: Ok(value.sql),
                    text_search_query: Ok(value.text_search_query),
                    text_search_weight: Ok(value.text_search_weight),
                    tokenizer_type: Ok(value.tokenizer_type),
                    top_k: Ok(value.top_k),
                }
            }
        }
        #[derive(Clone, Debug)]
        pub struct QueryDocumentsBodyQueryEmbedding {
            dimensionality: Result<u64, String>,
            values: Result<Vec<f32>, String>,
        }
        impl Default for QueryDocumentsBodyQueryEmbedding {
            fn default() -> Self {
                Self {
                    dimensionality: Err(
                        "no value supplied for dimensionality".to_string(),
                    ),
                    values: Err("no value supplied for values".to_string()),
                }
            }
        }
        impl QueryDocumentsBodyQueryEmbedding {
            pub fn dimensionality<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<u64>,
                T::Error: std::fmt::Display,
            {
                self
                    .dimensionality = value
                    .try_into()
                    .map_err(|e| {
                        format!(
                            "error converting supplied value for dimensionality: {}", e
                        )
                    });
                self
            }
            pub fn values<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Vec<f32>>,
                T::Error: std::fmt::Display,
            {
                self
                    .values = value
                    .try_into()
                    .map_err(|e| {
                        format!("error converting supplied value for values: {}", e)
                    });
                self
            }
        }
        impl std::convert::TryFrom<QueryDocumentsBodyQueryEmbedding>
        for super::QueryDocumentsBodyQueryEmbedding {
            type Error = String;
            fn try_from(
                value: QueryDocumentsBodyQueryEmbedding,
            ) -> Result<Self, String> {
                Ok(Self {
                    dimensionality: value.dimensionality?,
                    values: value.values?,
                })
            }
        }
        impl From<super::QueryDocumentsBodyQueryEmbedding>
        for QueryDocumentsBodyQueryEmbedding {
            fn from(value: super::QueryDocumentsBodyQueryEmbedding) -> Self {
                Self {
                    dimensionality: Ok(value.dimensionality),
                    values: Ok(value.values),
                }
            }
        }
        #[derive(Clone, Debug)]
        pub struct QueryDocumentsResponse {
            collection_id: Result<uuid::Uuid, String>,
            distance_metric: Result<super::QueryDocumentsResponseDistanceMetric, String>,
            result_count: Result<u64, String>,
            results: Result<Vec<super::QueryDocumentsResponseResultsItem>, String>,
            sql: Result<Option<String>, String>,
        }
        impl Default for QueryDocumentsResponse {
            fn default() -> Self {
                Self {
                    collection_id: Err(
                        "no value supplied for collection_id".to_string(),
                    ),
                    distance_metric: Err(
                        "no value supplied for distance_metric".to_string(),
                    ),
                    result_count: Err("no value supplied for result_count".to_string()),
                    results: Err("no value supplied for results".to_string()),
                    sql: Ok(Default::default()),
                }
            }
        }
        impl QueryDocumentsResponse {
            pub fn collection_id<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<uuid::Uuid>,
                T::Error: std::fmt::Display,
            {
                self
                    .collection_id = value
                    .try_into()
                    .map_err(|e| {
                        format!(
                            "error converting supplied value for collection_id: {}", e
                        )
                    });
                self
            }
            pub fn distance_metric<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<super::QueryDocumentsResponseDistanceMetric>,
                T::Error: std::fmt::Display,
            {
                self
                    .distance_metric = value
                    .try_into()
                    .map_err(|e| {
                        format!(
                            "error converting supplied value for distance_metric: {}", e
                        )
                    });
                self
            }
            pub fn result_count<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<u64>,
                T::Error: std::fmt::Display,
            {
                self
                    .result_count = value
                    .try_into()
                    .map_err(|e| {
                        format!(
                            "error converting supplied value for result_count: {}", e
                        )
                    });
                self
            }
            pub fn results<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Vec<super::QueryDocumentsResponseResultsItem>>,
                T::Error: std::fmt::Display,
            {
                self
                    .results = value
                    .try_into()
                    .map_err(|e| {
                        format!("error converting supplied value for results: {}", e)
                    });
                self
            }
            pub fn sql<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self
                    .sql = value
                    .try_into()
                    .map_err(|e| {
                        format!("error converting supplied value for sql: {}", e)
                    });
                self
            }
        }
        impl std::convert::TryFrom<QueryDocumentsResponse>
        for super::QueryDocumentsResponse {
            type Error = String;
            fn try_from(value: QueryDocumentsResponse) -> Result<Self, String> {
                Ok(Self {
                    collection_id: value.collection_id?,
                    distance_metric: value.distance_metric?,
                    result_count: value.result_count?,
                    results: value.results?,
                    sql: value.sql?,
                })
            }
        }
        impl From<super::QueryDocumentsResponse> for QueryDocumentsResponse {
            fn from(value: super::QueryDocumentsResponse) -> Self {
                Self {
                    collection_id: Ok(value.collection_id),
                    distance_metric: Ok(value.distance_metric),
                    result_count: Ok(value.result_count),
                    results: Ok(value.results),
                    sql: Ok(value.sql),
                }
            }
        }
        #[derive(Clone, Debug)]
        pub struct QueryDocumentsResponseResultsItem {
            id: Result<String, String>,
            metadata: Result<serde_json::Map<String, serde_json::Value>, String>,
            score: Result<f32, String>,
        }
        impl Default for QueryDocumentsResponseResultsItem {
            fn default() -> Self {
                Self {
                    id: Err("no value supplied for id".to_string()),
                    metadata: Err("no value supplied for metadata".to_string()),
                    score: Err("no value supplied for score".to_string()),
                }
            }
        }
        impl QueryDocumentsResponseResultsItem {
            pub fn id<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<String>,
                T::Error: std::fmt::Display,
            {
                self
                    .id = value
                    .try_into()
                    .map_err(|e| {
                        format!("error converting supplied value for id: {}", e)
                    });
                self
            }
            pub fn metadata<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<serde_json::Map<String, serde_json::Value>>,
                T::Error: std::fmt::Display,
            {
                self
                    .metadata = value
                    .try_into()
                    .map_err(|e| {
                        format!("error converting supplied value for metadata: {}", e)
                    });
                self
            }
            pub fn score<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<f32>,
                T::Error: std::fmt::Display,
            {
                self
                    .score = value
                    .try_into()
                    .map_err(|e| {
                        format!("error converting supplied value for score: {}", e)
                    });
                self
            }
        }
        impl std::convert::TryFrom<QueryDocumentsResponseResultsItem>
        for super::QueryDocumentsResponseResultsItem {
            type Error = String;
            fn try_from(
                value: QueryDocumentsResponseResultsItem,
            ) -> Result<Self, String> {
                Ok(Self {
                    id: value.id?,
                    metadata: value.metadata?,
                    score: value.score?,
                })
            }
        }
        impl From<super::QueryDocumentsResponseResultsItem>
        for QueryDocumentsResponseResultsItem {
            fn from(value: super::QueryDocumentsResponseResultsItem) -> Self {
                Self {
                    id: Ok(value.id),
                    metadata: Ok(value.metadata),
                    score: Ok(value.score),
                }
            }
        }
        #[derive(Clone, Debug)]
        pub struct QueryRequest {
            collection_id: Result<Option<uuid::Uuid>, String>,
            collection_name: Result<Option<String>, String>,
            distance_metric: Result<Option<super::QueryRequestDistanceMetric>, String>,
            params: Result<Option<Vec<serde_json::Value>>, String>,
            query_document_id: Result<Option<String>, String>,
            query_embedding: Result<Option<super::QueryRequestQueryEmbedding>, String>,
            sql: Result<Option<String>, String>,
            text_search_query: Result<Option<Vec<String>>, String>,
            text_search_weight: Result<Option<f32>, String>,
            tokenizer_type: Result<Option<super::QueryRequestTokenizerType>, String>,
            top_k: Result<Option<u64>, String>,
        }
        impl Default for QueryRequest {
            fn default() -> Self {
                Self {
                    collection_id: Ok(Default::default()),
                    collection_name: Ok(Default::default()),
                    distance_metric: Ok(Default::default()),
                    params: Ok(Default::default()),
                    query_document_id: Ok(Default::default()),
                    query_embedding: Ok(Default::default()),
                    sql: Ok(Default::default()),
                    text_search_query: Ok(Default::default()),
                    text_search_weight: Ok(Default::default()),
                    tokenizer_type: Ok(Default::default()),
                    top_k: Ok(Default::default()),
                }
            }
        }
        impl QueryRequest {
            pub fn collection_id<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<uuid::Uuid>>,
                T::Error: std::fmt::Display,
            {
                self
                    .collection_id = value
                    .try_into()
                    .map_err(|e| {
                        format!(
                            "error converting supplied value for collection_id: {}", e
                        )
                    });
                self
            }
            pub fn collection_name<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self
                    .collection_name = value
                    .try_into()
                    .map_err(|e| {
                        format!(
                            "error converting supplied value for collection_name: {}", e
                        )
                    });
                self
            }
            pub fn distance_metric<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<super::QueryRequestDistanceMetric>>,
                T::Error: std::fmt::Display,
            {
                self
                    .distance_metric = value
                    .try_into()
                    .map_err(|e| {
                        format!(
                            "error converting supplied value for distance_metric: {}", e
                        )
                    });
                self
            }
            pub fn params<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<Vec<serde_json::Value>>>,
                T::Error: std::fmt::Display,
            {
                self
                    .params = value
                    .try_into()
                    .map_err(|e| {
                        format!("error converting supplied value for params: {}", e)
                    });
                self
            }
            pub fn query_document_id<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self
                    .query_document_id = value
                    .try_into()
                    .map_err(|e| {
                        format!(
                            "error converting supplied value for query_document_id: {}",
                            e
                        )
                    });
                self
            }
            pub fn query_embedding<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<super::QueryRequestQueryEmbedding>>,
                T::Error: std::fmt::Display,
            {
                self
                    .query_embedding = value
                    .try_into()
                    .map_err(|e| {
                        format!(
                            "error converting supplied value for query_embedding: {}", e
                        )
                    });
                self
            }
            pub fn sql<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self
                    .sql = value
                    .try_into()
                    .map_err(|e| {
                        format!("error converting supplied value for sql: {}", e)
                    });
                self
            }
            pub fn text_search_query<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<Vec<String>>>,
                T::Error: std::fmt::Display,
            {
                self
                    .text_search_query = value
                    .try_into()
                    .map_err(|e| {
                        format!(
                            "error converting supplied value for text_search_query: {}",
                            e
                        )
                    });
                self
            }
            pub fn text_search_weight<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<f32>>,
                T::Error: std::fmt::Display,
            {
                self
                    .text_search_weight = value
                    .try_into()
                    .map_err(|e| {
                        format!(
                            "error converting supplied value for text_search_weight: {}",
                            e
                        )
                    });
                self
            }
            pub fn tokenizer_type<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<super::QueryRequestTokenizerType>>,
                T::Error: std::fmt::Display,
            {
                self
                    .tokenizer_type = value
                    .try_into()
                    .map_err(|e| {
                        format!(
                            "error converting supplied value for tokenizer_type: {}", e
                        )
                    });
                self
            }
            pub fn top_k<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<u64>>,
                T::Error: std::fmt::Display,
            {
                self
                    .top_k = value
                    .try_into()
                    .map_err(|e| {
                        format!("error converting supplied value for top_k: {}", e)
                    });
                self
            }
        }
        impl std::convert::TryFrom<QueryRequest> for super::QueryRequest {
            type Error = String;
            fn try_from(value: QueryRequest) -> Result<Self, String> {
                Ok(Self {
                    collection_id: value.collection_id?,
                    collection_name: value.collection_name?,
                    distance_metric: value.distance_metric?,
                    params: value.params?,
                    query_document_id: value.query_document_id?,
                    query_embedding: value.query_embedding?,
                    sql: value.sql?,
                    text_search_query: value.text_search_query?,
                    text_search_weight: value.text_search_weight?,
                    tokenizer_type: value.tokenizer_type?,
                    top_k: value.top_k?,
                })
            }
        }
        impl From<super::QueryRequest> for QueryRequest {
            fn from(value: super::QueryRequest) -> Self {
                Self {
                    collection_id: Ok(value.collection_id),
                    collection_name: Ok(value.collection_name),
                    distance_metric: Ok(value.distance_metric),
                    params: Ok(value.params),
                    query_document_id: Ok(value.query_document_id),
                    query_embedding: Ok(value.query_embedding),
                    sql: Ok(value.sql),
                    text_search_query: Ok(value.text_search_query),
                    text_search_weight: Ok(value.text_search_weight),
                    tokenizer_type: Ok(value.tokenizer_type),
                    top_k: Ok(value.top_k),
                }
            }
        }
        #[derive(Clone, Debug)]
        pub struct QueryRequestQueryEmbedding {
            dimensionality: Result<u64, String>,
            values: Result<Vec<f32>, String>,
        }
        impl Default for QueryRequestQueryEmbedding {
            fn default() -> Self {
                Self {
                    dimensionality: Err(
                        "no value supplied for dimensionality".to_string(),
                    ),
                    values: Err("no value supplied for values".to_string()),
                }
            }
        }
        impl QueryRequestQueryEmbedding {
            pub fn dimensionality<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<u64>,
                T::Error: std::fmt::Display,
            {
                self
                    .dimensionality = value
                    .try_into()
                    .map_err(|e| {
                        format!(
                            "error converting supplied value for dimensionality: {}", e
                        )
                    });
                self
            }
            pub fn values<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Vec<f32>>,
                T::Error: std::fmt::Display,
            {
                self
                    .values = value
                    .try_into()
                    .map_err(|e| {
                        format!("error converting supplied value for values: {}", e)
                    });
                self
            }
        }
        impl std::convert::TryFrom<QueryRequestQueryEmbedding>
        for super::QueryRequestQueryEmbedding {
            type Error = String;
            fn try_from(value: QueryRequestQueryEmbedding) -> Result<Self, String> {
                Ok(Self {
                    dimensionality: value.dimensionality?,
                    values: value.values?,
                })
            }
        }
        impl From<super::QueryRequestQueryEmbedding> for QueryRequestQueryEmbedding {
            fn from(value: super::QueryRequestQueryEmbedding) -> Self {
                Self {
                    dimensionality: Ok(value.dimensionality),
                    values: Ok(value.values),
                }
            }
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
    /**Sends a `POST` request to `/api/v1/query`

```ignore
let response = client.query_documents()
    .body(body)
    .send()
    .await;
```*/
    pub fn query_documents(&self) -> builder::QueryDocuments {
        builder::QueryDocuments::new(self)
    }
}
pub mod builder {
    use super::types;
    #[allow(unused_imports)]
    use super::{
        encode_path, ByteStream, Error, HeaderMap, HeaderValue, RequestBuilderExt,
        ResponseValue,
    };
    /**Builder for [`Client::query_documents`]

[`Client::query_documents`]: super::Client::query_documents*/
    #[derive(Debug, Clone)]
    pub struct QueryDocuments<'a> {
        client: &'a super::Client,
        body: Result<types::builder::QueryDocumentsBody, String>,
    }
    impl<'a> QueryDocuments<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                body: Ok(types::builder::QueryDocumentsBody::default()),
            }
        }
        pub fn body<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::QueryDocumentsBody>,
            <V as std::convert::TryInto<
                types::QueryDocumentsBody,
            >>::Error: std::fmt::Display,
        {
            self
                .body = value
                .try_into()
                .map(From::from)
                .map_err(|s| {
                    format!("conversion to `QueryDocumentsBody` for body failed: {}", s)
                });
            self
        }
        pub fn body_map<F>(mut self, f: F) -> Self
        where
            F: std::ops::FnOnce(
                types::builder::QueryDocumentsBody,
            ) -> types::builder::QueryDocumentsBody,
        {
            self.body = self.body.map(f);
            self
        }
        ///Sends a `POST` request to `/api/v1/query`
        pub async fn send(
            self,
        ) -> Result<ResponseValue<types::QueryDocumentsResponse>, Error<()>> {
            let Self { client, body } = self;
            let body = body
                .and_then(std::convert::TryInto::<types::QueryDocumentsBody>::try_into)
                .map_err(Error::InvalidRequest)?;
            let url = format!("{}/api/v1/query", client.baseurl,);
            let request = client
                .client
                .post(url)
                .header(
                    reqwest::header::ACCEPT,
                    reqwest::header::HeaderValue::from_static("application/json"),
                )
                .json(&body)
                .build()?;
            let result = client.client.execute(request).await;
            let response = result?;
            match response.status().as_u16() {
                200u16 => ResponseValue::from_response(response).await,
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }
}
pub mod prelude {
    pub use self::super::Client;
}
