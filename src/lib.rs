mod helpers;
mod request;
mod response;
mod url;

use pyo3::prelude::*;

pub const VERSION: &str = "0.1";
pub const CRLF: &str = "\r\n";

/// Kawa. A lightweight http client for Python, written in Rust.
#[pymodule]
fn kawa(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(helpers::request_helper, m)?)?;
    m.add_function(wrap_pyfunction!(helpers::get_helper, m)?)?;
    m.add_function(wrap_pyfunction!(helpers::post_helper, m)?)?;
    m.add_function(wrap_pyfunction!(helpers::put_helper, m)?)?;
    m.add_function(wrap_pyfunction!(helpers::delete_helper, m)?)?;
    m.add_function(wrap_pyfunction!(helpers::connect_helper, m)?)?;
    m.add_function(wrap_pyfunction!(helpers::patch_helper, m)?)?;
    m.add_function(wrap_pyfunction!(helpers::options_helper, m)?)?;
    m.add_function(wrap_pyfunction!(helpers::trace_helper, m)?)?;
    m.add_function(wrap_pyfunction!(helpers::head_helper, m)?)?;
    m.add_class::<request::Request>()?;
    m.add_class::<response::Response>()?;
    m.add_class::<url::Url>()?;
    Ok(())
}
