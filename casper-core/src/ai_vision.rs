use base64::{Engine as _, engine::general_purpose};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
use std::path::Path;

/// Configuration for AI provider
#[derive(Debug, Clone)]
pub struct AIConfig {
    pub request_url: String,
    pub token: String,
    pub model: String,
    pub max_tokens: Option<u32>,
    pub temperature: Option<f32>,
    pub timeout_seconds: Option<u64>,
}

impl AIConfig {
    /// Load configuration from environment variables
    pub fn from_env() -> Result<Self, String> {
        // Load .env file if it exists
        dotenv::dotenv().ok();

        let request_url = env::var("AI_REQUEST_URL")
            .map_err(|_| "AI_REQUEST_URL not set in environment".to_string())?;

        let token =
            env::var("AI_TOKEN").map_err(|_| "AI_TOKEN not set in environment".to_string())?;

        let model =
            env::var("AI_MODEL").map_err(|_| "AI_MODEL not set in environment".to_string())?;

        let max_tokens = env::var("AI_MAX_TOKENS").ok().and_then(|v| v.parse().ok());

        let temperature = env::var("AI_TEMPERATURE").ok().and_then(|v| v.parse().ok());

        let timeout_seconds = env::var("AI_TIMEOUT_SECONDS")
            .ok()
            .and_then(|v| v.parse().ok());

        Ok(AIConfig {
            request_url,
            token,
            model,
            max_tokens,
            temperature,
            timeout_seconds,
        })
    }
}

/// Request to Gemini API with vision
#[derive(Debug, Serialize)]
struct GeminiRequest {
    contents: Vec<GeminiContent>,
    #[serde(skip_serializing_if = "Option::is_none")]
    generation_config: Option<GenerationConfig>,
}

#[derive(Debug, Serialize)]
struct GeminiContent {
    parts: Vec<GeminiPart>,
}

#[derive(Debug, Serialize)]
#[serde(untagged)]
enum GeminiPart {
    Text { text: String },
    Image { inline_data: InlineData },
}

#[derive(Debug, Serialize)]
struct InlineData {
    mime_type: String,
    data: String,
}

#[derive(Debug, Serialize)]
struct GenerationConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    temperature: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_output_tokens: Option<u32>,
}

/// Response from Gemini API
#[derive(Debug, Deserialize)]
struct GeminiResponse {
    candidates: Vec<GeminiCandidate>,
}

#[derive(Debug, Deserialize)]
struct GeminiCandidate {
    content: GeminiResponseContent,
}

#[derive(Debug, Deserialize)]
struct GeminiResponseContent {
    parts: Vec<GeminiResponsePart>,
}

#[derive(Debug, Deserialize)]
struct GeminiResponsePart {
    text: String,
}

/// AI Vision client for understanding screen content
pub struct AIVision {
    config: AIConfig,
    client: Client,
}

impl AIVision {
    /// Create a new AI vision client
    pub fn new(config: AIConfig) -> Self {
        let timeout = std::time::Duration::from_secs(config.timeout_seconds.unwrap_or(30));
        let client = Client::builder()
            .timeout(timeout)
            .build()
            .unwrap_or_else(|_| Client::new());

        AIVision { config, client }
    }

    /// Create from environment variables
    pub fn from_env() -> Result<Self, String> {
        let config = AIConfig::from_env()?;
        Ok(Self::new(config))
    }

    /// Analyze a screenshot and answer a question about it
    pub async fn analyze_screenshot(
        &self,
        image_path: &str,
        prompt: &str,
    ) -> Result<String, String> {
        // Read and encode image
        let image_data =
            fs::read(image_path).map_err(|e| format!("Failed to read image: {}", e))?;

        self.analyze_image(&image_data, prompt).await
    }

    /// Analyze image data directly
    pub async fn analyze_image(&self, image_data: &[u8], prompt: &str) -> Result<String, String> {
        // Encode image to base64
        let base64_image = general_purpose::STANDARD.encode(image_data);

        // Detect MIME type (simplified - assumes PNG for now)
        let mime_type = detect_image_mime_type(image_data);

        // Build request for Gemini
        let request = GeminiRequest {
            contents: vec![GeminiContent {
                parts: vec![
                    GeminiPart::Text {
                        text: prompt.to_string(),
                    },
                    GeminiPart::Image {
                        inline_data: InlineData {
                            mime_type: mime_type.to_string(),
                            data: base64_image,
                        },
                    },
                ],
            }],
            generation_config: Some(GenerationConfig {
                temperature: self.config.temperature,
                max_output_tokens: self.config.max_tokens,
            }),
        };

        // Make API request
        let url = format!("{}?key={}", self.config.request_url, self.config.token);

        let response = self
            .client
            .post(&url)
            .json(&request)
            .send()
            .await
            .map_err(|e| format!("Failed to send request: {}", e))?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            return Err(format!("API error {}: {}", status, error_text));
        }

        let gemini_response: GeminiResponse = response
            .json()
            .await
            .map_err(|e| format!("Failed to parse response: {}", e))?;

        // Extract text from response
        let text = gemini_response
            .candidates
            .first()
            .and_then(|c| c.content.parts.first())
            .map(|p| p.text.clone())
            .ok_or_else(|| "No response text from API".to_string())?;

        Ok(text)
    }

    /// Find UI element coordinates by description
    pub async fn find_element(
        &self,
        image_path: &str,
        element_description: &str,
    ) -> Result<Option<ElementPosition>, String> {
        let prompt = format!(
            "Look at this screenshot and find the '{}' element. \
             If you find it, respond ONLY with JSON in this exact format: \
             {{\"found\": true, \"x\": <x_coordinate>, \"y\": <y_coordinate>, \
             \"width\": <width>, \"height\": <height>, \"confidence\": <0-100>}} \
             If you cannot find it, respond with: {{\"found\": false}} \
             Do not include any other text in your response.",
            element_description
        );

        let response = self.analyze_screenshot(image_path, &prompt).await?;

        // Try to parse JSON response
        match serde_json::from_str::<ElementPosition>(&response) {
            Ok(pos) => {
                if pos.found {
                    Ok(Some(pos))
                } else {
                    Ok(None)
                }
            }
            Err(_) => {
                // If JSON parsing fails, the AI might have added extra text
                // Try to extract JSON from the response
                if let Some(json_str) = extract_json_from_text(&response) {
                    match serde_json::from_str::<ElementPosition>(&json_str) {
                        Ok(pos) => Ok(if pos.found { Some(pos) } else { None }),
                        Err(e) => Err(format!("Failed to parse element position: {}", e)),
                    }
                } else {
                    Err(format!("AI response is not valid JSON: {}", response))
                }
            }
        }
    }

    /// Understand what's currently on screen
    pub async fn describe_screen(&self, image_path: &str) -> Result<String, String> {
        let prompt = "Describe what you see on this screen. \
                      Focus on: the main application, visible UI elements, \
                      any text content, and the current state. \
                      Be concise but thorough.";

        self.analyze_screenshot(image_path, prompt).await
    }

    /// Check if a specific element is visible
    pub async fn is_element_visible(
        &self,
        image_path: &str,
        element_description: &str,
    ) -> Result<bool, String> {
        let prompt = format!(
            "Look at this screenshot. Is there a '{}' visible? \
             Respond with ONLY 'yes' or 'no'.",
            element_description
        );

        let response = self.analyze_screenshot(image_path, &prompt).await?;
        Ok(response.trim().to_lowercase().starts_with("yes"))
    }

    /// Get actionable suggestions for a task
    pub async fn suggest_actions(
        &self,
        image_path: &str,
        task: &str,
    ) -> Result<Vec<String>, String> {
        let prompt = format!(
            "Looking at this screenshot, I want to: {} \
             List the specific steps I should take, one per line. \
             Format each step as: 'Action: Description'. \
             Be specific about what to click, type, or do.",
            task
        );

        let response = self.analyze_screenshot(image_path, &prompt).await?;

        // Parse steps from response
        let steps: Vec<String> = response
            .lines()
            .filter(|line| !line.trim().is_empty())
            .map(|line| line.trim().to_string())
            .collect();

        Ok(steps)
    }
}

/// Position of a UI element
#[derive(Debug, Deserialize, Serialize)]
pub struct ElementPosition {
    pub found: bool,
    #[serde(default)]
    pub x: i32,
    #[serde(default)]
    pub y: i32,
    #[serde(default)]
    pub width: i32,
    #[serde(default)]
    pub height: i32,
    #[serde(default)]
    pub confidence: u8,
}

/// Detect MIME type from image data
fn detect_image_mime_type(data: &[u8]) -> &'static str {
    if data.len() < 4 {
        return "image/png"; // default
    }

    // Check magic numbers
    match &data[0..4] {
        [0x89, b'P', b'N', b'G'] => "image/png",
        [0xFF, 0xD8, 0xFF, _] => "image/jpeg",
        [b'G', b'I', b'F', b'8'] => "image/gif",
        [b'R', b'I', b'F', b'F'] => "image/webp",
        _ => "image/png", // default
    }
}

/// Extract JSON object from text that might contain extra content
fn extract_json_from_text(text: &str) -> Option<String> {
    // Find the first { and last }
    let start = text.find('{')?;
    let end = text.rfind('}')?;

    if end > start {
        Some(text[start..=end].to_string())
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_png() {
        let png_header = [0x89, b'P', b'N', b'G', 0x0D, 0x0A];
        assert_eq!(detect_image_mime_type(&png_header), "image/png");
    }

    #[test]
    fn test_detect_jpeg() {
        let jpeg_header = [0xFF, 0xD8, 0xFF, 0xE0];
        assert_eq!(detect_image_mime_type(&jpeg_header), "image/jpeg");
    }

    #[test]
    fn test_extract_json() {
        let text = "Sure, here's the result: {\"found\": true, \"x\": 100}";
        let json = extract_json_from_text(text);
        assert!(json.is_some());
        assert_eq!(json.unwrap(), r#"{"found": true, "x": 100}"#);
    }
}
