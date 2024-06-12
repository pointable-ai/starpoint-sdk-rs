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
          "metadata": {
            "type": [
              "object",
              "null"
            ]
          },
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
    impl EmbedBody {
        pub fn builder() -> builder::EmbedBody {
            Default::default()
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
    "metadata": {
      "type": [
        "object",
        "null"
      ]
    },
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
        pub metadata: Option<serde_json::Map<String, serde_json::Value>>,
        pub text: String,
    }
    impl From<&EmbedBodyItemsItem> for EmbedBodyItemsItem {
        fn from(value: &EmbedBodyItemsItem) -> Self {
            value.clone()
        }
    }
    impl EmbedBodyItemsItem {
        pub fn builder() -> builder::EmbedBodyItemsItem {
            Default::default()
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
          "metadata": {
            "type": [
              "object",
              "null"
            ]
          },
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
    impl EmbedResponse {
        pub fn builder() -> builder::EmbedResponse {
            Default::default()
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
    "metadata": {
      "type": [
        "object",
        "null"
      ]
    },
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
        pub metadata: Option<serde_json::Map<String, serde_json::Value>>,
        pub text: String,
    }
    impl From<&EmbedResponseResultsItem> for EmbedResponseResultsItem {
        fn from(value: &EmbedResponseResultsItem) -> Self {
            value.clone()
        }
    }
    impl EmbedResponseResultsItem {
        pub fn builder() -> builder::EmbedResponseResultsItem {
            Default::default()
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
    impl ModelIdsResponse {
        pub fn builder() -> builder::ModelIdsResponse {
            Default::default()
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
          "metadata": {
            "type": [
              "object",
              "null"
            ]
          },
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
    impl TextEmbeddingRequest {
        pub fn builder() -> builder::TextEmbeddingRequest {
            Default::default()
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
    "metadata": {
      "type": [
        "object",
        "null"
      ]
    },
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
        pub metadata: Option<serde_json::Map<String, serde_json::Value>>,
        pub text: String,
    }
    impl From<&TextEmbeddingRequestItemsItem> for TextEmbeddingRequestItemsItem {
        fn from(value: &TextEmbeddingRequestItemsItem) -> Self {
            value.clone()
        }
    }
    impl TextEmbeddingRequestItemsItem {
        pub fn builder() -> builder::TextEmbeddingRequestItemsItem {
            Default::default()
        }
    }
    pub mod builder {
        #[derive(Clone, Debug)]
        pub struct EmbedBody {
            items: Result<Vec<super::EmbedBodyItemsItem>, String>,
            model: Result<String, String>,
        }
        impl Default for EmbedBody {
            fn default() -> Self {
                Self {
                    items: Err("no value supplied for items".to_string()),
                    model: Err("no value supplied for model".to_string()),
                }
            }
        }
        impl EmbedBody {
            pub fn items<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Vec<super::EmbedBodyItemsItem>>,
                T::Error: std::fmt::Display,
            {
                self
                    .items = value
                    .try_into()
                    .map_err(|e| {
                        format!("error converting supplied value for items: {}", e)
                    });
                self
            }
            pub fn model<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<String>,
                T::Error: std::fmt::Display,
            {
                self
                    .model = value
                    .try_into()
                    .map_err(|e| {
                        format!("error converting supplied value for model: {}", e)
                    });
                self
            }
        }
        impl std::convert::TryFrom<EmbedBody> for super::EmbedBody {
            type Error = String;
            fn try_from(value: EmbedBody) -> Result<Self, String> {
                Ok(Self {
                    items: value.items?,
                    model: value.model?,
                })
            }
        }
        impl From<super::EmbedBody> for EmbedBody {
            fn from(value: super::EmbedBody) -> Self {
                Self {
                    items: Ok(value.items),
                    model: Ok(value.model),
                }
            }
        }
        #[derive(Clone, Debug)]
        pub struct EmbedBodyItemsItem {
            metadata: Result<Option<serde_json::Map<String, serde_json::Value>>, String>,
            text: Result<String, String>,
        }
        impl Default for EmbedBodyItemsItem {
            fn default() -> Self {
                Self {
                    metadata: Ok(Default::default()),
                    text: Err("no value supplied for text".to_string()),
                }
            }
        }
        impl EmbedBodyItemsItem {
            pub fn metadata<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<
                    Option<serde_json::Map<String, serde_json::Value>>,
                >,
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
            pub fn text<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<String>,
                T::Error: std::fmt::Display,
            {
                self
                    .text = value
                    .try_into()
                    .map_err(|e| {
                        format!("error converting supplied value for text: {}", e)
                    });
                self
            }
        }
        impl std::convert::TryFrom<EmbedBodyItemsItem> for super::EmbedBodyItemsItem {
            type Error = String;
            fn try_from(value: EmbedBodyItemsItem) -> Result<Self, String> {
                Ok(Self {
                    metadata: value.metadata?,
                    text: value.text?,
                })
            }
        }
        impl From<super::EmbedBodyItemsItem> for EmbedBodyItemsItem {
            fn from(value: super::EmbedBodyItemsItem) -> Self {
                Self {
                    metadata: Ok(value.metadata),
                    text: Ok(value.text),
                }
            }
        }
        #[derive(Clone, Debug)]
        pub struct EmbedResponse {
            model: Result<String, String>,
            results: Result<Vec<super::EmbedResponseResultsItem>, String>,
        }
        impl Default for EmbedResponse {
            fn default() -> Self {
                Self {
                    model: Err("no value supplied for model".to_string()),
                    results: Err("no value supplied for results".to_string()),
                }
            }
        }
        impl EmbedResponse {
            pub fn model<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<String>,
                T::Error: std::fmt::Display,
            {
                self
                    .model = value
                    .try_into()
                    .map_err(|e| {
                        format!("error converting supplied value for model: {}", e)
                    });
                self
            }
            pub fn results<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Vec<super::EmbedResponseResultsItem>>,
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
        }
        impl std::convert::TryFrom<EmbedResponse> for super::EmbedResponse {
            type Error = String;
            fn try_from(value: EmbedResponse) -> Result<Self, String> {
                Ok(Self {
                    model: value.model?,
                    results: value.results?,
                })
            }
        }
        impl From<super::EmbedResponse> for EmbedResponse {
            fn from(value: super::EmbedResponse) -> Self {
                Self {
                    model: Ok(value.model),
                    results: Ok(value.results),
                }
            }
        }
        #[derive(Clone, Debug)]
        pub struct EmbedResponseResultsItem {
            embedding: Result<Vec<f32>, String>,
            metadata: Result<Option<serde_json::Map<String, serde_json::Value>>, String>,
            text: Result<String, String>,
        }
        impl Default for EmbedResponseResultsItem {
            fn default() -> Self {
                Self {
                    embedding: Err("no value supplied for embedding".to_string()),
                    metadata: Ok(Default::default()),
                    text: Err("no value supplied for text".to_string()),
                }
            }
        }
        impl EmbedResponseResultsItem {
            pub fn embedding<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Vec<f32>>,
                T::Error: std::fmt::Display,
            {
                self
                    .embedding = value
                    .try_into()
                    .map_err(|e| {
                        format!("error converting supplied value for embedding: {}", e)
                    });
                self
            }
            pub fn metadata<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<
                    Option<serde_json::Map<String, serde_json::Value>>,
                >,
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
            pub fn text<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<String>,
                T::Error: std::fmt::Display,
            {
                self
                    .text = value
                    .try_into()
                    .map_err(|e| {
                        format!("error converting supplied value for text: {}", e)
                    });
                self
            }
        }
        impl std::convert::TryFrom<EmbedResponseResultsItem>
        for super::EmbedResponseResultsItem {
            type Error = String;
            fn try_from(value: EmbedResponseResultsItem) -> Result<Self, String> {
                Ok(Self {
                    embedding: value.embedding?,
                    metadata: value.metadata?,
                    text: value.text?,
                })
            }
        }
        impl From<super::EmbedResponseResultsItem> for EmbedResponseResultsItem {
            fn from(value: super::EmbedResponseResultsItem) -> Self {
                Self {
                    embedding: Ok(value.embedding),
                    metadata: Ok(value.metadata),
                    text: Ok(value.text),
                }
            }
        }
        #[derive(Clone, Debug)]
        pub struct ModelIdsResponse {
            model_ids: Result<Vec<String>, String>,
        }
        impl Default for ModelIdsResponse {
            fn default() -> Self {
                Self {
                    model_ids: Err("no value supplied for model_ids".to_string()),
                }
            }
        }
        impl ModelIdsResponse {
            pub fn model_ids<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Vec<String>>,
                T::Error: std::fmt::Display,
            {
                self
                    .model_ids = value
                    .try_into()
                    .map_err(|e| {
                        format!("error converting supplied value for model_ids: {}", e)
                    });
                self
            }
        }
        impl std::convert::TryFrom<ModelIdsResponse> for super::ModelIdsResponse {
            type Error = String;
            fn try_from(value: ModelIdsResponse) -> Result<Self, String> {
                Ok(Self {
                    model_ids: value.model_ids?,
                })
            }
        }
        impl From<super::ModelIdsResponse> for ModelIdsResponse {
            fn from(value: super::ModelIdsResponse) -> Self {
                Self {
                    model_ids: Ok(value.model_ids),
                }
            }
        }
        #[derive(Clone, Debug)]
        pub struct TextEmbeddingRequest {
            items: Result<Vec<super::TextEmbeddingRequestItemsItem>, String>,
            model: Result<String, String>,
        }
        impl Default for TextEmbeddingRequest {
            fn default() -> Self {
                Self {
                    items: Err("no value supplied for items".to_string()),
                    model: Err("no value supplied for model".to_string()),
                }
            }
        }
        impl TextEmbeddingRequest {
            pub fn items<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Vec<super::TextEmbeddingRequestItemsItem>>,
                T::Error: std::fmt::Display,
            {
                self
                    .items = value
                    .try_into()
                    .map_err(|e| {
                        format!("error converting supplied value for items: {}", e)
                    });
                self
            }
            pub fn model<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<String>,
                T::Error: std::fmt::Display,
            {
                self
                    .model = value
                    .try_into()
                    .map_err(|e| {
                        format!("error converting supplied value for model: {}", e)
                    });
                self
            }
        }
        impl std::convert::TryFrom<TextEmbeddingRequest>
        for super::TextEmbeddingRequest {
            type Error = String;
            fn try_from(value: TextEmbeddingRequest) -> Result<Self, String> {
                Ok(Self {
                    items: value.items?,
                    model: value.model?,
                })
            }
        }
        impl From<super::TextEmbeddingRequest> for TextEmbeddingRequest {
            fn from(value: super::TextEmbeddingRequest) -> Self {
                Self {
                    items: Ok(value.items),
                    model: Ok(value.model),
                }
            }
        }
        #[derive(Clone, Debug)]
        pub struct TextEmbeddingRequestItemsItem {
            metadata: Result<Option<serde_json::Map<String, serde_json::Value>>, String>,
            text: Result<String, String>,
        }
        impl Default for TextEmbeddingRequestItemsItem {
            fn default() -> Self {
                Self {
                    metadata: Ok(Default::default()),
                    text: Err("no value supplied for text".to_string()),
                }
            }
        }
        impl TextEmbeddingRequestItemsItem {
            pub fn metadata<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<
                    Option<serde_json::Map<String, serde_json::Value>>,
                >,
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
            pub fn text<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<String>,
                T::Error: std::fmt::Display,
            {
                self
                    .text = value
                    .try_into()
                    .map_err(|e| {
                        format!("error converting supplied value for text: {}", e)
                    });
                self
            }
        }
        impl std::convert::TryFrom<TextEmbeddingRequestItemsItem>
        for super::TextEmbeddingRequestItemsItem {
            type Error = String;
            fn try_from(value: TextEmbeddingRequestItemsItem) -> Result<Self, String> {
                Ok(Self {
                    metadata: value.metadata?,
                    text: value.text?,
                })
            }
        }
        impl From<super::TextEmbeddingRequestItemsItem>
        for TextEmbeddingRequestItemsItem {
            fn from(value: super::TextEmbeddingRequestItemsItem) -> Self {
                Self {
                    metadata: Ok(value.metadata),
                    text: Ok(value.text),
                }
            }
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

```ignore
let response = client.embed()
    .body(body)
    .send()
    .await;
```*/
    pub fn embed(&self) -> builder::Embed {
        builder::Embed::new(self)
    }
    /**Sends a `GET` request to `/api/v1/model_ids`

```ignore
let response = client.model_ids()
    .send()
    .await;
```*/
    pub fn model_ids(&self) -> builder::ModelIds {
        builder::ModelIds::new(self)
    }
}
pub mod builder {
    use super::types;
    #[allow(unused_imports)]
    use super::{
        encode_path, ByteStream, Error, HeaderMap, HeaderValue, RequestBuilderExt,
        ResponseValue,
    };
    /**Builder for [`Client::embed`]

[`Client::embed`]: super::Client::embed*/
    #[derive(Debug, Clone)]
    pub struct Embed<'a> {
        client: &'a super::Client,
        body: Result<types::builder::EmbedBody, String>,
    }
    impl<'a> Embed<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                body: Ok(types::builder::EmbedBody::default()),
            }
        }
        pub fn body<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::EmbedBody>,
            <V as std::convert::TryInto<types::EmbedBody>>::Error: std::fmt::Display,
        {
            self
                .body = value
                .try_into()
                .map(From::from)
                .map_err(|s| {
                    format!("conversion to `EmbedBody` for body failed: {}", s)
                });
            self
        }
        pub fn body_map<F>(mut self, f: F) -> Self
        where
            F: std::ops::FnOnce(types::builder::EmbedBody) -> types::builder::EmbedBody,
        {
            self.body = self.body.map(f);
            self
        }
        ///Sends a `POST` request to `/api/v1/embed`
        pub async fn send(
            self,
        ) -> Result<ResponseValue<types::EmbedResponse>, Error<()>> {
            let Self { client, body } = self;
            let body = body
                .and_then(std::convert::TryInto::<types::EmbedBody>::try_into)
                .map_err(Error::InvalidRequest)?;
            let url = format!("{}/api/v1/embed", client.baseurl,);
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
    /**Builder for [`Client::model_ids`]

[`Client::model_ids`]: super::Client::model_ids*/
    #[derive(Debug, Clone)]
    pub struct ModelIds<'a> {
        client: &'a super::Client,
    }
    impl<'a> ModelIds<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self { client: client }
        }
        ///Sends a `GET` request to `/api/v1/model_ids`
        pub async fn send(
            self,
        ) -> Result<ResponseValue<types::ModelIdsResponse>, Error<()>> {
            let Self { client } = self;
            let url = format!("{}/api/v1/model_ids", client.baseurl,);
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
}
pub mod prelude {
    pub use self::super::Client;
}
