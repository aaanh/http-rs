use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{SocketAddr, TcpListener, TcpStream},
    path::Path,
    process::Command,
};
mod lib;

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|res| res.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    let request_line = &http_request[0]; // Get the first line of the request

    let path = lib::parse_requested_file(&request_line); // Extract file path

    let (status_line, filename, content_type) = if Path::new(&path).exists() {
        (
            "HTTP/1.1 200 OK",
            path.clone(),
            lib::get_content_type(&path),
        )
    } else {
        (
            "HTTP/1.1 404 NOT FOUND",
            "web/404.html".to_string(),
            "text/html",
        )
    };
    let contents = fs::read_to_string(filename).unwrap_or_else(|_| String::from("File not found"));

    let length = contents.len();

    let response = format!(
        "{status_line}\r\nContent-Length: {length}\r\nContent-Type: {content_type}\r\n\r\n{contents}"
    );

    let splitted_request_line: Vec<_> = request_line.split_whitespace().collect();

    let request_route = &splitted_request_line[1];

    if request_route == &"/api/status" {
        let contents = "Status OK, 200, API server is up";

        let response = format!("{status_line}\r\nContent-Length: {length}\r\nContent-Type: {content_type}\r\n\r\n{contents}");

        stream.write_all(response.as_bytes()).unwrap();
    }

    stream.write_all(response.as_bytes()).unwrap();

    println!("Request: {http_request:#?}")
}

fn main() {
    let addrs = [SocketAddr::from(([127, 0, 0, 1], 8080))];

    let listener = TcpListener::bind(&addrs[..]).unwrap();

    println!("Server started on: {addrs:#?}");

    let exec_tailwind = Command::new("tailwindcss")
        .arg("-i web/globals.css -o web/output.css")
        .output()
        .expect("Failed to execute process");

    let exec_tailwind_out = exec_tailwind.stdout;

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}
