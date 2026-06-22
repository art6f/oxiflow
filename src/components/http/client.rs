//! HTTP client to perform the requests

use std::{
    error::Error,
    time::{Duration, Instant},
};

use crate::components::http;
use crate::components::worker::request::WorkerRequest;

use crate::components::http::{HttpResult, error::HttpError, response::HttpResponse};
use reqwest::{Client, ClientBuilder, Request};

/// HTTP specific worker, used to call HTTP/HTTPS urlsØ
pub struct HttpClient {
    client: Client,
}

impl HttpClient {
    pub fn new(timeout_sec: u8) -> HttpClient {
        let default_timeout = Duration::from_secs(timeout_sec as u64);

        HttpClient {
            client: ClientBuilder::new()
                .timeout(default_timeout)
                .build()
                .expect("Error creating client"),
        }
    }

    pub fn resolve_request(&self, req: &WorkerRequest) -> Result<Request, Box<dyn Error>> {
        if !http::method_supported(&req.method) {
            return Err(format!("Unsupported method: '{}'", &req.method).into());
        }

        let url = req.url.clone();
        let req = match req.method.trim().to_uppercase().as_str() {
            "GET" => self.client.get(url),
            "POST" => self.client.post(url),
            "PUT" => self.client.put(url),
            "PATCH" => self.client.patch(url),
            "DELETE" => self.client.delete(url),
            _ => panic!("Unmatched method found, previous checks failed. This is a bug!"),
        };

        Ok(req.build().unwrap())
    }

    pub async fn execute_request(&self, request: Request) -> HttpResult {
        let start = Instant::now();
        let method = request.method().to_string();

        let response = self.client.execute(request).await;
        let elapsed = start.elapsed().as_millis();

        response
            .map(|res| HttpResponse::new(res, method.clone(), elapsed, start))
            .map_err(|err| HttpError::new(err, method.clone(), elapsed, start))
    }
}
