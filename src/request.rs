use std::{
    collections::HashMap,
    io::{Read, Write},
    net::{Shutdown, TcpStream},
};

use pyo3::prelude::*;

use crate::{response::Response, url::Url, CRLF, VERSION};

#[pyclass]
pub struct Request {
    url: Url,
    method: String,
    headers: HashMap<String, String>,
    body: Option<String>,
}

#[pymethods]
impl Request {
    #[new]
    pub fn new(method: String, url: Url) -> Self {
        let default_headers = [
            ("User-Agent".into(), format!("kawa/{VERSION}")),
            ("Host".into(), url.address()),
            ("Connection".into(), "close".into()),
        ];
        Self {
            url,
            method,
            headers: HashMap::from(default_headers),
            body: None,
        }
    }

    pub fn add_header(&mut self, header: String, value: String) {
        self.headers.insert(header, value);
    }

    pub fn set_body(&mut self, body: String) {
        let length = body.len();
        self.body = Some(body);
        self.add_header("Content-Length".into(), length.to_string());
    }

    fn create_message(&self) -> Vec<u8> {
        let mut message = format!("{} {} HTTP/1.1{CRLF}", &self.method, &self.url.path);
        for (header, value) in &self.headers {
            let value = format!("{header}: {value}{CRLF}");
            message.push_str(&value);
        }
        message.push_str(CRLF);

        if let Some(body) = &self.body {
            message.push_str(body);
        }

        let bytes = message.as_bytes();
        bytes.to_owned()
    }

    pub fn send(&self) -> PyResult<Response> {
        let mut stream = TcpStream::connect(self.url.address())?;

        let data = self.create_message();
        stream.write_all(&data)?;

        stream.flush()?;

        let mut buf = String::new();
        stream.read_to_string(&mut buf)?;

        stream.shutdown(Shutdown::Both).ok();

        let response = Response::parse_response(buf)?;
        Ok(response)
    }
}
