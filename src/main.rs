use log::error;
use openai_api_rust::*;
use openai_api_rust::chat::*;
use serde::{Deserialize, Serialize};
use std::env;
use std::fs;

#[derive(Serialize, Deserialize, Debug)]
struct Config {
    model: String,
}

impl Config {
    fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let config_dir = dirs::config_dir()
            .ok_or("Could not find config directory")?
            .join("tellme");
        fs::create_dir_all(&config_dir)?;

        let config_file = config_dir.join("config.json");
        let config: Config = if config_file.exists() {
            let contents = fs::read_to_string(&config_file)?;
            serde_json::from_str(&contents)?
        } else {
            let default_config = Config {
                model: "gpt-4o".to_string(),
            };
            let json = serde_json::to_string_pretty(&default_config)?;
            fs::write(&config_file, json)?;
            default_config
        };

        Ok(config)
    }
}

async fn get_openai_response(openai: &OpenAI, config: &Config, prompt: &str) -> Result<String, Box<dyn std::error::Error>> {
    let body = ChatBody {
        model: config.model.clone(),
        max_tokens: Some(50),
        temperature: Some(0.7),
        top_p: Some(1.0),
        n: Some(1),
        stream: Some(false),
        stop: None,
        presence_penalty: None,
        frequency_penalty: None,
        logit_bias: None,
        user: None,
        messages: vec![Message {
            role: Role::User,
            content: format!("You are part of the user's command prompt. If the answer involves a command, respond with just the command itself (no formatting). Provide a concise, one-line response to: {}
", prompt),
        }],
    };

    let response = openai.chat_completion_create(&body)
        .map_err(|e| format!("Failed to get OpenAI response: {}", e))?;

    response.choices.first()
        .and_then(|choice| choice.message.as_ref())
        .map(|message| message.content.trim().to_string())
        .ok_or_else(|| "No response from OpenAI".into())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::new()?;

    let auth = Auth::from_env().map_err(|e| format!("Failed to load API key from environment: {}", e))?;
    let openai = OpenAI::new(auth, "https://api.openai.com/v1/");

    let prompt = env::args().skip(1).collect::<Vec<String>>().join(" ");
    if prompt.is_empty() {
        println!("Usage: tellme <your question or command>");
        return Ok(());
    }

    match get_openai_response(&openai, &config, &prompt).await {
        Ok(response) => {
            println!("{}", response);
        }
        Err(e) => {
            error!("Error: {:?}", e);
        }
    }

    Ok(())
}