use pyo3::prelude::*;

#[pyclass]
#[derive(Clone)]
pub struct Url {
    host: String,
    port: usize,
    scheme: String,
    pub path: String,
}

impl Url {
    pub fn address(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}

#[pymethods]
impl Url {
    // http(s)://127.0.0.1:8080/
    // https://www.google.com
    // https://google.com
    #[new]
    pub fn parse(url: String) -> PyResult<Self> {
        let supported_schemes: Vec<&str> = vec!["http", "https", "ws", "wss"];
        let scheme_other_split: Vec<&str> = url.split("://").collect();
        let (scheme, other) = (scheme_other_split[0], scheme_other_split[1]);

        assert!(supported_schemes.contains(&scheme));

        let mut port: usize = match scheme {
            "https" => 443,
            "wss" => 443,
            "http" => 80,
            "ws" => 80,
            _ => 443 // we just assume its 443 if we somehow got here
        };

        let mut path: String = "/".into();
        let mut host: String;

        // we're now left with something like this:
        // - (www.)google.com(:80)(/(path))

        // first lets see if a path exists
        // if not we assume its "/"

        let mut other_path_split: Vec<&str> = other.split('/').collect();
        if other_path_split.len() != 1 {
            // there could be a path
            // "/" exists in the url
            host = other_path_split.remove(0).into();
            let possible_path = other_path_split[0];

            if !possible_path.is_empty() {
                let joined_back_path = other_path_split.join("/");
                path = format!("/{joined_back_path}"); // we need the leading slash
            }
        } else {
            host = other_path_split[0].into();
        }

        // now we parse out the port from the host
        if host.contains(':') {
            // contains a colon, possible new port
            let host_port_split: Vec<&str> = host.split(':').collect();
            let new_host = host_port_split[0].into();
            let new_port: usize = host_port_split[1].parse()?;
            port = new_port;
            host = new_host;
        }

        let url_struct = Self {
            host,
            port,
            path,
            scheme: scheme.into()
        };
        
        Ok(url_struct)
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
