use anyhow::Result;
use mini_agent::{LLM_MODEL, chat_complete};
use tracing::Level;
use tracing_subscriber::FmtSubscriber;

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv()?;

    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(subscriber)?;

    let content =
        chat_complete(LLM_MODEL, Some("你是一个全能的助手"), "中国的首都在哪里？").await?;

    println!("Response: {}", content);

    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn func() {
        assert_eq!(1, 1);
    }
}
