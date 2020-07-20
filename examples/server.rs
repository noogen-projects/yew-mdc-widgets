use std::{ffi::OsStr, fs::File, path::Path, str::FromStr};

use tiny_http::{Header, Response, Server};

fn main() {
    let addr = "127.0.0.1:9080";
    let server = Server::http(addr).unwrap();
    println!("Yew MDC widgets example server started on {}", addr);

    let static_path = Path::new("examples").join("static");

    for request in server.incoming_requests() {
        println!(
            "Received request. Method: {:?}, url: {:?}, headers: {:?}",
            request.method(),
            request.url(),
            request.headers()
        );

        if request.url() == "/favicon.ico" {
            request.respond(Response::empty(404)).unwrap();
            continue;
        }

        let file_path = if request.url().len() > 1 {
            static_path.join(request.url().trim_start_matches('/'))
        } else {
            static_path.join("index.html")
        };

        println!("    Response file: {}", file_path.display());
        let mut response = Response::from_file(File::open(&file_path).unwrap());
        let content_type = match file_path.extension().and_then(OsStr::to_str) {
            Some("js") => "Content-Type: application/javascript",
            Some("wasm") => "Content-Type: application/wasm",
            Some("html") => "Content-Type: text/html",
            Some("css") => "Content-Type: text/css",
            _ => "Content-Type: text/plain",
        };
        response.add_header(Header::from_str(content_type).unwrap());

        request.respond(response).unwrap();
    }
}
