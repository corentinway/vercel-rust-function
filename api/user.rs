use http::{StatusCode};
use std::error::Error;
use vercel_lambda::lambda;
use vercel_lambda::Request;
use vercel_lambda::Response;
use vercel_lambda::IntoResponse;
use vercel_lambda::error::VercelError;

fn handler(_: Request) -> Result<impl IntoResponse, VercelError> {
    let response = Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "text/plain")
        .body("Hello World")
        .expect("Internal Server Error");
    
    Ok(response)
}

fn main() -> Result<(), Box<dyn Error>> {
    Ok(lambda!(handler))
}