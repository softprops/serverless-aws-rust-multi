use lambda_runtime::{service_fn, LambdaEvent};
use serde_json::Value;

type Error = Box<dyn std::error::Error + Sync + Send + 'static>;

#[tokio::main]
async fn main() -> Result<(), Error> {
    lambda_runtime::run(service_fn(hello)).await?;
    Ok(())
}

async fn hello(event: LambdaEvent<Value>) -> Result<Value, Error> {
    Ok(event.payload)
}

#[cfg(test)]
mod tests {
    use super::*;
    use lambda_runtime::Context;
    use serde_json::json;

    #[tokio::test]
    async fn hello_handles() {
        let event = LambdaEvent::new(json!({"answer": 42}), Context::default());
        assert_eq!(
            hello(event.clone()).await.expect("expected Ok(_) value"),
            event.payload
        )
    }
}
