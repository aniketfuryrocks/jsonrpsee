//! Various middleware implementations for RPC specific purposes.

/// Proxy `GET /path` to internal RPC methods.
pub mod proxy_get_request;
/// Proxy switch requset
pub mod request_middleware;
