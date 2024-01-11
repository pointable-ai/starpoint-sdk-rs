pub use progenitor_client::{ByteStream, Error, ResponseValue};
#[allow(unused_imports)]
use progenitor_client::{encode_path, RequestBuilderExt};
#[allow(unused_imports)]
use reqwest::header::{HeaderMap, HeaderValue};
pub mod types {
    use serde::{Deserialize, Serialize};
    #[allow(unused_imports)]
    use std::convert::TryFrom;
    ///EmbedBody
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /**{
  "type": "object",
  "required": [
    "items",
    "model"
  ],
  "properties": {
    "items": {
      "type": "array",
      "items": {
        "type": "object",
        "required": [
          "text"
        ],
        "properties": {
          "metadata": {},
          "text": {
            "type": "string"
          }
        }
      }
    },
    "model": {
      "type": "string"
    }
  }
}*/
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct EmbedBody {
        pub items: Vec<EmbedBodyItemsItem>,
        pub model: String,
    }
    impl From<&EmbedBody> for EmbedBody {
        fn from(value: &EmbedBody) -> Self {
            value.clone()
        }
    }
    ///EmbedBodyItemsItem
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /**{
  "type": "object",
  "required": [
    "text"
  ],
  "properties": {
    "metadata": {},
    "text": {
      "type": "string"
    }
  }
}*/
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct EmbedBodyItemsItem {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub metadata: Option<serde_json::Value>,
        pub text: String,
    }
    impl From<&EmbedBodyItemsItem> for EmbedBodyItemsItem {
        fn from(value: &EmbedBodyItemsItem) -> Self {
            value.clone()
        }
    }
    ///EmbedResponse
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /**{
  "type": "object",
  "required": [
    "model",
    "results"
  ],
  "properties": {
    "model": {
      "type": "string"
    },
    "results": {
      "type": "array",
      "items": {
        "type": "object",
        "required": [
          "embedding",
          "text"
        ],
        "properties": {
          "embedding": {
            "type": "array",
            "items": {
              "type": "number",
              "format": "float"
            }
          },
          "metadata": {},
          "text": {
            "type": "string"
          }
        }
      }
    }
  }
}*/
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct EmbedResponse {
        pub model: String,
        pub results: Vec<EmbedResponseResultsItem>,
    }
    impl From<&EmbedResponse> for EmbedResponse {
        fn from(value: &EmbedResponse) -> Self {
            value.clone()
        }
    }
    ///EmbedResponseResultsItem
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /**{
  "type": "object",
  "required": [
    "embedding",
    "text"
  ],
  "properties": {
    "embedding": {
      "type": "array",
      "items": {
        "type": "number",
        "format": "float"
      }
    },
    "metadata": {},
    "text": {
      "type": "string"
    }
  }
}*/
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct EmbedResponseResultsItem {
        pub embedding: Vec<f32>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub metadata: Option<serde_json::Value>,
        pub text: String,
    }
    impl From<&EmbedResponseResultsItem> for EmbedResponseResultsItem {
        fn from(value: &EmbedResponseResultsItem) -> Self {
            value.clone()
        }
    }
    ///ModelIdsResponse
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /**{
  "type": "object",
  "required": [
    "model_ids"
  ],
  "properties": {
    "model_ids": {
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
    pub struct ModelIdsResponse {
        pub model_ids: Vec<String>,
    }
    impl From<&ModelIdsResponse> for ModelIdsResponse {
        fn from(value: &ModelIdsResponse) -> Self {
            value.clone()
        }
    }
    ///TextEmbeddingRequest
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /**{
  "type": "object",
  "required": [
    "items",
    "model"
  ],
  "properties": {
    "items": {
      "type": "array",
      "items": {
        "type": "object",
        "required": [
          "text"
        ],
        "properties": {
          "metadata": {},
          "text": {
            "type": "string"
          }
        }
      }
    },
    "model": {
      "type": "string"
    }
  }
}*/
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct TextEmbeddingRequest {
        pub items: Vec<TextEmbeddingRequestItemsItem>,
        pub model: String,
    }
    impl From<&TextEmbeddingRequest> for TextEmbeddingRequest {
        fn from(value: &TextEmbeddingRequest) -> Self {
            value.clone()
        }
    }
    ///TextEmbeddingRequestItemsItem
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /**{
  "type": "object",
  "required": [
    "text"
  ],
  "properties": {
    "metadata": {},
    "text": {
      "type": "string"
    }
  }
}*/
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct TextEmbeddingRequestItemsItem {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub metadata: Option<serde_json::Value>,
        pub text: String,
    }
    impl From<&TextEmbeddingRequestItemsItem> for TextEmbeddingRequestItemsItem {
        fn from(value: &TextEmbeddingRequestItemsItem) -> Self {
            value.clone()
        }
    }
}
#[derive(Clone, Debug)]
/**Client for scholar



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
    /**Sends a `POST` request to `/api/v1/embed`

*/
    pub async fn embed<'a>(
        &'a self,
        body: &'a types::EmbedBody,
    ) -> Result<ResponseValue<types::EmbedResponse>, Error<()>> {
        let url = format!("{}/api/v1/embed", self.baseurl,);
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
    /**Sends a `GET` request to `/api/v1/model_ids`

*/
    pub async fn model_ids<'a>(
        &'a self,
    ) -> Result<ResponseValue<types::ModelIdsResponse>, Error<()>> {
        let url = format!("{}/api/v1/model_ids", self.baseurl,);
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
}
pub mod prelude {
    pub use super::Client;
}
