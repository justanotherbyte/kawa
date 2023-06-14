use std::collections::HashMap;

use pyo3::prelude::*;

use crate::CRLF;

#[pyclass]
pub struct Response {
    status: usize,
    headers: HashMap<String, String>,
    body: String,
}

impl Response {
    pub fn parse_response(buf: String) -> PyResult<Self> {
        let lines = buf.split(CRLF);
        let mut headers = HashMap::new();

        let mut status_code_parsed: usize = 0;
        let mut headers_complete = false;
        let mut body = String::new();

        for (idx, line) in lines.into_iter().enumerate() {
            if idx == 0 {
                // first line, get the status code and status message
                let components = line.split(' ').collect::<Vec<&str>>();
                let status_code = components[1];
                status_code_parsed = status_code.parse()?;
            } else {
                // we're either on the headers or the body
                if line.is_empty() {
                    headers_complete = true;
                }
                if headers_complete {
                    body.push_str(line);
                } else {
                    let mut components = line.split(": ").collect::<Vec<&str>>();
                    let header = components[0];
                    components.remove(0);
                    let value = components.join("");
                    headers.insert(header.to_string(), value);
                }
            }
        }

        let resp = Self {
            status: status_code_parsed,
            headers,
            body,
        };
        Ok(resp)
    }
}

#[pymethods]
impl Response {
    #[getter]
    fn get_headers(&self) -> HashMap<String, String> {
        self.headers.clone()
    }
    #[getter]
    fn get_status(&self) -> usize {
        self.status
    }
    #[getter]
    fn get_body(&self) -> String {
        self.body.clone()
    }

    fn __repr__(&self) -> String {
        format!("<Response {}>", self.status)
    }
}
