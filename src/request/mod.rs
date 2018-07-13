use std::collections::HashMap;
use std::str;

pub struct HttpRequest {
    get_arguments: HashMap<String, String>,
    headers: HashMap<String, String>,
    post_arguments: HashMap<String, String>,
    request_line: HttpRequestLine
}

#[derive(Debug)]
pub struct HttpRequestLine {
    method: HttpRequestMethod,
    protocol: HttpRequestProtocol,
    request_uri: String
}

#[derive(Debug)]
enum HttpRequestMethod {
    Connect,
    Delete,
    Get,
    Head,
    Invalid,
    Options,
    Patch,
    Post,
    Put,
    Trace
}

#[derive(Debug)]
enum HttpRequestProtocol {
    Invalid,
    TwoDotZero,
    OneDotZero,
    OneDotOne,
    ZeroDotNine,
}

enum HttpRequestSection {
    RequestLine,
    HeaderFields,
    MessageBody
}

impl HttpRequest {

    pub fn get_request_line(line: &str) -> Option<HttpRequestLine> {
        let line = line.trim();
        let mut request_method: HttpRequestMethod = HttpRequestMethod::Invalid;
        let request_uri: String = "".to_string();
        let mut request_protocol: HttpRequestProtocol = HttpRequestProtocol::Invalid;
        let mut error = false;

        let parts: Vec<&str> = line.split(" ").collect();
        println!("Collect request-line parts {:?}", &parts);
        if parts.len() > 1 {

            // Parse method
            if let Some(method_string) = parts.get(0) {
                println!("method_string: {}", method_string);
                request_method = match method_string.as_ref() {
                    "CONNECT" => HttpRequestMethod::Connect,
                    "DELETE" => HttpRequestMethod::Delete,
                    "GET" => HttpRequestMethod::Get,
                    "HEAD" => HttpRequestMethod::Head,
                    "OPTIONS" => HttpRequestMethod::Options,
                    "PATCH" => HttpRequestMethod::Patch,
                    "PUT" => HttpRequestMethod::Put,
                    "POST" => HttpRequestMethod::Post,
                    "TRACE" => HttpRequestMethod::Trace,
                    __ => {
                        error = true;
                        HttpRequestMethod::Invalid
                    }
                };
            } else {
                error = true;
            }

            // Parse request URI
            if let Some(request_uri) = parts.get(1) {
                println!("Found request uri: {}", &request_uri);
            } else {
                error = true;
            }

            // Parse protocol here
            if let Some(request_protocol_string) = parts.get(2) {
                request_protocol = match request_protocol_string.as_ref() {
                    "HTTP/0.9" => HttpRequestProtocol::ZeroDotNine,
                    "HTTP/1.0" => HttpRequestProtocol::OneDotZero,
                    "HTTP/1.1" => HttpRequestProtocol::OneDotOne,
                    "HTTP/2.0" => HttpRequestProtocol::TwoDotZero,
                    _ => {
                        error = true;
                        HttpRequestProtocol::Invalid
                    }
                };
                println!("Found protocol: {:?}", &request_protocol);
            } else {
                error = true;
            }

            if !error {
                println!("No error, proceeding to header fields");
                return Some(HttpRequestLine {
                    method: request_method,
                    protocol: request_protocol,
                    request_uri: request_uri
                });
            }
        }
        None
    }

    // TODO Implement this
    pub fn from_tcp_stream(request: &[u8]) -> Option<HttpRequest> {
        if let Ok(request) = str::from_utf8(request) {
            println!("Request as string: {}", request);
            if request.is_ascii() {
                println!("Request is ASCII");
                let mut section = HttpRequestSection::RequestLine;
                let mut error = false;
                for mut line in request.lines() {
                    match section {
                        HttpRequestSection::RequestLine => {
                            if let Some(request_line) = HttpRequest::get_request_line(line) {
                                section = HttpRequestSection::HeaderFields;
                            } else {
                                error = true;
                            }
                        },
                        HttpRequestSection::HeaderFields => {
                            if line.trim().is_empty() {
                                section = HttpRequestSection::MessageBody;
                            } else {
                                // TODO: Do stuff here
                            }
                        },
                        HttpRequestSection::MessageBody => {
                            if !line.is_empty() {
                                // TODO: Do stuff here
                            }
                        }
                    }
                    if error {
                        break;
                    }
                }
            }
        }
        None
    }
}

#[cfg(test)]
mod request_test {
    use super::*;

    #[test]
    fn test_get_request_line() {
        let response = HttpRequest::get_request_line(
            "GET / HTTP/0.9\r\n"
        );
        assert!(response.is_some());

        let response = HttpRequest::get_request_line(
            "GET / HTTP/1.0\r\n"
        );
        assert!(response.is_some());

        let response = HttpRequest::get_request_line(
            "GET / HTTP/1.1\r\n"
        );
        assert!(response.is_some());

        let response = HttpRequest::get_request_line(
            "GET / HTTP/2.0\r\n"
        );
        assert!(response.is_some());

        let response = HttpRequest::get_request_line(
            "GET / HTTP/2.2\r\n"
        );
        assert!(response.is_none());

    }
}



