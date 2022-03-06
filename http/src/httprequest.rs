use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub enum Method {
    Get,
    Post,
    UnInit,
}

impl From<&str> for Method {
    fn from(s: &str) -> Method {
        match s {
            "GET" => Method::Get,
            "Post" => Method::Post,
            _ => Method::UnInit,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Version {
    V1_1,
    V2_0,
    UnInit,
}

impl From<&str> for Version {
    fn from(s: &str) -> Version {
        match s {
            "HTTP/1.1" => Version::V1_1,
            "HTTP/2.0" => Version::V2_0,
            _ => Version::UnInit,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Resource {
    Path(String),
}

#[derive(Debug)]
pub struct HttpRequest {
    pub method: Method,
    pub version: Version,
    pub resource: Resource,
    pub headers: HashMap<String, String>,
    pub msg_body: String,
}

impl From<String> for HttpRequest {
    fn from(req: String) -> Self {
        let mut parsed_method = Method::UnInit;
        let mut parsed_version = Version::V1_1;
        let mut parsed_resource = Resource::Path("".to_string());
        let mut parsed_headers = HashMap::new();
        let mut parsed_msg_body = "";

        for line in req.lines() {
            if line.contains("HTTP") {
                let (method, version, resource) = parse_req(line);
                parsed_method = method;
                parsed_version = version;
                parsed_resource = resource;
            } else if line.contains(":") {
                let (key, value) = parse_header(line);
                parsed_headers.insert(key, value);
            } else if line.len() == 0 {
                continue;
            } else {
                parsed_msg_body = line;
            }
        }

        HttpRequest {
            method: parsed_method,
            version: parsed_version,
            resource: parsed_resource,
            headers: parsed_headers,
            msg_body: parsed_msg_body.to_string(),
        }
    }
}

fn parse_req(s: &str) -> (Method, Version, Resource) {
    let mut words = s.split_whitespace();
    let method = words.next().unwrap();
    let path = words.next().unwrap();
    let version = words.next().unwrap();
    (
        method.into(),
        version.into(),
        Resource::Path(path.to_string()),
    )
}

fn parse_header(s: &str) -> (String, String) {
    let mut words = s.split(":");
    let mut key = String::from("");
    let mut value = String::from("");
    if let Some(k) = words.next() {
        key = k.trim().to_string();
    }
    if let Some(v) = words.next() {
        value = v.trim().to_string();
    }
    (key, value)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_method_into() {
        let m: Method = "GET".into();
        assert_eq!(m, Method::Get);
    }

    #[test]
    fn test_version_into() {
        let m: Version = "HTTP/1.1".into();
        assert_eq!(m, Version::V1_1);
    }

    #[test]
    fn test_parse_http() {
        let s = String::from("GET /test HTTP/1.1\r\nHost: localhost");
        let mut headers_expected = HashMap::new();
        headers_expected.insert("Host".into(), "localhost".into());
        let req: HttpRequest = s.into();
        assert_eq!(Method::Get, req.method);
        assert_eq!(Version::V1_1, req.version);
        assert_eq!(Resource::Path("/test".to_string()), req.resource);
        assert_eq!(headers_expected, req.headers);
    }
}
