use anyhow::{Error, Result};
use async_trait::async_trait;
use reqwest::Client;
use serde_json::{json, Value};
use std::time::Duration;
use regex::Regex;
use chrono::Utc;

use super::base::{ConfigKey, Provider, ProviderMetadata, ProviderUsage, Usage};
use super::errors::ProviderError;
use super::utils::{emit_debug_trace, get_model, handle_response_openai_compat};
use super::tool_parser::ToolParserProvider;
use crate::message::{Message, MessageContent};
use crate::model::ModelConfig;
use crate::providers::formats::openai::{create_request, get_usage, response_to_message};
use mcp_core::{role::Role, tool::Tool, content::TextContent};
use url::Url;

pub const OPENROUTER_DEFAULT_MODEL: &str = "anthropic/claude-3.5-sonnet";
pub const OPENROUTER_MODEL_PREFIX_ANTHROPIC: &str = "anthropic";

// OpenRouter can run many models, we suggest the default
pub const OPENROUTER_KNOWN_MODELS: &[&str] = &[OPENROUTER_DEFAULT_MODEL];
pub const OPENROUTER_DOC_URL: &str = "https://openrouter.ai/models";

#[derive(serde::Serialize)]
pub struct OpenRouterProvider {
    #[serde(skip)]
    client: Client,
    host: String,
    api_key: String,
    model: ModelConfig,
    #[serde(skip)]
    tool_parser: ToolParserProvider,
}

impl Default for OpenRouterProvider {
    fn default() -> Self {
        let model = ModelConfig::new(OpenRouterProvider::metadata().default_model.to_string());
        OpenRouterProvider::from_env(model).expect("Failed to initialize OpenRouter provider")
    }
}

impl OpenRouterProvider {
    pub fn from_env(model: ModelConfig) -> Result<Self> {
        let config = crate::config::Config::global();
        let api_key: String = config.get_secret("OPENROUTER_API_KEY")?;
        let host: String = config
            .get("OPENROUTER_HOST")
            .unwrap_or_else(|_| "https://openrouter.ai".to_string());

        let client = Client::builder()
            .timeout(Duration::from_secs(600))
            .build()?;

        Ok(Self {
            client,
            host,
            api_key,
            model,
            tool_parser: ToolParserProvider::default(),
        })
    }

    async fn post(&self, payload: Value) -> Result<Value, ProviderError> {
        let base_url = Url::parse(&self.host)
            .map_err(|e| ProviderError::RequestFailed(format!("Invalid base URL: {e}")))?;
        let url = base_url.join("api/v1/chat/completions").map_err(|e| {
            ProviderError::RequestFailed(format!("Failed to construct endpoint URL: {e}"))
        })?;

        let response = self
            .client
            .post(url)
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("HTTP-Referer", "https://github.com/block/goose")
            .header("X-Title", "Goose")
            .json(&payload)
            .send()
            .await?;

        handle_response_openai_compat(response).await
    }
}

/// Update the request when using anthropic model.
/// For anthropic model, we can enable prompt caching to save cost. Since openrouter is the OpenAI compatible
/// endpoint, we need to modify the open ai request to have anthropic cache control field.
fn update_request_for_anthropic(original_payload: &Value) -> Value {
    let mut payload = original_payload.clone();

    if let Some(messages_spec) = payload
        .as_object_mut()
        .and_then(|obj| obj.get_mut("messages"))
        .and_then(|messages| messages.as_array_mut())
    {
        // Add "cache_control" to the last and second-to-last "user" messages.
        // During each turn, we mark the final message with cache_control so the conversation can be
        // incrementally cached. The second-to-last user message is also marked for caching with the
        // cache_control parameter, so that this checkpoint can read from the previous cache.
        let mut user_count = 0;
        for message in messages_spec.iter_mut().rev() {
            if message.get("role") == Some(&json!("user")) {
                if let Some(content) = message.get_mut("content") {
                    if let Some(content_str) = content.as_str() {
                        *content = json!([{
                            "type": "text",
                            "text": content_str,
                            "cache_control": { "type": "ephemeral" }
                        }]);
                    }
                }
                user_count += 1;
                if user_count >= 2 {
                    break;
                }
            }
        }

        // Update the system message to have cache_control field.
        if let Some(system_message) = messages_spec
            .iter_mut()
            .find(|msg| msg.get("role") == Some(&json!("system")))
        {
            if let Some(content) = system_message.get_mut("content") {
                if let Some(content_str) = content.as_str() {
                    *system_message = json!({
                        "role": "system",
                        "content": [{
                            "type": "text",
                            "text": content_str,
                            "cache_control": { "type": "ephemeral" }
                        }]
                    });
                }
            }
        }
    }
    payload
}

fn create_request_based_on_model(
    model_config: &ModelConfig,
    system: &str,
    messages: &[Message],
    tools: &[Tool],
) -> anyhow::Result<Value, Error> {
    let mut modified_system = system.to_string();
    if !tools.is_empty() {
        // For providers without native tool calling, embed the list of tools directly into the system prompt.
        modified_system.push_str("\nAvailable tools: ");
        let tools_text = serde_json::to_string_pretty(&tools)
            .unwrap_or_else(|_| "[Error serializing tools]".to_string());
        modified_system.push_str(&tools_text);
        modified_system.push_str(r#"\nWhen you want to use a tool, respond with a JSON object in this format:
{
  "tool": "tool_name",
  "args": {
    "arg1": "value1",
    ...
  }
}

For example:
{
  "tool": "developer__shell",
  "args": {
    "command": "ls -l"
  }
}"#);
    }
    let mut payload = create_request(
        model_config,
        &modified_system,
        messages,
        &[],
        // tools,
        &super::utils::ImageFormat::OpenAi,
    )?;
    // println!("payload: {}", serde_json::to_string_pretty(&payload)?);

    if model_config
        .model_name
        .starts_with(OPENROUTER_MODEL_PREFIX_ANTHROPIC)
    {
        payload = update_request_for_anthropic(&payload);
    }

    Ok(payload)
}

async fn process_tool_calls(message: Message, tool_parser: &ToolParserProvider) -> Message {
    let mut processed = Message {
        role: Role::Assistant,
        created: Utc::now().timestamp(),
        content: vec![],
    };

    // Extract tool calls from the message content
    let text = message.as_concat_text();
    if !text.is_empty() {
        let re = Regex::new(r"\{[^{}]*\}").unwrap(); // Basic regex to find JSON-like structures
        let mut found_valid_json = false;

        for cap in re.find_iter(&text) {
            if let Ok(json) = serde_json::from_str::<Value>(cap.as_str()) {
                if let (Some(tool), Some(args)) = (json.get("tool"), json.get("args")) {
                    if let (Some(_tool_name), Some(_args_obj)) = (tool.as_str(), args.as_object()) {
                        found_valid_json = true;
                        processed.content.push(MessageContent::Text(TextContent {
                            text: serde_json::to_string(&json).unwrap(),
                            annotations: None,
                        }));
                    }
                }
            }
        }

        // If no valid JSON was found, try using the tool parser
        if !found_valid_json {
            if let Ok(tool_calls) = tool_parser.parse_tool_calls(&text).await {
                for tool_call in tool_calls {
                    processed.content.push(MessageContent::Text(TextContent {
                        text: serde_json::to_string(&tool_call).unwrap(),
                        annotations: None,
                    }));
                }
            } else {
                // If tool parser fails, pass through the original text
                processed.content.push(MessageContent::Text(TextContent {
                    text: text,
                    annotations: None,
                }));
            }
        }
    }

    processed
}

#[async_trait]
impl Provider for OpenRouterProvider {
    fn metadata() -> ProviderMetadata {
        ProviderMetadata::new(
            "openrouter",
            "OpenRouter",
            "Router for many model providers",
            OPENROUTER_DEFAULT_MODEL,
            OPENROUTER_KNOWN_MODELS
                .iter()
                .map(|&s| s.to_string())
                .collect(),
            OPENROUTER_DOC_URL,
            vec![
                ConfigKey::new("OPENROUTER_API_KEY", true, true, None),
                ConfigKey::new(
                    "OPENROUTER_HOST",
                    false,
                    false,
                    Some("https://openrouter.ai"),
                ),
            ],
        )
    }

    fn get_model_config(&self) -> ModelConfig {
        self.model.clone()
    }

    #[tracing::instrument(
        skip(self, system, messages, tools),
        fields(model_config, input, output, input_tokens, output_tokens, total_tokens)
    )]
    async fn complete(
        &self,
        system: &str,
        messages: &[Message],
        tools: &[Tool],
    ) -> Result<(Message, ProviderUsage), ProviderError> {
        // Create the base payload
        let payload = create_request_based_on_model(&self.model, system, messages, tools)?;

        // Make request
        let response = self.post(payload.clone()).await?;

        // Parse response
        let message = response_to_message(response.clone())?;
        let message = process_tool_calls(message, &self.tool_parser).await;
        let usage = match get_usage(&response) {
            Ok(usage) => usage,
            Err(ProviderError::UsageError(e)) => {
                tracing::warn!("Failed to get usage data: {}", e);
                Usage::default()
            }
            Err(e) => return Err(e),
        };
        let model = get_model(&response);
        emit_debug_trace(self, &payload, &response, &usage);
        Ok((message, ProviderUsage::new(model, usage)))
    }
}
