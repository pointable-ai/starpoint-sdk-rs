use std::error::Error;

use progenitor::GenerationSettings;

async fn generate_sdk_file(url: &str, out_file: &str) -> Result<(), Box<dyn Error>> {
    let resp = reqwest::get(url).await?.text().await?;
    let spec = serde_json::from_str(&resp).unwrap();
    let mut settings = GenerationSettings::new();
    settings.with_interface(progenitor::InterfaceStyle::Builder);
    let mut generator = progenitor::Generator::new(&settings);

    let tokens = generator.generate_tokens(&spec).unwrap();
    let ast = syn::parse2(tokens).unwrap();
    let content = prettyplease::unparse(&ast);

    let out_file = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("src")
        .join(out_file);

    std::fs::write(out_file, content).unwrap();

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    if Ok("true".to_owned()) == std::env::var("___UNSAFE_REGENERATE_SDK") {
        let reader_url = "https://reader.starpoint.ai/api/v1/openapi.json";
        let writer_url = "https://writer.starpoint.ai/api/v1/openapi.json";
        let embedding_url = "https://embedding.starpoint.ai/api/v1/openapi.json";

        generate_sdk_file(reader_url, "reader.rs").await?;
        generate_sdk_file(writer_url, "writer.rs").await?;
        generate_sdk_file(embedding_url, "embedding.rs").await?;
    }
    Ok(())
}
