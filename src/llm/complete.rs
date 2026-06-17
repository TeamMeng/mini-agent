use anyhow::Result;
use async_openai::types::chat::{
    ChatCompletionRequestSystemMessageArgs, CreateChatCompletionRequestArgs,
};

pub async fn chat_complete(model: &str, system: Option<&str>, prompt: &str) -> Result<String> {
    let client = async_openai::Client::new();
    let mut messages = Vec::new();

    if let Some(system) = system {
        messages.push(
            ChatCompletionRequestSystemMessageArgs::default()
                .content(system)
                .build()?
                .into(),
        );
    }

    messages.push(
        ChatCompletionRequestSystemMessageArgs::default()
            .content(prompt)
            .build()?
            .into(),
    );

    let request = CreateChatCompletionRequestArgs::default()
        .model(model)
        .messages(messages)
        .max_tokens(2048u32)
        .build()?;

    let response = client.chat().create(request).await?;

    tracing::info!("Response: {:#?}", response);

    let content = response
        .choices
        .into_iter()
        .next()
        .and_then(|c| c.message.content)
        .ok_or_else(|| anyhow::anyhow!("No content in response"))?;

    Ok(content)
}
