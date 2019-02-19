use tiny_http::{Header, Request, Response, Server};

fn serve(path: &str, request: Request) {
    let mime = mime_guess::guess_mime_type(path);
    println!("mime: {}", mime);
    let data = match std::fs::read(&path) {
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
    let server = Server::http("0.0.0.0:8000").unwrap();

    for request in server.incoming_requests() {
        let url = request.url();
        println!("{:?}", url);
        let corrected_path = if url == "/" {
            "index.html".to_owned()
        } else if url.find('.').is_some() {
            format!(".{}", url)
        } else {
            format!("content{}.html", url)
        };
        println!("[c] {:?}", corrected_path);
        serve(&corrected_path, request);
    }
}
