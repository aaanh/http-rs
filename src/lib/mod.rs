// Function to parse the requested file from the HTTP request
pub fn parse_requested_file(request_line: &str) -> String {
    // Extract the path from the "GET /path HTTP/1.1" request
    let parts: Vec<&str> = request_line.split_whitespace().collect();
    let path = parts.get(1).unwrap_or(&"/");

    // Default to index.html if requesting the root "/"
    if *path == "/" {
        "web/index.html".to_string()
    } else {
        // Construct the path relative to the "web" directory
        format!("web{}", path)
    }
}

// Function to determine the Content-Type based on file extension
pub fn get_content_type(path: &str) -> &'static str {
    if path.ends_with(".html") {
        "text/html"
    } else if path.ends_with(".css") {
        "text/css"
    } else if path.ends_with(".js") {
        "application/javascript"
    } else if path.ends_with(".png") {
        "image/png"
    } else if path.ends_with(".jpg") || path.ends_with(".jpeg") {
        "image/jpeg"
    } else if path.ends_with(".gif") {
        "image/gif"
    } else if path.ends_with(".svg") {
        "image/svg+xml"
    } else {
        "application/octet-stream" // Default binary content type
    }
}
