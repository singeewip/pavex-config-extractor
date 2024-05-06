pub mod ping;

use pavex::blueprint::{router::GET, Blueprint};
use pavex::f;
use pavex::http::StatusCode;

use crate::configuration::DummyConfig;

pub fn register(bp: &mut Blueprint) {
    bp.route(GET, "/api/ping", f!(self::ping::get));
    bp.route(GET, "/test", f!(self::test));
}

pub fn test(_dummy_config: &DummyConfig) -> StatusCode {
    StatusCode::OK
}