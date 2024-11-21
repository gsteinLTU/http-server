use http::{Request, Response};
use http::header::{self, HeaderValue};
use hyper::Body;
use tokio::sync::Mutex;

use std::sync::Arc;

use super::MiddlewareAfter;

pub fn make_cache_control_middleware(max_age: Option<u32>) -> MiddlewareAfter {
    Box::new(
        move |request: Arc<Mutex<Request<Body>>>, response: Arc<Mutex<Response<Body>>>| {
            Box::pin(async move {
                let mut response = response.lock().await;

                if let Some(max_age) = max_age {
                    response.headers_mut().insert(
                        header::CACHE_CONTROL,
                        HeaderValue::from_str(&format!("max-age={}", max_age)).unwrap(),
                    );
                } else {
                    response.headers_mut().insert(
                        header::CACHE_CONTROL,
                        HeaderValue::from_str("no-cache").unwrap(),
                    );
                }

                Ok(())
            })
        },
    )
}