mod request;
mod response;
mod url;

use std::collections::HashMap;

use pyo3::prelude::*;

pub const VERSION: &str = "0.1";
pub const CRLF: &str = "\r\n";

/// Send an HTTP request
#[pyfunction(name = "request")]
#[pyo3(signature = (method, host, port, path, headers=None, body=None))]
fn request_helper(
    method: String,
    host: String,
    port: usize,
    path: String,
    headers: Option<HashMap<String, String>>,
    body: Option<String>,
) -> PyResult<response::Response> {
    let mut req = request::Request::new(method, host, port, path);

    if let Some(body) = body {
        req.set_body(body);
    }
    if let Some(headers) = headers {
        for (header, value) in headers {
            req.add_header(header, value);
        }
    }

    let resp = req.send()?;
    Ok(resp)
}

/// Kawa. A lightweight http client for Python, written in Rust.
#[pymodule]
fn kawa(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(request_helper, m)?)?;
    m.add_class::<request::Request>()?;
    m.add_class::<response::Response>()?;
    m.add_class::<url::Url>()?;
    Ok(())
}
