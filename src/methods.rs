pub enum HttpMethods {
    GET,
    HEAD,
    POST,
    PUT,
    DELETE,
    CONNECT,
    OPTIONS,
    TRACE,
    PATCH,
}

impl HttpMethods {
    pub fn to_bytes(&self) -> Vec<u8> {
        let response_name = match self {
            HttpMethods::GET => "GET",
            HttpMethods::HEAD => "HEAD",
            HttpMethods::POST => "POST",
            HttpMethods::PUT => "PUT",
            HttpMethods::DELETE => "DELETE",
            HttpMethods::CONNECT => "CONNECT",
            HttpMethods::OPTIONS => "OPTIONS",
            HttpMethods::TRACE => "TRACE",
            HttpMethods::PATCH => "PATCH",
        };
        let bytes = response_name.as_bytes().to_vec();
        bytes
    }
}
