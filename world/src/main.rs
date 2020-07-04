use lambda_http::{handler, lambda, IntoResponse, Request, Context};
use serde_json::json;

type Error = Box<dyn std::error::Error + Sync + Send + 'static>;

#[tokio::main]
async fn main() -> Result<(), Error> {
    lambda::run(handler(world)).await?;
    Ok(())
}

async fn world(_: Request, _: Context) -> Result<impl IntoResponse, Error> {
    // `serde_json::Values` impl `IntoResponse` by default
    // creating an application/json response
    Ok(json!({
    "message": "Go Serverless v1.0! Your function executed successfully!"
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn world_handles() {
        let request = Request::default();
        let expected = json!({
        "message": "Go Serverless v1.0! Your function executed successfully!"
        })
        .into_response();
        let response = world(request, Context::default())
            .await
            .expect("expected Ok(_) value")
            .into_response();
        assert_eq!(response.body(), expected.body())
    }
}
