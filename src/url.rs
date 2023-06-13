use pyo3::prelude::*;

#[pyclass]
pub struct Url {
    host: String,
    port: usize,
    scheme: String,
    path: String
}

#[pymethods]
impl Url {
    // http(s)://127.0.0.1:8080/
    #[new]
    pub fn parse(url: String) -> PyResult<Self> {
        let supported_schemes: Vec<&str> = vec!["http", "https", "ws", "wss"];

        let components = url.split("://").collect::<Vec<&str>>();
        let scheme = components[0];
        let parts = components[1];
        
        assert!(supported_schemes.contains(&scheme));
        
        // now we are left with just the 127.0.0.1:8080/ section to deal with
        let host_port = parts.split(':').collect::<Vec<&str>>();
        let (host, port_part) = (host_port[0], host_port[1]);

        // now we need to extract the port section
        // there COULD be a path, there might not be
        // if there isn't a path, we simply assume its "/"

        let mut port_path = port_part.split('/').collect::<Vec<&str>>();
        let port: usize = port_path[0].parse()?;
        port_path.remove(0);

        let mut path: String = "/".into();

        if !port_path.is_empty() {
            // seems like there was a path
            if port_path.len() > 1 {
                // we have a path with the following structure: /x/y/z
                let mut path_inner = port_path.join("/");
                if !path_inner.starts_with('/') {
                    path_inner = format!("/{path_inner}");
                }
                path = path_inner;
            } else {
                // we have a single path: /x
                let mut path_inner = port_path[0].to_string();
                if !path_inner.is_empty() {
                    if !path_inner.starts_with('/') {
                        path_inner = format!("/{path_inner}");
                    }
                    path = path_inner;
                }
            }
        }

        let url = Self {
            host: host.into(),
            port,
            scheme: scheme.into(),
            path
        };
        Ok(url)
    }

    #[getter]
    fn get_host(&self) -> String {
        self.host.clone()
    }

    #[getter]
    fn get_port(&self) -> usize {
        self.port
    }

    #[getter]
    fn get_path(&self) -> String {
        self.path.clone()
    }

    #[getter]
    fn scheme(&self) -> String {
        self.scheme.clone()
    }
}