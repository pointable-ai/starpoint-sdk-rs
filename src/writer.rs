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
      "type": "string",
      "format": "uuid"
    }
  }
}*/
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct CreateCollectionBody {
        pub dimensionality: i32,
        pub name: uuid::Uuid,
    }
    impl From<&CreateCollectionBody> for CreateCollectionBody {
        fn from(value: &CreateCollectionBody) -> Self {
            value.clone()
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
    ///CreateDocumentsBody
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
        pub collection_id: uuid::Uuid,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub collection_name: Option<String>,
        pub documents: Vec<CreateDocumentsBodyDocumentsItem>,
    }
    impl From<&CreateDocumentsBody> for CreateDocumentsBody {
        fn from(value: &CreateDocumentsBody) -> Self {
            value.clone()
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
    ///CreateDocumentsRequest
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
        pub collection_id: uuid::Uuid,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub collection_name: Option<String>,
        pub documents: Vec<CreateDocumentsRequestDocumentsItem>,
    }
    impl From<&CreateDocumentsRequest> for CreateDocumentsRequest {
        fn from(value: &CreateDocumentsRequest) -> Self {
            value.clone()
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
    ///DeleteDocumentsBody
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
        pub collection_id: uuid::Uuid,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub collection_name: Option<String>,
        pub documents: Vec<String>,
    }
    impl From<&DeleteDocumentsBody> for DeleteDocumentsBody {
        fn from(value: &DeleteDocumentsBody) -> Self {
            value.clone()
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

*/
    pub async fn get_collections<'a>(
        &'a self,
    ) -> Result<ResponseValue<types::GetCollectionsResponse>, Error<()>> {
        let url = format!("{}/api/v1/collections", self.baseurl,);
        let request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Sends a `PUT` request to `/api/v1/collections`

*/
    pub async fn change_collection_name<'a>(
        &'a self,
        body: &'a types::ChangeCollectionNameBody,
    ) -> Result<ResponseValue<types::ChangeCollectionNameResponse>, Error<()>> {
        let url = format!("{}/api/v1/collections", self.baseurl,);
        let request = self
            .client
            .put(url)
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
    /**Sends a `POST` request to `/api/v1/collections`

*/
    pub async fn create_collection<'a>(
        &'a self,
        body: &'a types::CreateCollectionBody,
    ) -> Result<ResponseValue<types::CreateCollectionResponse>, Error<()>> {
        let url = format!("{}/api/v1/collections", self.baseurl,);
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
    /**Sends a `DELETE` request to `/api/v1/collections`

*/
    pub async fn delete_collection<'a>(
        &'a self,
        body: &'a types::DeleteCollectionBody,
    ) -> Result<ResponseValue<types::DeleteCollectionResponse>, Error<()>> {
        let url = format!("{}/api/v1/collections", self.baseurl,);
        let request = self
            .client
            .delete(url)
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
    /**Sends a `POST` request to `/api/v1/documents`

*/
    pub async fn create_documents<'a>(
        &'a self,
        body: &'a types::CreateDocumentsBody,
    ) -> Result<ResponseValue<types::CreateDocumentsResponse>, Error<()>> {
        let url = format!("{}/api/v1/documents", self.baseurl,);
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
    /**Sends a `PATCH` request to `/api/v1/documents`

*/
    pub async fn delete_documents<'a>(
        &'a self,
        body: &'a types::DeleteDocumentsBody,
    ) -> Result<ResponseValue<types::DeleteDocumentsResponse>, Error<()>> {
        let url = format!("{}/api/v1/documents", self.baseurl,);
        let request = self
            .client
            .patch(url)
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
