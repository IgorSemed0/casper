use reqwest::Client;

pub async fn connect_to_service(service: &str, _action: &str) -> Result<String, String> {
    //  Example HTTP request
    let client = Client::new();
    match service {
        "example_api" => {
            let response = client
                .get("https://api.example.com")
                .send()
                .await
                .map_err(|e| e.to_string())?;
            response.text().await.map_err(|e| e.to_string())
        },
        _ => Err(format!("Unsupported service: {}", service)),
    }
}