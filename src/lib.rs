use reqwest::header;

pub mod embedding;
pub mod reader;
pub mod writer;

#[derive(Clone, Debug)]
pub struct StarpointClient {
    pub reader: reader::Client,
    pub writer: writer::Client,
    pub embedding: embedding::Client,
}

const DEFAULT_READER_HOST: &str = "https://reader.starpoint.ai";
const DEFAULT_WRITER_HOST: &str = "https://writer.starpoint.ai";
const DEFAULT_EMBEDDING_HOST: &str = "https://embedding.starpoint.ai";

impl StarpointClient {
    pub fn new(
        api_key: &str,
        custom_reader_host: &Option<String>,
        custom_writer_host: &Option<String>,
        custom_embedding_host: &Option<String>,
    ) -> Self {
        let mut headers = header::HeaderMap::new();
        headers.insert(
            "x-starpoint-key",
            header::HeaderValue::from_str(api_key).expect(
                "Invalid Starpoint API key. Check `api_key` passed into StarpointClient::new.",
            ),
        );

        let reqwest_client = reqwest::Client::builder()
            .default_headers(headers)
            .build()
            .expect("Failed to build reqwest client for StarpointClient::new.");

        let reader_host = custom_reader_host
            .as_ref()
            .map(|s| s.as_str())
            .unwrap_or_else(|| DEFAULT_READER_HOST);
        let reader = reader::Client::new_with_client(reader_host, reqwest_client.clone());

        let writer_host = custom_writer_host
            .as_ref()
            .map(|s| s.as_str())
            .unwrap_or_else(|| DEFAULT_WRITER_HOST);
        let writer = writer::Client::new_with_client(writer_host, reqwest_client.clone());

        let embedding_host = custom_embedding_host
            .as_ref()
            .map(|s| s.as_str())
            .unwrap_or_else(|| DEFAULT_EMBEDDING_HOST);
        let embedding = embedding::Client::new_with_client(embedding_host, reqwest_client.clone());

        Self {
            reader,
            writer,
            embedding,
        }
    }
}
