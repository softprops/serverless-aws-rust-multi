use lambda::handler_fn;
use serde_json::Value;

type Error = Box<dyn std::error::Error + Sync + Send + 'static>;

#[tokio::main]
async fn main() -> Result<(), Error> {
    lambda::run(handler_fn(hello)).await?;
    Ok(())
}

async fn hello(event: Value) -> Result<Value, Error> {
    Ok(event)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[tokio::test]
    async fn hello_handles() {
        let event = json!({
            "answer": 42
        });
        assert_eq!(
            hello(event.clone()).await.expect("expected Ok(_) value"),
            event
        )
    }
}
