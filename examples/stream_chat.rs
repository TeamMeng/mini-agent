use anyhow::Result;
use futures::StreamExt;
use mini_agent::{LLM_MODEL, llm::chat_stream};
use tracing::Level;
use tracing_subscriber::FmtSubscriber;

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv()?;

    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(subscriber)?;

    // let content =
    //     chat_complete(LLM_MODEL, Some("你是一个全能的助手"), "中国的首都在哪里？").await?;
    // println!("Response: {}", content);

    let s = chat_stream(
        LLM_MODEL,
        Some("你是一个全能的助手"),
        "道德经第四章的内容是什么？",
    );

    futures::pin_mut!(s);
    let mut output = String::new();
    while let Some(result) = s.next().await {
        match result {
            Ok(txt) => {
                output.push_str(&txt);
                print!("{}", txt)
            }
            Err(err) => {
                tracing::error!("\nError while streaming: {}", err);
                return Err(err);
            }
        }
    }

    println!("Result: {}", output);

    Ok(())
}
