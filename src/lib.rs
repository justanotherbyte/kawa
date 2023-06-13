mod request;

use pyo3::prelude::*;

pub const VERSION: &str = "0.1";

/// Send an HTTP request
#[pyfunction(name = "request")]
fn request_helper(method: String, host: String, port: usize, path: String) -> PyResult<()> {
    let req = request::Request::new(
        method, host, port, path
    );
    req.send()
}

/// Kawa. A lightweight http client for Python, written in Rust.
#[pymodule]
fn kawa(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(request_helper, m)?)?;
    m.add_class::<request::Request>()?;
    Ok(())
}
