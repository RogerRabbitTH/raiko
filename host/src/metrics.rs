use lazy_static::lazy_static;
use prometheus::{
    labels, register_histogram_vec, register_int_counter_vec, register_int_gauge, HistogramVec,
    IntCounterVec, IntGauge,
};

use crate::request::ProofType;

lazy_static! {
    pub static ref HOST_REQ_COUNT: IntCounterVec = register_int_counter_vec!(
        "host_request_count",
        "the number of requests sent to the host",
        &["block_id"]
    )
    .unwrap();
    pub static ref HOST_ERROR_COUNT: IntCounterVec = register_int_counter_vec!(
        "host_error_count",
        "the number of failed requests produced by the host",
        &["block_id"]
    )
    .unwrap();
    pub static ref GUEST_PROOF_REQ_COUNT: IntCounterVec = register_int_counter_vec!(
        "guest_proof_request_count",
        "the number of requests sent to this guest",
        &["guest", "block_id"]
    )
    .unwrap();
    pub static ref GUEST_PROOF_SUCCESS_COUNT: IntCounterVec = register_int_counter_vec!(
        "guest_proof_success_count",
        "the number of successful proofs generated by this guest",
        &["guest", "block_id"]
    )
    .unwrap();
    pub static ref GUEST_PROOF_ERROR_COUNT: IntCounterVec = register_int_counter_vec!(
        "guest_proof_error_count",
        "the number of failed proofs generated by this guest",
        &["guest", "block_id"]
    )
    .unwrap();
    pub static ref GUEST_PROOF_TIME: HistogramVec = register_histogram_vec!(
        "guest_proof_time_histogram",
        "time taken for proof generation by this guest",
        &["guest", "block_id", "success"]
    )
    .unwrap();
    pub static ref PREPARE_INPUT_TIME: HistogramVec = register_histogram_vec!(
        "prepare_input_time_histogram",
        "time taken for prepare input",
        &["block_id", "success"]
    )
    .unwrap();
    pub static ref TOTAL_TIME: HistogramVec = register_histogram_vec!(
        "total_time_histogram",
        "time taken for the whole request",
        &["block_id", "success"]
    )
    .unwrap();
    pub static ref CONCURRENT_REQUESTS: IntGauge = register_int_gauge!(
        "concurrent_requests",
        "number of requests currently being processed"
    )
    .unwrap();
}

/// Increase the count of requests currently being processed.
pub fn inc_current_req() {
    CONCURRENT_REQUESTS.inc();
}

/// Decrease the count of requests currently being processed.
pub fn dec_current_req() {
    CONCURRENT_REQUESTS.dec();
}

/// Increment the request count for the host.
pub fn inc_host_req_count(block_id: u64) {
    let block_id = block_id.to_string();
    let labels = labels! {
        "block_id" => block_id.as_str(),
    };
    HOST_REQ_COUNT.with(&labels).inc();
}

/// Increment the error count for the host.
pub fn inc_host_error(block_id: u64) {
    let block_id = block_id.to_string();
    let labels = labels! {
        "block_id" => block_id.as_str(),
    };
    HOST_ERROR_COUNT.with(&labels).inc();
}

/// Increment the request count for the given guest.
pub fn inc_guest_req_count(guest: &ProofType, block_id: u64) {
    let guest = guest.to_string();
    let block_id = block_id.to_string();
    let labels = labels! {
        "guest" => guest.as_str(),
        "block_id" => &block_id,
    };
    GUEST_PROOF_REQ_COUNT.with(&labels).inc();
}

/// Increment the success count for the given guest.
pub fn inc_guest_success(guest: &ProofType, block_id: u64) {
    let guest = guest.to_string();
    let block_id = block_id.to_string();
    let labels = labels! {
        "guest" => guest.as_str(),
        "block_id" => &block_id,
    };
    GUEST_PROOF_SUCCESS_COUNT.with(&labels).inc();
}

/// Increment the error count for the given guest.
pub fn inc_guest_error(guest: &ProofType, block_id: u64) {
    let guest = guest.to_string();
    let block_id = block_id.to_string();
    let labels = labels! {
        "guest" => guest.as_str(),
        "block_id" => &block_id,
    };
    GUEST_PROOF_ERROR_COUNT.with(&labels).inc();
}

/// Observe the time taken for the given guest to generate a proof.
pub fn observe_guest_time(guest: &ProofType, block_id: u64, time: u128, success: bool) {
    let guest = guest.to_string();
    let block_id = block_id.to_string();
    let success = success.to_string();
    let labels = labels! {
        "guest" => guest.as_str(),
        "block_id" => &block_id,
        "success" => &success,
    };
    GUEST_PROOF_TIME.with(&labels).observe(time as f64);
}

/// Observe the time taken for prepare input.
pub fn observe_prepare_input_time(block_id: u64, time: u128, success: bool) {
    let block_id = block_id.to_string();
    let success = success.to_string();
    let labels = labels! {
        "block_id" => block_id.as_str(),
        "success" => &success,
    };
    PREPARE_INPUT_TIME.with(&labels).observe(time as f64);
}

/// Observe the time taken for prepare input.
pub fn observe_total_time(block_id: u64, time: u128, success: bool) {
    let block_id = block_id.to_string();
    let success = success.to_string();
    let labels = labels! {
        "block_id" => block_id.as_str(),
        "success" => &success,
    };
    TOTAL_TIME.with(&labels).observe(time as f64);
}
