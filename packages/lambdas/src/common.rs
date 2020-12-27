use lambda_http::Request;

pub fn extract_key_from_request(event: Request) -> String {
    let path: Vec<&str> = event.uri().path().rsplit('/').collect();
    path.into_iter().next().unwrap().to_string()
}
