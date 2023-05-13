pub fn url_to_string( url: url::Url) -> String {
    let scheme = url.scheme();
    let host = url.host().unwrap();
    let port = url.port().unwrap_or(if scheme == "https" { 443 } else { 80 });
    let path = url.path();
    
    match port {
        80 | 443 => format!("{}://{}{}", scheme, host, path),
        _ => format!("{}://{}:{}{}", scheme, host, port, path),
    }
}