use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::Value;

pub mod distance_metric;
pub mod id;
pub mod query_embeddings;
pub mod tokenizer_type;

use distance_metric::DistanceMetric;
use id::{DocumentID, ID};
use query_embeddings::QueryEmbeddings;
use tokenizer_type::TokenizerType;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct QueryRequest {
    pub collection_id: Option<ID>,
    pub collection_name: Option<String>,
    pub query_embedding: Option<QueryEmbeddings>,
    pub query_document_id: Option<DocumentID>,
    pub text_search_weight: Option<f32>,
    pub sql: Option<String>,
    pub params: Option<Vec<Value>>,
    pub distance_metric: Option<DistanceMetric>,
    pub text_search_query: Option<Vec<String>>,
    pub tokenizer_type: Option<TokenizerType>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct QueryResponse {
    pub collection_id: ID,
    pub sql: Option<String>,
    pub results: Vec<Value>,
    pub result_count: usize,
    pub distance_metric: DistanceMetric,
}

const DEFAULT_READER_HOST: &str = "https://reader.starpoint.ai";
const DEFAULT_WRITER_HOST: &str = "https://writer.starpoint.ai";

pub struct StarpointClient {
    pub client: reqwest::Client,
    pub reader_host: String,
    pub writer_host: String,
}

impl StarpointClient {
    pub fn new(reader_host: Option<String>, writer_host: Option<String>) -> Self {
        let client = Client::builder()
            .gzip(true)
            .brotli(true)
            .build()
            .expect("Could not build Reqwest client. There is something very wrong here.");
        let reader_host = reader_host.unwrap_or(DEFAULT_READER_HOST.to_string());
        let writer_host = writer_host.unwrap_or(DEFAULT_WRITER_HOST.to_string());

        Self {
            client,
            reader_host,
            writer_host,
        }
    }

    pub async fn query(&self, request: QueryRequest) -> Result<QueryResponse, reqwest::Error> {
        let endpoint = format!("{}/api/v1/query", &self.reader_host);

        let response = self
            .client
            .post(endpoint)
            .json::<QueryRequest>(&request)
            .send()
            .await?
            .json::<QueryResponse>()
            .await?;
        Ok(response)
    }
}
