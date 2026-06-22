//! Worker result module. This isn't `std::result::Result` and not related to it.
//!
//! This result stores how many actuall server reponses were receiver, how many failed
//! and what what the average response time for each HTTP code.

#![allow(clippy::print_stderr, clippy::print_stdout)]

use crate::components::http::{error::HttpError, response::HttpResponse};

use self::{single::ResultSingle, totals::ResultTotals};

mod single;
mod totals;

#[derive(Default)]
pub struct WorkerResult {
    /// per-requests detail
    pub responses: Vec<ResultSingle>,
    
    /// total requests count
    pub totals: ResultTotals,
}

impl WorkerResult {
    pub fn new() -> WorkerResult {
        WorkerResult::default()
    }

    pub fn add_success(&mut self, response: &HttpResponse) {
        self.totals.count_response(response);

        self.responses.push(ResultSingle::success(
            response.url.clone(),
            response.method.clone(),
            response.code,
            response.response_time,
        ));
    }

    pub fn add_failure(&mut self, response: &HttpError) {
        self.totals.inc_error();

        self.responses.push(ResultSingle::failure(
            response.url.clone(),
            response.method.clone(),
            response.timeout
        ));
    }
}
