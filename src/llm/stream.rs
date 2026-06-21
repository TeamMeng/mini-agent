use async_openai::types::chat::{
    ChatCompletionRequestSystemMessageArgs, CreateChatCompletionRequestArgs,
};
use async_stream::stream;
use futures::{Stream, StreamExt};

pub fn chat_stream(
    model: &str,
    system: Option<&str>,
    prompt: &str,
) -> impl Stream<Item = anyhow::Result<String>> {
    stream! {
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

            let mut stream = client.chat().create_stream(request).await?;

            while let Some(response_result) = stream.next().await {
                match response_result {
                    Ok(chunk) => {
                        if let Some(choice) = chunk.choices.first()
                            && let Some(new_text) = &choice.delta.content {
                                yield Ok(new_text.clone())
                        }
                    }
                    Err(err) => yield Err(err.into()),
                }
        }
    }
}
