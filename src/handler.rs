use crate::config::get_key;
use crate::verify::verify_signature;
use axum::{
    body::{Body, Bytes},
    http::{HeaderMap, StatusCode},
    response::{IntoResponse, Response},
};
pub async fn webhook_handler(headers: HeaderMap, body: Bytes) -> impl IntoResponse {
    //println!("webhook received {:#}", payload);

    let signature = match headers.get("X-hub-Signature-256") {
        Some(sig) => match sig.to_str() {
            Ok(ss) => ss,
            Err(_) => {
                return Response::builder() //https://docs.rs/axum/latest/axum/response/index.html
                    .status(StatusCode::BAD_REQUEST)
                    .body(Body::from("Invalid signature header")) //.body("Invalid signature header".into()) is also valid
                    .unwrap();
            }
        },
        None => {
            return Response::builder()
                .status(StatusCode::BAD_REQUEST)
                .body("Missing header".into())
                .unwrap();
        }
    };

    let payload = match std::str::from_utf8(&body) {
        Ok(s) => s,
        Err(_) => {
            return match Response::builder()
                .status(StatusCode::BAD_REQUEST)
                .body("bytes to str failed".into())
            {
                // Intead of using unrwap we return empty body (just to practice , it would be rare
                // that the Err above fails)
                Ok(s) => s,
                Err(_) => Response::builder()
                    .status(StatusCode::BAD_REQUEST)
                    .body("bytes to str failed".into())
                    .unwrap_or_else(|_| Response::new(Body::empty())),
            };
        }
    };

    let key = get_key();
    if !verify_signature(&key, payload, signature) {
        return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body("Signature verification failed".into())
            .unwrap();
    }
    println!("Webhook verification successful! {}", payload);

    Response::builder()
        .status(StatusCode::OK)
        .body("Webhook processing successful".into())
        .unwrap()
}
