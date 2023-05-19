use reqwest;

pub async fn get_gpt_response() -> Result<Option<String>, Box<dyn std::error::Error>> {
    let api_key: String = dotenv::var("OPENAI_API_KEY").expect("Error: OPENAI_API_KEY environment variable is not set. Please set it in the .env file. Make sure to obtain an API key from OpenAI (https://platform.openai.com/) and assign it to the OPENAI_API_KEY variable.");
    let prompt = "Hello, how are you?";

    let client = reqwest::Client::new();
    let response = client
        .post("https://api.openai.com/v1/chat/completions")
        .header("Authorization", format!("Bearer {}", api_key))
        .json(&serde_json::json!({
            "prompt": prompt,
            "max_tokens": 50,
            "temperature": 0.8,
            "n": 1,
            "stop": "\n"
        }))
        .send()
        .await?;

    let response_json: serde_json::Value = response.json().await?;
    let choices = response_json["choices"].as_array();
    if let Some(choices) = choices {
        let completion = choices[0]["text"].as_str().map(|s| s.to_owned());
        println!("Generated response: {:?}", completion);
        Ok(completion)
    } else {
        println!("No response received from the GPT API.");
        Ok(None)
    }
}