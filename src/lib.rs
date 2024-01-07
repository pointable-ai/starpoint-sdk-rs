use reqwest::header;

pub mod reader;
pub mod writer;

pub struct StarpointClient {
    pub reader: reader::Client,
    pub writer: writer::Client,
}

const DEFAULT_READER_HOST: &str = "https://reader.starpoint.ai";
const DEFAULT_WRITER_HOST: &str = "https://writer.starpoint.ai";

impl StarpointClient {
    pub fn new(
        api_key: &str,
        custom_reader_host: Option<&str>,
        custom_writer_host: Option<&str>,
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

        let reader_host = custom_reader_host.unwrap_or(DEFAULT_READER_HOST);
        let writer_host = custom_writer_host.unwrap_or(DEFAULT_WRITER_HOST);
        let reader = reader::Client::new_with_client(reader_host, reqwest_client.clone());
        let writer = writer::Client::new_with_client(writer_host, reqwest_client.clone());

        Self { reader, writer }
    }
}
