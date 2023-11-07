struct Request {
    method: String,
    uri: String,
    headers: Vec<String>,
}

struct Response {
    status: u8,
    body: String,
}

impl Request {
    fn execute_request(&self) -> Response {
        
    }
}