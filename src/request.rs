use http::client::RequestWriter;
use http::method::Get;
use url::Url;

pub fn request(url: &str) -> String {
    let url = match Url::parse(url) {
        Ok(x) => x,
        Err(e) => panic!("Invalid URL: {}", e),
    };
    let request: RequestWriter = RequestWriter::new(Get, url).unwrap();

    let mut response = match request.read_response() {
        Ok(response) => response,
        Err(_request) => panic!("No response received"),
    };
    let body = match response.read_to_string() {
        Ok(body) => body,
        Err(err) => panic!("Reading response failed: {}", err),
    };

    body
}
