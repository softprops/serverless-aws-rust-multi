use lambda_http::{service_fn, IntoResponse, Request};
use serde_json::json;

type Error = Box<dyn std::error::Error + Sync + Send + 'static>;

#[tokio::main]
async fn main() -> Result<(), Error> {
    lambda_http::run(service_fn(world)).await?;
    Ok(())
}

async fn world(_: Request) -> Result<impl IntoResponse, Error> {
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
        let response = world(request)
            .await
            .expect("expected Ok(_) value")
            .into_response();
        assert_eq!(response.await.body(), expected.await.body())
    }
}
