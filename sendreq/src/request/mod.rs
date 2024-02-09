use std::collections::HashMap;

const HEADERSEP: &'static str = "\n";
const HEADERKVSEP: &'static str = ":";

pub struct RequestBuilder {
    method: String,
    headers: String,
    body: String,
}

#[derive(Debug, Clone)]
pub struct Request {
    method: String,
    headers: HashMap<String, String>,
    body: String,
}

impl RequestBuilder {
    pub fn new(method: String, headers: String, body: String) -> Self {
        Self {
            method: method,
            headers: headers,
            body: body,
        }
    }

    pub fn build(&self) -> Request {
        let mut req = Request {
            method: self.method.clone(),
            headers: HashMap::new(),
            body: self.body.clone(),
        };

        for header in self.headers.split(HEADERSEP) {
            let keyvals: Vec<&str> = header.split(HEADERKVSEP).collect();
            req.headers
                .insert(keyvals[0].to_string(), keyvals[1].to_string());
        }

        req
    }
}

impl Request {
    pub fn dispatch(&self) {}
}

pub struct Response {
    code: i32,
    message: String,
    err: String,
}
