use std::collections::HashMap;

use pyo3::prelude::*;

use crate::{request, response, url};

/// Send a HTTP request
#[pyfunction(name = "request")]
#[pyo3(signature = (method, url, headers=None, body=None))]
pub fn request_helper(
    method: String,
    url: String,
    headers: Option<HashMap<String, String>>,
    body: Option<String>,
) -> PyResult<response::Response> {
    let url = url::Url::parse(url)?;
    let mut req = request::Request::new(method, url);

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

/// Send a GET request
#[pyfunction(name = "get")]
#[pyo3(signature = (url, headers=None))]
pub fn get_helper(
    url: String,
    headers: Option<HashMap<String, String>>,
) -> PyResult<response::Response> {
    request_helper("GET".into(), url, headers, None)
    // Protocol states that GET is for retrieving data
    // not sending data
}

/// Send a HEAD request
#[pyfunction(name = "head")]
#[pyo3(signature = (url, headers=None, body=None))]
pub fn head_helper(
    url: String,
    headers: Option<HashMap<String, String>>,
    body: Option<String>,
) -> PyResult<response::Response> {
    request_helper("HEAD".into(), url, headers, body)
}

/// Send a POST request
#[pyfunction(name = "post")]
#[pyo3(signature = (url, headers=None, body=None))]
pub fn post_helper(
    url: String,
    headers: Option<HashMap<String, String>>,
    body: Option<String>,
) -> PyResult<response::Response> {
    request_helper("POST".into(), url, headers, body)
}

/// Send a PUT request
#[pyfunction(name = "put")]
#[pyo3(signature = (url, headers=None, body=None))]
pub fn put_helper(
    url: String,
    headers: Option<HashMap<String, String>>,
    body: Option<String>,
) -> PyResult<response::Response> {
    request_helper("PUT".into(), url, headers, body)
}

/// Send a DELETE request
#[pyfunction(name = "delete")]
#[pyo3(signature = (url, headers=None, body=None))]
pub fn delete_helper(
    url: String,
    headers: Option<HashMap<String, String>>,
    body: Option<String>,
) -> PyResult<response::Response> {
    request_helper("DELETE".into(), url, headers, body)
}

/// Send a CONNECT request
#[pyfunction(name = "connect")]
#[pyo3(signature = (url, headers=None, body=None))]
pub fn connect_helper(
    url: String,
    headers: Option<HashMap<String, String>>,
    body: Option<String>,
) -> PyResult<response::Response> {
    request_helper("CONNECT".into(), url, headers, body)
}

/// Send a OPTIONS request
#[pyfunction(name = "options")]
#[pyo3(signature = (url, headers=None, body=None))]
pub fn options_helper(
    url: String,
    headers: Option<HashMap<String, String>>,
    body: Option<String>,
) -> PyResult<response::Response> {
    request_helper("OPTIONS".into(), url, headers, body)
}

/// Send a TRACE request
#[pyfunction(name = "trace")]
#[pyo3(signature = (url, headers=None, body=None))]
pub fn trace_helper(
    url: String,
    headers: Option<HashMap<String, String>>,
    body: Option<String>,
) -> PyResult<response::Response> {
    request_helper("TRACE".into(), url, headers, body)
}

/// Send a PATCH request
#[pyfunction(name = "patch")]
#[pyo3(signature = (url, headers=None, body=None))]
pub fn patch_helper(
    url: String,
    headers: Option<HashMap<String, String>>,
    body: Option<String>,
) -> PyResult<response::Response> {
    request_helper("PATCH".into(), url, headers, body)
}