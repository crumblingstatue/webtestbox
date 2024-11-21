use tiny_http::{Header, Request, Response, Server};

fn serve(path: &str, request: Request) {
    let mime = mime_guess::from_path(path).first_or_octet_stream();
    let data = match std::fs::read(path) {
        Ok(data) => data,
        Err(e) => {
            let s = format!("{}: {}", path, e);
            request.respond(Response::from_string(s)).unwrap();
            return;
        }
    };
    let mut response = Response::from_data(data);
    response.add_header(Header::from_bytes("Content-Type", mime.to_string()).unwrap());
    if let Err(e) = request.respond(response) {
        eprintln!("RESPONSE ERROR: {}", e);
    }
}

fn main() {
    let addr = "0.0.0.0:8000";
    println!("Serving on {}", addr);
    let server = Server::http(addr).unwrap();

    for request in server.incoming_requests() {
        let url = request.url();
        let corrected_path = if url == "/" {
            "index.html".to_owned()
        } else if url.find('.').is_some() {
            format!(".{}", url)
        } else {
            format!("content{}.html", url)
        };
        serve(&corrected_path, request);
    }
}
