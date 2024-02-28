pub use progenitor_client::{ByteStream, Error, ResponseValue};
#[allow(unused_imports)]
use progenitor_client::{encode_path, RequestBuilderExt};
#[allow(unused_imports)]
use reqwest::header::{HeaderMap, HeaderValue};
pub mod types {
    use serde::{Deserialize, Serialize};
    #[allow(unused_imports)]
    use std::convert::TryFrom;
    ///ChangeCollectionNameBody
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /**{
  "type": "object",
  "required": [
    "collection_id",
    "name"
  ],
  "properties": {
    "collection_id": {
      "type": "string",
      "format": "uuid"
    },
    "name": {
      "type": "string"
    }
  }
}*/
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct ChangeCollectionNameBody {
        pub collection_id: uuid::Uuid,
        pub name: String,
    }
    impl From<&ChangeCollectionNameBody> for ChangeCollectionNameBody {
        fn from(value: &ChangeCollectionNameBody) -> Self {
            value.clone()
        }
    }
    impl ChangeCollectionNameBody {
        pub fn builder() -> builder::ChangeCollectionNameBody {
            Default::default()
        }
    }
    ///ChangeCollectionNameResponse
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /**{
  "type": "object",
  "required": [
    "success"
  ],
  "properties": {
    "success": {
      "type": "boolean"
    }
  }
}*/
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct ChangeCollectionNameResponse {
        pub success: bool,
    }
    impl From<&ChangeCollectionNameResponse> for ChangeCollectionNameResponse {
        fn from(value: &ChangeCollectionNameResponse) -> Self {
            value.clone()
        }
    }
    impl ChangeCollectionNameResponse {
        pub fn builder() -> builder::ChangeCollectionNameResponse {
            Default::default()
        }
    }
    ///CreateCollectionBody
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /**{
  "type": "object",
  "required": [
    "dimensionality",
    "name"
  ],
  "properties": {
    "dimensionality": {
      "type": "integer",
      "format": "int32"
    },
    "name": {
      "type": "string"
    }
  }
}*/
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct CreateCollectionBody {
        pub dimensionality: i32,
        pub name: String,
    }
    impl From<&CreateCollectionBody> for CreateCollectionBody {
        fn from(value: &CreateCollectionBody) -> Self {
            value.clone()
        }
    }
    impl CreateCollectionBody {
        pub fn builder() -> builder::CreateCollectionBody {
            Default::default()
        }
    }
    ///CreateCollectionResponse
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /**{
  "type": "object",
  "required": [
    "dimensionality",
    "id",
    "name"
  ],
  "properties": {
    "dimensionality": {
      "type": "integer",
      "format": "int32"
    },
    "id": {
      "type": "string",
      "format": "uuid"
    },
    "name": {
      "type": "string"
    }
  }
}*/
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct CreateCollectionResponse {
        pub dimensionality: i32,
        pub id: uuid::Uuid,
        pub name: String,
    }
    impl From<&CreateCollectionResponse> for CreateCollectionResponse {
        fn from(value: &CreateCollectionResponse) -> Self {
            value.clone()
        }
    }
    impl CreateCollectionResponse {
        pub fn builder() -> builder::CreateCollectionResponse {
            Default::default()
        }
    }
    ///CreateDocumentsBody
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /**{
  "type": "object",
  "required": [
    "documents"
  ],
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
    "documents": {
      "type": "array",
      "items": {
        "type": "object",
        "required": [
          "embeddings",
          "metadata"
        ],
        "properties": {
          "embeddings": {
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
          },
          "metadata": {
            "type": "object"
          }
        }
      }
    }
  }
}*/
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct CreateDocumentsBody {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub collection_id: Option<uuid::Uuid>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub collection_name: Option<String>,
        pub documents: Vec<CreateDocumentsBodyDocumentsItem>,
    }
    impl From<&CreateDocumentsBody> for CreateDocumentsBody {
        fn from(value: &CreateDocumentsBody) -> Self {
            value.clone()
        }
    }
    impl CreateDocumentsBody {
        pub fn builder() -> builder::CreateDocumentsBody {
            Default::default()
        }
    }
    ///CreateDocumentsBodyDocumentsItem
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /**{
  "type": "object",
  "required": [
    "embeddings",
    "metadata"
  ],
  "properties": {
    "embeddings": {
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
    },
    "metadata": {
      "type": "object"
    }
  }
}*/
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct CreateDocumentsBodyDocumentsItem {
        pub embeddings: CreateDocumentsBodyDocumentsItemEmbeddings,
        pub metadata: serde_json::Map<String, serde_json::Value>,
    }
    impl From<&CreateDocumentsBodyDocumentsItem> for CreateDocumentsBodyDocumentsItem {
        fn from(value: &CreateDocumentsBodyDocumentsItem) -> Self {
            value.clone()
        }
    }
    impl CreateDocumentsBodyDocumentsItem {
        pub fn builder() -> builder::CreateDocumentsBodyDocumentsItem {
            Default::default()
        }
    }
    ///CreateDocumentsBodyDocumentsItemEmbeddings
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
    pub struct CreateDocumentsBodyDocumentsItemEmbeddings {
        pub dimensionality: u64,
        pub values: Vec<f32>,
    }
    impl From<&CreateDocumentsBodyDocumentsItemEmbeddings>
    for CreateDocumentsBodyDocumentsItemEmbeddings {
        fn from(value: &CreateDocumentsBodyDocumentsItemEmbeddings) -> Self {
            value.clone()
        }
    }
    impl CreateDocumentsBodyDocumentsItemEmbeddings {
        pub fn builder() -> builder::CreateDocumentsBodyDocumentsItemEmbeddings {
            Default::default()
        }
    }
    ///CreateDocumentsRequest
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /**{
  "type": "object",
  "required": [
    "documents"
  ],
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
    "documents": {
      "type": "array",
      "items": {
        "type": "object",
        "required": [
          "embeddings",
          "metadata"
        ],
        "properties": {
          "embeddings": {
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
          },
          "metadata": {
            "type": "object"
          }
        }
      }
    }
  }
}*/
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct CreateDocumentsRequest {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub collection_id: Option<uuid::Uuid>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub collection_name: Option<String>,
        pub documents: Vec<CreateDocumentsRequestDocumentsItem>,
    }
    impl From<&CreateDocumentsRequest> for CreateDocumentsRequest {
        fn from(value: &CreateDocumentsRequest) -> Self {
            value.clone()
        }
    }
    impl CreateDocumentsRequest {
        pub fn builder() -> builder::CreateDocumentsRequest {
            Default::default()
        }
    }
    ///CreateDocumentsRequestDocumentsItem
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /**{
  "type": "object",
  "required": [
    "embeddings",
    "metadata"
  ],
  "properties": {
    "embeddings": {
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
    },
    "metadata": {
      "type": "object"
    }
  }
}*/
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct CreateDocumentsRequestDocumentsItem {
        pub embeddings: CreateDocumentsRequestDocumentsItemEmbeddings,
        pub metadata: serde_json::Map<String, serde_json::Value>,
    }
    impl From<&CreateDocumentsRequestDocumentsItem>
    for CreateDocumentsRequestDocumentsItem {
        fn from(value: &CreateDocumentsRequestDocumentsItem) -> Self {
            value.clone()
        }
    }
    impl CreateDocumentsRequestDocumentsItem {
        pub fn builder() -> builder::CreateDocumentsRequestDocumentsItem {
            Default::default()
        }
    }
    ///CreateDocumentsRequestDocumentsItemEmbeddings
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
    pub struct CreateDocumentsRequestDocumentsItemEmbeddings {
        pub dimensionality: u64,
        pub values: Vec<f32>,
    }
    impl From<&CreateDocumentsRequestDocumentsItemEmbeddings>
    for CreateDocumentsRequestDocumentsItemEmbeddings {
        fn from(value: &CreateDocumentsRequestDocumentsItemEmbeddings) -> Self {
            value.clone()
        }
    }
    impl CreateDocumentsRequestDocumentsItemEmbeddings {
        pub fn builder() -> builder::CreateDocumentsRequestDocumentsItemEmbeddings {
            Default::default()
        }
    }
    ///CreateDocumentsResponse
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /**{
  "type": "object",
  "required": [
    "collection_id",
    "documents"
  ],
  "properties": {
    "collection_id": {
      "type": "string",
      "format": "uuid"
    },
    "documents": {
      "type": "array",
      "items": {
        "type": "object",
        "required": [
          "id"
        ],
        "properties": {
          "id": {
            "type": "string",
            "format": "uuid"
          }
        }
      }
    }
  }
}*/
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct CreateDocumentsResponse {
        pub collection_id: uuid::Uuid,
        pub documents: Vec<CreateDocumentsResponseDocumentsItem>,
    }
    impl From<&CreateDocumentsResponse> for CreateDocumentsResponse {
        fn from(value: &CreateDocumentsResponse) -> Self {
            value.clone()
        }
    }
    impl CreateDocumentsResponse {
        pub fn builder() -> builder::CreateDocumentsResponse {
            Default::default()
        }
    }
    ///CreateDocumentsResponseDocumentsItem
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /**{
  "type": "object",
  "required": [
    "id"
  ],
  "properties": {
    "id": {
      "type": "string",
      "format": "uuid"
    }
  }
}*/
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct CreateDocumentsResponseDocumentsItem {
        pub id: uuid::Uuid,
    }
    impl From<&CreateDocumentsResponseDocumentsItem>
    for CreateDocumentsResponseDocumentsItem {
        fn from(value: &CreateDocumentsResponseDocumentsItem) -> Self {
            value.clone()
        }
    }
    impl CreateDocumentsResponseDocumentsItem {
        pub fn builder() -> builder::CreateDocumentsResponseDocumentsItem {
            Default::default()
        }
    }
    ///DeleteCollectionBody
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /**{
  "type": "object",
  "required": [
    "collection_id"
  ],
  "properties": {
    "collection_id": {
      "type": "string",
      "format": "uuid"
    }
  }
}*/
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct DeleteCollectionBody {
        pub collection_id: uuid::Uuid,
    }
    impl From<&DeleteCollectionBody> for DeleteCollectionBody {
        fn from(value: &DeleteCollectionBody) -> Self {
            value.clone()
        }
    }
    impl DeleteCollectionBody {
        pub fn builder() -> builder::DeleteCollectionBody {
            Default::default()
        }
    }
    ///DeleteCollectionResponse
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /**{
  "type": "object",
  "required": [
    "success"
  ],
  "properties": {
    "success": {
      "type": "boolean"
    }
  }
}*/
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct DeleteCollectionResponse {
        pub success: bool,
    }
    impl From<&DeleteCollectionResponse> for DeleteCollectionResponse {
        fn from(value: &DeleteCollectionResponse) -> Self {
            value.clone()
        }
    }
    impl DeleteCollectionResponse {
        pub fn builder() -> builder::DeleteCollectionResponse {
            Default::default()
        }
    }
    ///DeleteDocumentsBody
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /**{
  "type": "object",
  "required": [
    "documents"
  ],
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
    "documents": {
      "type": "array",
      "items": {
        "type": "string"
      }
    }
  }
}*/
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct DeleteDocumentsBody {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub collection_id: Option<uuid::Uuid>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub collection_name: Option<String>,
        pub documents: Vec<String>,
    }
    impl From<&DeleteDocumentsBody> for DeleteDocumentsBody {
        fn from(value: &DeleteDocumentsBody) -> Self {
            value.clone()
        }
    }
    impl DeleteDocumentsBody {
        pub fn builder() -> builder::DeleteDocumentsBody {
            Default::default()
        }
    }
    ///DeleteDocumentsResponse
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /**{
  "type": "object",
  "required": [
    "collection_id",
    "document_ids"
  ],
  "properties": {
    "collection_id": {
      "type": "string",
      "format": "uuid"
    },
    "document_ids": {
      "type": "array",
      "items": {
        "type": "string"
      }
    }
  }
}*/
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct DeleteDocumentsResponse {
        pub collection_id: uuid::Uuid,
        pub document_ids: Vec<String>,
    }
    impl From<&DeleteDocumentsResponse> for DeleteDocumentsResponse {
        fn from(value: &DeleteDocumentsResponse) -> Self {
            value.clone()
        }
    }
    impl DeleteDocumentsResponse {
        pub fn builder() -> builder::DeleteDocumentsResponse {
            Default::default()
        }
    }
    ///GetCollectionsResponse
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /**{
  "type": "object",
  "required": [
    "collections"
  ],
  "properties": {
    "collections": {
      "type": "array",
      "items": {
        "type": "object",
        "required": [
          "dimensionality",
          "document_count",
          "id",
          "name",
          "organization_id",
          "page_size_bytes"
        ],
        "properties": {
          "dimensionality": {
            "type": "integer",
            "format": "int32"
          },
          "document_count": {
            "type": "integer",
            "format": "int32"
          },
          "id": {
            "type": "string",
            "format": "uuid"
          },
          "name": {
            "type": "string"
          },
          "organization_id": {
            "type": "string",
            "format": "uuid"
          },
          "page_size_bytes": {
            "type": "integer",
            "format": "int32"
          }
        }
      }
    }
  }
}*/
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct GetCollectionsResponse {
        pub collections: Vec<GetCollectionsResponseCollectionsItem>,
    }
    impl From<&GetCollectionsResponse> for GetCollectionsResponse {
        fn from(value: &GetCollectionsResponse) -> Self {
            value.clone()
        }
    }
    impl GetCollectionsResponse {
        pub fn builder() -> builder::GetCollectionsResponse {
            Default::default()
        }
    }
    ///GetCollectionsResponseCollectionsItem
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /**{
  "type": "object",
  "required": [
    "dimensionality",
    "document_count",
    "id",
    "name",
    "organization_id",
    "page_size_bytes"
  ],
  "properties": {
    "dimensionality": {
      "type": "integer",
      "format": "int32"
    },
    "document_count": {
      "type": "integer",
      "format": "int32"
    },
    "id": {
      "type": "string",
      "format": "uuid"
    },
    "name": {
      "type": "string"
    },
    "organization_id": {
      "type": "string",
      "format": "uuid"
    },
    "page_size_bytes": {
      "type": "integer",
      "format": "int32"
    }
  }
}*/
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct GetCollectionsResponseCollectionsItem {
        pub dimensionality: i32,
        pub document_count: i32,
        pub id: uuid::Uuid,
        pub name: String,
        pub organization_id: uuid::Uuid,
        pub page_size_bytes: i32,
    }
    impl From<&GetCollectionsResponseCollectionsItem>
    for GetCollectionsResponseCollectionsItem {
        fn from(value: &GetCollectionsResponseCollectionsItem) -> Self {
            value.clone()
        }
    }
    impl GetCollectionsResponseCollectionsItem {
        pub fn builder() -> builder::GetCollectionsResponseCollectionsItem {
            Default::default()
        }
    }
    pub mod builder {
        #[derive(Clone, Debug)]
        pub struct ChangeCollectionNameBody {
            collection_id: Result<uuid::Uuid, String>,
            name: Result<String, String>,
        }
        impl Default for ChangeCollectionNameBody {
            fn default() -> Self {
                Self {
                    collection_id: Err(
                        "no value supplied for collection_id".to_string(),
                    ),
                    name: Err("no value supplied for name".to_string()),
                }
            }
        }
        impl ChangeCollectionNameBody {
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
            pub fn name<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<String>,
                T::Error: std::fmt::Display,
            {
                self
                    .name = value
                    .try_into()
                    .map_err(|e| {
                        format!("error converting supplied value for name: {}", e)
                    });
                self
            }
        }
        impl std::convert::TryFrom<ChangeCollectionNameBody>
        for super::ChangeCollectionNameBody {
            type Error = String;
            fn try_from(value: ChangeCollectionNameBody) -> Result<Self, String> {
                Ok(Self {
                    collection_id: value.collection_id?,
                    name: value.name?,
                })
            }
        }
        impl From<super::ChangeCollectionNameBody> for ChangeCollectionNameBody {
            fn from(value: super::ChangeCollectionNameBody) -> Self {
                Self {
                    collection_id: Ok(value.collection_id),
                    name: Ok(value.name),
                }
            }
        }
        #[derive(Clone, Debug)]
        pub struct ChangeCollectionNameResponse {
            success: Result<bool, String>,
        }
        impl Default for ChangeCollectionNameResponse {
            fn default() -> Self {
                Self {
                    success: Err("no value supplied for success".to_string()),
                }
            }
        }
        impl ChangeCollectionNameResponse {
            pub fn success<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<bool>,
                T::Error: std::fmt::Display,
            {
                self
                    .success = value
                    .try_into()
                    .map_err(|e| {
                        format!("error converting supplied value for success: {}", e)
                    });
                self
            }
        }
        impl std::convert::TryFrom<ChangeCollectionNameResponse>
        for super::ChangeCollectionNameResponse {
            type Error = String;
            fn try_from(value: ChangeCollectionNameResponse) -> Result<Self, String> {
                Ok(Self { success: value.success? })
            }
        }
        impl From<super::ChangeCollectionNameResponse> for ChangeCollectionNameResponse {
            fn from(value: super::ChangeCollectionNameResponse) -> Self {
                Self { success: Ok(value.success) }
            }
        }
        #[derive(Clone, Debug)]
        pub struct CreateCollectionBody {
            dimensionality: Result<i32, String>,
            name: Result<String, String>,
        }
        impl Default for CreateCollectionBody {
            fn default() -> Self {
                Self {
                    dimensionality: Err(
                        "no value supplied for dimensionality".to_string(),
                    ),
                    name: Err("no value supplied for name".to_string()),
                }
            }
        }
        impl CreateCollectionBody {
            pub fn dimensionality<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<i32>,
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
            pub fn name<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<String>,
                T::Error: std::fmt::Display,
            {
                self
                    .name = value
                    .try_into()
                    .map_err(|e| {
                        format!("error converting supplied value for name: {}", e)
                    });
                self
            }
        }
        impl std::convert::TryFrom<CreateCollectionBody>
        for super::CreateCollectionBody {
            type Error = String;
            fn try_from(value: CreateCollectionBody) -> Result<Self, String> {
                Ok(Self {
                    dimensionality: value.dimensionality?,
                    name: value.name?,
                })
            }
        }
        impl From<super::CreateCollectionBody> for CreateCollectionBody {
            fn from(value: super::CreateCollectionBody) -> Self {
                Self {
                    dimensionality: Ok(value.dimensionality),
                    name: Ok(value.name),
                }
            }
        }
        #[derive(Clone, Debug)]
        pub struct CreateCollectionResponse {
            dimensionality: Result<i32, String>,
            id: Result<uuid::Uuid, String>,
            name: Result<String, String>,
        }
        impl Default for CreateCollectionResponse {
            fn default() -> Self {
                Self {
                    dimensionality: Err(
                        "no value supplied for dimensionality".to_string(),
                    ),
                    id: Err("no value supplied for id".to_string()),
                    name: Err("no value supplied for name".to_string()),
                }
            }
        }
        impl CreateCollectionResponse {
            pub fn dimensionality<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<i32>,
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
            pub fn id<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<uuid::Uuid>,
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
            pub fn name<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<String>,
                T::Error: std::fmt::Display,
            {
                self
                    .name = value
                    .try_into()
                    .map_err(|e| {
                        format!("error converting supplied value for name: {}", e)
                    });
                self
            }
        }
        impl std::convert::TryFrom<CreateCollectionResponse>
        for super::CreateCollectionResponse {
            type Error = String;
            fn try_from(value: CreateCollectionResponse) -> Result<Self, String> {
                Ok(Self {
                    dimensionality: value.dimensionality?,
                    id: value.id?,
                    name: value.name?,
                })
            }
        }
        impl From<super::CreateCollectionResponse> for CreateCollectionResponse {
            fn from(value: super::CreateCollectionResponse) -> Self {
                Self {
                    dimensionality: Ok(value.dimensionality),
                    id: Ok(value.id),
                    name: Ok(value.name),
                }
            }
        }
        #[derive(Clone, Debug)]
        pub struct CreateDocumentsBody {
            collection_id: Result<Option<uuid::Uuid>, String>,
            collection_name: Result<Option<String>, String>,
            documents: Result<Vec<super::CreateDocumentsBodyDocumentsItem>, String>,
        }
        impl Default for CreateDocumentsBody {
            fn default() -> Self {
                Self {
                    collection_id: Ok(Default::default()),
                    collection_name: Ok(Default::default()),
                    documents: Err("no value supplied for documents".to_string()),
                }
            }
        }
        impl CreateDocumentsBody {
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
            pub fn documents<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Vec<super::CreateDocumentsBodyDocumentsItem>>,
                T::Error: std::fmt::Display,
            {
                self
                    .documents = value
                    .try_into()
                    .map_err(|e| {
                        format!("error converting supplied value for documents: {}", e)
                    });
                self
            }
        }
        impl std::convert::TryFrom<CreateDocumentsBody> for super::CreateDocumentsBody {
            type Error = String;
            fn try_from(value: CreateDocumentsBody) -> Result<Self, String> {
                Ok(Self {
                    collection_id: value.collection_id?,
                    collection_name: value.collection_name?,
                    documents: value.documents?,
                })
            }
        }
        impl From<super::CreateDocumentsBody> for CreateDocumentsBody {
            fn from(value: super::CreateDocumentsBody) -> Self {
                Self {
                    collection_id: Ok(value.collection_id),
                    collection_name: Ok(value.collection_name),
                    documents: Ok(value.documents),
                }
            }
        }
        #[derive(Clone, Debug)]
        pub struct CreateDocumentsBodyDocumentsItem {
            embeddings: Result<
                super::CreateDocumentsBodyDocumentsItemEmbeddings,
                String,
            >,
            metadata: Result<serde_json::Map<String, serde_json::Value>, String>,
        }
        impl Default for CreateDocumentsBodyDocumentsItem {
            fn default() -> Self {
                Self {
                    embeddings: Err("no value supplied for embeddings".to_string()),
                    metadata: Err("no value supplied for metadata".to_string()),
                }
            }
        }
        impl CreateDocumentsBodyDocumentsItem {
            pub fn embeddings<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<
                    super::CreateDocumentsBodyDocumentsItemEmbeddings,
                >,
                T::Error: std::fmt::Display,
            {
                self
                    .embeddings = value
                    .try_into()
                    .map_err(|e| {
                        format!("error converting supplied value for embeddings: {}", e)
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
        }
        impl std::convert::TryFrom<CreateDocumentsBodyDocumentsItem>
        for super::CreateDocumentsBodyDocumentsItem {
            type Error = String;
            fn try_from(
                value: CreateDocumentsBodyDocumentsItem,
            ) -> Result<Self, String> {
                Ok(Self {
                    embeddings: value.embeddings?,
                    metadata: value.metadata?,
                })
            }
        }
        impl From<super::CreateDocumentsBodyDocumentsItem>
        for CreateDocumentsBodyDocumentsItem {
            fn from(value: super::CreateDocumentsBodyDocumentsItem) -> Self {
                Self {
                    embeddings: Ok(value.embeddings),
                    metadata: Ok(value.metadata),
                }
            }
        }
        #[derive(Clone, Debug)]
        pub struct CreateDocumentsBodyDocumentsItemEmbeddings {
            dimensionality: Result<u64, String>,
            values: Result<Vec<f32>, String>,
        }
        impl Default for CreateDocumentsBodyDocumentsItemEmbeddings {
            fn default() -> Self {
                Self {
                    dimensionality: Err(
                        "no value supplied for dimensionality".to_string(),
                    ),
                    values: Err("no value supplied for values".to_string()),
                }
            }
        }
        impl CreateDocumentsBodyDocumentsItemEmbeddings {
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
        impl std::convert::TryFrom<CreateDocumentsBodyDocumentsItemEmbeddings>
        for super::CreateDocumentsBodyDocumentsItemEmbeddings {
            type Error = String;
            fn try_from(
                value: CreateDocumentsBodyDocumentsItemEmbeddings,
            ) -> Result<Self, String> {
                Ok(Self {
                    dimensionality: value.dimensionality?,
                    values: value.values?,
                })
            }
        }
        impl From<super::CreateDocumentsBodyDocumentsItemEmbeddings>
        for CreateDocumentsBodyDocumentsItemEmbeddings {
            fn from(value: super::CreateDocumentsBodyDocumentsItemEmbeddings) -> Self {
                Self {
                    dimensionality: Ok(value.dimensionality),
                    values: Ok(value.values),
                }
            }
        }
        #[derive(Clone, Debug)]
        pub struct CreateDocumentsRequest {
            collection_id: Result<Option<uuid::Uuid>, String>,
            collection_name: Result<Option<String>, String>,
            documents: Result<Vec<super::CreateDocumentsRequestDocumentsItem>, String>,
        }
        impl Default for CreateDocumentsRequest {
            fn default() -> Self {
                Self {
                    collection_id: Ok(Default::default()),
                    collection_name: Ok(Default::default()),
                    documents: Err("no value supplied for documents".to_string()),
                }
            }
        }
        impl CreateDocumentsRequest {
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
            pub fn documents<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<
                    Vec<super::CreateDocumentsRequestDocumentsItem>,
                >,
                T::Error: std::fmt::Display,
            {
                self
                    .documents = value
                    .try_into()
                    .map_err(|e| {
                        format!("error converting supplied value for documents: {}", e)
                    });
                self
            }
        }
        impl std::convert::TryFrom<CreateDocumentsRequest>
        for super::CreateDocumentsRequest {
            type Error = String;
            fn try_from(value: CreateDocumentsRequest) -> Result<Self, String> {
                Ok(Self {
                    collection_id: value.collection_id?,
                    collection_name: value.collection_name?,
                    documents: value.documents?,
                })
            }
        }
        impl From<super::CreateDocumentsRequest> for CreateDocumentsRequest {
            fn from(value: super::CreateDocumentsRequest) -> Self {
                Self {
                    collection_id: Ok(value.collection_id),
                    collection_name: Ok(value.collection_name),
                    documents: Ok(value.documents),
                }
            }
        }
        #[derive(Clone, Debug)]
        pub struct CreateDocumentsRequestDocumentsItem {
            embeddings: Result<
                super::CreateDocumentsRequestDocumentsItemEmbeddings,
                String,
            >,
            metadata: Result<serde_json::Map<String, serde_json::Value>, String>,
        }
        impl Default for CreateDocumentsRequestDocumentsItem {
            fn default() -> Self {
                Self {
                    embeddings: Err("no value supplied for embeddings".to_string()),
                    metadata: Err("no value supplied for metadata".to_string()),
                }
            }
        }
        impl CreateDocumentsRequestDocumentsItem {
            pub fn embeddings<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<
                    super::CreateDocumentsRequestDocumentsItemEmbeddings,
                >,
                T::Error: std::fmt::Display,
            {
                self
                    .embeddings = value
                    .try_into()
                    .map_err(|e| {
                        format!("error converting supplied value for embeddings: {}", e)
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
        }
        impl std::convert::TryFrom<CreateDocumentsRequestDocumentsItem>
        for super::CreateDocumentsRequestDocumentsItem {
            type Error = String;
            fn try_from(
                value: CreateDocumentsRequestDocumentsItem,
            ) -> Result<Self, String> {
                Ok(Self {
                    embeddings: value.embeddings?,
                    metadata: value.metadata?,
                })
            }
        }
        impl From<super::CreateDocumentsRequestDocumentsItem>
        for CreateDocumentsRequestDocumentsItem {
            fn from(value: super::CreateDocumentsRequestDocumentsItem) -> Self {
                Self {
                    embeddings: Ok(value.embeddings),
                    metadata: Ok(value.metadata),
                }
            }
        }
        #[derive(Clone, Debug)]
        pub struct CreateDocumentsRequestDocumentsItemEmbeddings {
            dimensionality: Result<u64, String>,
            values: Result<Vec<f32>, String>,
        }
        impl Default for CreateDocumentsRequestDocumentsItemEmbeddings {
            fn default() -> Self {
                Self {
                    dimensionality: Err(
                        "no value supplied for dimensionality".to_string(),
                    ),
                    values: Err("no value supplied for values".to_string()),
                }
            }
        }
        impl CreateDocumentsRequestDocumentsItemEmbeddings {
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
        impl std::convert::TryFrom<CreateDocumentsRequestDocumentsItemEmbeddings>
        for super::CreateDocumentsRequestDocumentsItemEmbeddings {
            type Error = String;
            fn try_from(
                value: CreateDocumentsRequestDocumentsItemEmbeddings,
            ) -> Result<Self, String> {
                Ok(Self {
                    dimensionality: value.dimensionality?,
                    values: value.values?,
                })
            }
        }
        impl From<super::CreateDocumentsRequestDocumentsItemEmbeddings>
        for CreateDocumentsRequestDocumentsItemEmbeddings {
            fn from(
                value: super::CreateDocumentsRequestDocumentsItemEmbeddings,
            ) -> Self {
                Self {
                    dimensionality: Ok(value.dimensionality),
                    values: Ok(value.values),
                }
            }
        }
        #[derive(Clone, Debug)]
        pub struct CreateDocumentsResponse {
            collection_id: Result<uuid::Uuid, String>,
            documents: Result<Vec<super::CreateDocumentsResponseDocumentsItem>, String>,
        }
        impl Default for CreateDocumentsResponse {
            fn default() -> Self {
                Self {
                    collection_id: Err(
                        "no value supplied for collection_id".to_string(),
                    ),
                    documents: Err("no value supplied for documents".to_string()),
                }
            }
        }
        impl CreateDocumentsResponse {
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
            pub fn documents<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<
                    Vec<super::CreateDocumentsResponseDocumentsItem>,
                >,
                T::Error: std::fmt::Display,
            {
                self
                    .documents = value
                    .try_into()
                    .map_err(|e| {
                        format!("error converting supplied value for documents: {}", e)
                    });
                self
            }
        }
        impl std::convert::TryFrom<CreateDocumentsResponse>
        for super::CreateDocumentsResponse {
            type Error = String;
            fn try_from(value: CreateDocumentsResponse) -> Result<Self, String> {
                Ok(Self {
                    collection_id: value.collection_id?,
                    documents: value.documents?,
                })
            }
        }
        impl From<super::CreateDocumentsResponse> for CreateDocumentsResponse {
            fn from(value: super::CreateDocumentsResponse) -> Self {
                Self {
                    collection_id: Ok(value.collection_id),
                    documents: Ok(value.documents),
                }
            }
        }
        #[derive(Clone, Debug)]
        pub struct CreateDocumentsResponseDocumentsItem {
            id: Result<uuid::Uuid, String>,
        }
        impl Default for CreateDocumentsResponseDocumentsItem {
            fn default() -> Self {
                Self {
                    id: Err("no value supplied for id".to_string()),
                }
            }
        }
        impl CreateDocumentsResponseDocumentsItem {
            pub fn id<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<uuid::Uuid>,
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
        }
        impl std::convert::TryFrom<CreateDocumentsResponseDocumentsItem>
        for super::CreateDocumentsResponseDocumentsItem {
            type Error = String;
            fn try_from(
                value: CreateDocumentsResponseDocumentsItem,
            ) -> Result<Self, String> {
                Ok(Self { id: value.id? })
            }
        }
        impl From<super::CreateDocumentsResponseDocumentsItem>
        for CreateDocumentsResponseDocumentsItem {
            fn from(value: super::CreateDocumentsResponseDocumentsItem) -> Self {
                Self { id: Ok(value.id) }
            }
        }
        #[derive(Clone, Debug)]
        pub struct DeleteCollectionBody {
            collection_id: Result<uuid::Uuid, String>,
        }
        impl Default for DeleteCollectionBody {
            fn default() -> Self {
                Self {
                    collection_id: Err("no value supplied for collection_id".to_string()),
                }
            }
        }
        impl DeleteCollectionBody {
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
        }
        impl std::convert::TryFrom<DeleteCollectionBody>
        for super::DeleteCollectionBody {
            type Error = String;
            fn try_from(value: DeleteCollectionBody) -> Result<Self, String> {
                Ok(Self {
                    collection_id: value.collection_id?,
                })
            }
        }
        impl From<super::DeleteCollectionBody> for DeleteCollectionBody {
            fn from(value: super::DeleteCollectionBody) -> Self {
                Self {
                    collection_id: Ok(value.collection_id),
                }
            }
        }
        #[derive(Clone, Debug)]
        pub struct DeleteCollectionResponse {
            success: Result<bool, String>,
        }
        impl Default for DeleteCollectionResponse {
            fn default() -> Self {
                Self {
                    success: Err("no value supplied for success".to_string()),
                }
            }
        }
        impl DeleteCollectionResponse {
            pub fn success<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<bool>,
                T::Error: std::fmt::Display,
            {
                self
                    .success = value
                    .try_into()
                    .map_err(|e| {
                        format!("error converting supplied value for success: {}", e)
                    });
                self
            }
        }
        impl std::convert::TryFrom<DeleteCollectionResponse>
        for super::DeleteCollectionResponse {
            type Error = String;
            fn try_from(value: DeleteCollectionResponse) -> Result<Self, String> {
                Ok(Self { success: value.success? })
            }
        }
        impl From<super::DeleteCollectionResponse> for DeleteCollectionResponse {
            fn from(value: super::DeleteCollectionResponse) -> Self {
                Self { success: Ok(value.success) }
            }
        }
        #[derive(Clone, Debug)]
        pub struct DeleteDocumentsBody {
            collection_id: Result<Option<uuid::Uuid>, String>,
            collection_name: Result<Option<String>, String>,
            documents: Result<Vec<String>, String>,
        }
        impl Default for DeleteDocumentsBody {
            fn default() -> Self {
                Self {
                    collection_id: Ok(Default::default()),
                    collection_name: Ok(Default::default()),
                    documents: Err("no value supplied for documents".to_string()),
                }
            }
        }
        impl DeleteDocumentsBody {
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
            pub fn documents<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Vec<String>>,
                T::Error: std::fmt::Display,
            {
                self
                    .documents = value
                    .try_into()
                    .map_err(|e| {
                        format!("error converting supplied value for documents: {}", e)
                    });
                self
            }
        }
        impl std::convert::TryFrom<DeleteDocumentsBody> for super::DeleteDocumentsBody {
            type Error = String;
            fn try_from(value: DeleteDocumentsBody) -> Result<Self, String> {
                Ok(Self {
                    collection_id: value.collection_id?,
                    collection_name: value.collection_name?,
                    documents: value.documents?,
                })
            }
        }
        impl From<super::DeleteDocumentsBody> for DeleteDocumentsBody {
            fn from(value: super::DeleteDocumentsBody) -> Self {
                Self {
                    collection_id: Ok(value.collection_id),
                    collection_name: Ok(value.collection_name),
                    documents: Ok(value.documents),
                }
            }
        }
        #[derive(Clone, Debug)]
        pub struct DeleteDocumentsResponse {
            collection_id: Result<uuid::Uuid, String>,
            document_ids: Result<Vec<String>, String>,
        }
        impl Default for DeleteDocumentsResponse {
            fn default() -> Self {
                Self {
                    collection_id: Err(
                        "no value supplied for collection_id".to_string(),
                    ),
                    document_ids: Err("no value supplied for document_ids".to_string()),
                }
            }
        }
        impl DeleteDocumentsResponse {
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
            pub fn document_ids<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Vec<String>>,
                T::Error: std::fmt::Display,
            {
                self
                    .document_ids = value
                    .try_into()
                    .map_err(|e| {
                        format!(
                            "error converting supplied value for document_ids: {}", e
                        )
                    });
                self
            }
        }
        impl std::convert::TryFrom<DeleteDocumentsResponse>
        for super::DeleteDocumentsResponse {
            type Error = String;
            fn try_from(value: DeleteDocumentsResponse) -> Result<Self, String> {
                Ok(Self {
                    collection_id: value.collection_id?,
                    document_ids: value.document_ids?,
                })
            }
        }
        impl From<super::DeleteDocumentsResponse> for DeleteDocumentsResponse {
            fn from(value: super::DeleteDocumentsResponse) -> Self {
                Self {
                    collection_id: Ok(value.collection_id),
                    document_ids: Ok(value.document_ids),
                }
            }
        }
        #[derive(Clone, Debug)]
        pub struct GetCollectionsResponse {
            collections: Result<
                Vec<super::GetCollectionsResponseCollectionsItem>,
                String,
            >,
        }
        impl Default for GetCollectionsResponse {
            fn default() -> Self {
                Self {
                    collections: Err("no value supplied for collections".to_string()),
                }
            }
        }
        impl GetCollectionsResponse {
            pub fn collections<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<
                    Vec<super::GetCollectionsResponseCollectionsItem>,
                >,
                T::Error: std::fmt::Display,
            {
                self
                    .collections = value
                    .try_into()
                    .map_err(|e| {
                        format!("error converting supplied value for collections: {}", e)
                    });
                self
            }
        }
        impl std::convert::TryFrom<GetCollectionsResponse>
        for super::GetCollectionsResponse {
            type Error = String;
            fn try_from(value: GetCollectionsResponse) -> Result<Self, String> {
                Ok(Self {
                    collections: value.collections?,
                })
            }
        }
        impl From<super::GetCollectionsResponse> for GetCollectionsResponse {
            fn from(value: super::GetCollectionsResponse) -> Self {
                Self {
                    collections: Ok(value.collections),
                }
            }
        }
        #[derive(Clone, Debug)]
        pub struct GetCollectionsResponseCollectionsItem {
            dimensionality: Result<i32, String>,
            document_count: Result<i32, String>,
            id: Result<uuid::Uuid, String>,
            name: Result<String, String>,
            organization_id: Result<uuid::Uuid, String>,
            page_size_bytes: Result<i32, String>,
        }
        impl Default for GetCollectionsResponseCollectionsItem {
            fn default() -> Self {
                Self {
                    dimensionality: Err(
                        "no value supplied for dimensionality".to_string(),
                    ),
                    document_count: Err(
                        "no value supplied for document_count".to_string(),
                    ),
                    id: Err("no value supplied for id".to_string()),
                    name: Err("no value supplied for name".to_string()),
                    organization_id: Err(
                        "no value supplied for organization_id".to_string(),
                    ),
                    page_size_bytes: Err(
                        "no value supplied for page_size_bytes".to_string(),
                    ),
                }
            }
        }
        impl GetCollectionsResponseCollectionsItem {
            pub fn dimensionality<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<i32>,
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
            pub fn document_count<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<i32>,
                T::Error: std::fmt::Display,
            {
                self
                    .document_count = value
                    .try_into()
                    .map_err(|e| {
                        format!(
                            "error converting supplied value for document_count: {}", e
                        )
                    });
                self
            }
            pub fn id<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<uuid::Uuid>,
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
            pub fn name<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<String>,
                T::Error: std::fmt::Display,
            {
                self
                    .name = value
                    .try_into()
                    .map_err(|e| {
                        format!("error converting supplied value for name: {}", e)
                    });
                self
            }
            pub fn organization_id<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<uuid::Uuid>,
                T::Error: std::fmt::Display,
            {
                self
                    .organization_id = value
                    .try_into()
                    .map_err(|e| {
                        format!(
                            "error converting supplied value for organization_id: {}", e
                        )
                    });
                self
            }
            pub fn page_size_bytes<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<i32>,
                T::Error: std::fmt::Display,
            {
                self
                    .page_size_bytes = value
                    .try_into()
                    .map_err(|e| {
                        format!(
                            "error converting supplied value for page_size_bytes: {}", e
                        )
                    });
                self
            }
        }
        impl std::convert::TryFrom<GetCollectionsResponseCollectionsItem>
        for super::GetCollectionsResponseCollectionsItem {
            type Error = String;
            fn try_from(
                value: GetCollectionsResponseCollectionsItem,
            ) -> Result<Self, String> {
                Ok(Self {
                    dimensionality: value.dimensionality?,
                    document_count: value.document_count?,
                    id: value.id?,
                    name: value.name?,
                    organization_id: value.organization_id?,
                    page_size_bytes: value.page_size_bytes?,
                })
            }
        }
        impl From<super::GetCollectionsResponseCollectionsItem>
        for GetCollectionsResponseCollectionsItem {
            fn from(value: super::GetCollectionsResponseCollectionsItem) -> Self {
                Self {
                    dimensionality: Ok(value.dimensionality),
                    document_count: Ok(value.document_count),
                    id: Ok(value.id),
                    name: Ok(value.name),
                    organization_id: Ok(value.organization_id),
                    page_size_bytes: Ok(value.page_size_bytes),
                }
            }
        }
    }
}
#[derive(Clone, Debug)]
/**Client for warden



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
    /**Sends a `GET` request to `/api/v1/collections`

```ignore
let response = client.get_collections()
    .send()
    .await;
```*/
    pub fn get_collections(&self) -> builder::GetCollections {
        builder::GetCollections::new(self)
    }
    /**Sends a `PUT` request to `/api/v1/collections`

```ignore
let response = client.change_collection_name()
    .body(body)
    .send()
    .await;
```*/
    pub fn change_collection_name(&self) -> builder::ChangeCollectionName {
        builder::ChangeCollectionName::new(self)
    }
    /**Sends a `POST` request to `/api/v1/collections`

```ignore
let response = client.create_collection()
    .body(body)
    .send()
    .await;
```*/
    pub fn create_collection(&self) -> builder::CreateCollection {
        builder::CreateCollection::new(self)
    }
    /**Sends a `DELETE` request to `/api/v1/collections`

```ignore
let response = client.delete_collection()
    .body(body)
    .send()
    .await;
```*/
    pub fn delete_collection(&self) -> builder::DeleteCollection {
        builder::DeleteCollection::new(self)
    }
    /**Sends a `POST` request to `/api/v1/documents`

```ignore
let response = client.create_documents()
    .body(body)
    .send()
    .await;
```*/
    pub fn create_documents(&self) -> builder::CreateDocuments {
        builder::CreateDocuments::new(self)
    }
    /**Sends a `PATCH` request to `/api/v1/documents`

```ignore
let response = client.delete_documents()
    .body(body)
    .send()
    .await;
```*/
    pub fn delete_documents(&self) -> builder::DeleteDocuments {
        builder::DeleteDocuments::new(self)
    }
}
pub mod builder {
    use super::types;
    #[allow(unused_imports)]
    use super::{
        encode_path, ByteStream, Error, HeaderMap, HeaderValue, RequestBuilderExt,
        ResponseValue,
    };
    /**Builder for [`Client::get_collections`]

[`Client::get_collections`]: super::Client::get_collections*/
    #[derive(Debug, Clone)]
    pub struct GetCollections<'a> {
        client: &'a super::Client,
    }
    impl<'a> GetCollections<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self { client: client }
        }
        ///Sends a `GET` request to `/api/v1/collections`
        pub async fn send(
            self,
        ) -> Result<ResponseValue<types::GetCollectionsResponse>, Error<()>> {
            let Self { client } = self;
            let url = format!("{}/api/v1/collections", client.baseurl,);
            let request = client
                .client
                .get(url)
                .header(
                    reqwest::header::ACCEPT,
                    reqwest::header::HeaderValue::from_static("application/json"),
                )
                .build()?;
            let result = client.client.execute(request).await;
            let response = result?;
            match response.status().as_u16() {
                200u16 => ResponseValue::from_response(response).await,
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }
    /**Builder for [`Client::change_collection_name`]

[`Client::change_collection_name`]: super::Client::change_collection_name*/
    #[derive(Debug, Clone)]
    pub struct ChangeCollectionName<'a> {
        client: &'a super::Client,
        body: Result<types::builder::ChangeCollectionNameBody, String>,
    }
    impl<'a> ChangeCollectionName<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                body: Ok(types::builder::ChangeCollectionNameBody::default()),
            }
        }
        pub fn body<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::ChangeCollectionNameBody>,
            <V as std::convert::TryInto<
                types::ChangeCollectionNameBody,
            >>::Error: std::fmt::Display,
        {
            self
                .body = value
                .try_into()
                .map(From::from)
                .map_err(|s| {
                    format!(
                        "conversion to `ChangeCollectionNameBody` for body failed: {}", s
                    )
                });
            self
        }
        pub fn body_map<F>(mut self, f: F) -> Self
        where
            F: std::ops::FnOnce(
                types::builder::ChangeCollectionNameBody,
            ) -> types::builder::ChangeCollectionNameBody,
        {
            self.body = self.body.map(f);
            self
        }
        ///Sends a `PUT` request to `/api/v1/collections`
        pub async fn send(
            self,
        ) -> Result<ResponseValue<types::ChangeCollectionNameResponse>, Error<()>> {
            let Self { client, body } = self;
            let body = body
                .and_then(
                    std::convert::TryInto::<types::ChangeCollectionNameBody>::try_into,
                )
                .map_err(Error::InvalidRequest)?;
            let url = format!("{}/api/v1/collections", client.baseurl,);
            let request = client
                .client
                .put(url)
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
    /**Builder for [`Client::create_collection`]

[`Client::create_collection`]: super::Client::create_collection*/
    #[derive(Debug, Clone)]
    pub struct CreateCollection<'a> {
        client: &'a super::Client,
        body: Result<types::builder::CreateCollectionBody, String>,
    }
    impl<'a> CreateCollection<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                body: Ok(types::builder::CreateCollectionBody::default()),
            }
        }
        pub fn body<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::CreateCollectionBody>,
            <V as std::convert::TryInto<
                types::CreateCollectionBody,
            >>::Error: std::fmt::Display,
        {
            self
                .body = value
                .try_into()
                .map(From::from)
                .map_err(|s| {
                    format!(
                        "conversion to `CreateCollectionBody` for body failed: {}", s
                    )
                });
            self
        }
        pub fn body_map<F>(mut self, f: F) -> Self
        where
            F: std::ops::FnOnce(
                types::builder::CreateCollectionBody,
            ) -> types::builder::CreateCollectionBody,
        {
            self.body = self.body.map(f);
            self
        }
        ///Sends a `POST` request to `/api/v1/collections`
        pub async fn send(
            self,
        ) -> Result<ResponseValue<types::CreateCollectionResponse>, Error<()>> {
            let Self { client, body } = self;
            let body = body
                .and_then(std::convert::TryInto::<types::CreateCollectionBody>::try_into)
                .map_err(Error::InvalidRequest)?;
            let url = format!("{}/api/v1/collections", client.baseurl,);
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
    /**Builder for [`Client::delete_collection`]

[`Client::delete_collection`]: super::Client::delete_collection*/
    #[derive(Debug, Clone)]
    pub struct DeleteCollection<'a> {
        client: &'a super::Client,
        body: Result<types::builder::DeleteCollectionBody, String>,
    }
    impl<'a> DeleteCollection<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                body: Ok(types::builder::DeleteCollectionBody::default()),
            }
        }
        pub fn body<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::DeleteCollectionBody>,
            <V as std::convert::TryInto<
                types::DeleteCollectionBody,
            >>::Error: std::fmt::Display,
        {
            self
                .body = value
                .try_into()
                .map(From::from)
                .map_err(|s| {
                    format!(
                        "conversion to `DeleteCollectionBody` for body failed: {}", s
                    )
                });
            self
        }
        pub fn body_map<F>(mut self, f: F) -> Self
        where
            F: std::ops::FnOnce(
                types::builder::DeleteCollectionBody,
            ) -> types::builder::DeleteCollectionBody,
        {
            self.body = self.body.map(f);
            self
        }
        ///Sends a `DELETE` request to `/api/v1/collections`
        pub async fn send(
            self,
        ) -> Result<ResponseValue<types::DeleteCollectionResponse>, Error<()>> {
            let Self { client, body } = self;
            let body = body
                .and_then(std::convert::TryInto::<types::DeleteCollectionBody>::try_into)
                .map_err(Error::InvalidRequest)?;
            let url = format!("{}/api/v1/collections", client.baseurl,);
            let request = client
                .client
                .delete(url)
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
    /**Builder for [`Client::create_documents`]

[`Client::create_documents`]: super::Client::create_documents*/
    #[derive(Debug, Clone)]
    pub struct CreateDocuments<'a> {
        client: &'a super::Client,
        body: Result<types::builder::CreateDocumentsBody, String>,
    }
    impl<'a> CreateDocuments<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                body: Ok(types::builder::CreateDocumentsBody::default()),
            }
        }
        pub fn body<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::CreateDocumentsBody>,
            <V as std::convert::TryInto<
                types::CreateDocumentsBody,
            >>::Error: std::fmt::Display,
        {
            self
                .body = value
                .try_into()
                .map(From::from)
                .map_err(|s| {
                    format!("conversion to `CreateDocumentsBody` for body failed: {}", s)
                });
            self
        }
        pub fn body_map<F>(mut self, f: F) -> Self
        where
            F: std::ops::FnOnce(
                types::builder::CreateDocumentsBody,
            ) -> types::builder::CreateDocumentsBody,
        {
            self.body = self.body.map(f);
            self
        }
        ///Sends a `POST` request to `/api/v1/documents`
        pub async fn send(
            self,
        ) -> Result<ResponseValue<types::CreateDocumentsResponse>, Error<()>> {
            let Self { client, body } = self;
            let body = body
                .and_then(std::convert::TryInto::<types::CreateDocumentsBody>::try_into)
                .map_err(Error::InvalidRequest)?;
            let url = format!("{}/api/v1/documents", client.baseurl,);
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
    /**Builder for [`Client::delete_documents`]

[`Client::delete_documents`]: super::Client::delete_documents*/
    #[derive(Debug, Clone)]
    pub struct DeleteDocuments<'a> {
        client: &'a super::Client,
        body: Result<types::builder::DeleteDocumentsBody, String>,
    }
    impl<'a> DeleteDocuments<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                body: Ok(types::builder::DeleteDocumentsBody::default()),
            }
        }
        pub fn body<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::DeleteDocumentsBody>,
            <V as std::convert::TryInto<
                types::DeleteDocumentsBody,
            >>::Error: std::fmt::Display,
        {
            self
                .body = value
                .try_into()
                .map(From::from)
                .map_err(|s| {
                    format!("conversion to `DeleteDocumentsBody` for body failed: {}", s)
                });
            self
        }
        pub fn body_map<F>(mut self, f: F) -> Self
        where
            F: std::ops::FnOnce(
                types::builder::DeleteDocumentsBody,
            ) -> types::builder::DeleteDocumentsBody,
        {
            self.body = self.body.map(f);
            self
        }
        ///Sends a `PATCH` request to `/api/v1/documents`
        pub async fn send(
            self,
        ) -> Result<ResponseValue<types::DeleteDocumentsResponse>, Error<()>> {
            let Self { client, body } = self;
            let body = body
                .and_then(std::convert::TryInto::<types::DeleteDocumentsBody>::try_into)
                .map_err(Error::InvalidRequest)?;
            let url = format!("{}/api/v1/documents", client.baseurl,);
            let request = client
                .client
                .patch(url)
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
