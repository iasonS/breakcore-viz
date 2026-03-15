use tiny_http::{Server, Response, Header};

fn main() {
    let server = Server::http("0.0.0.0:3000").expect("Failed to start server on port 3000");
    eprintln!("Breakcore Visualizer running on http://0.0.0.0:3000");

    for request in server.incoming_requests() {
        let path = request.url().split('?').next().unwrap_or("/");
        match path {
            "/" => {
                let html = include_str!("../static/index.html");
                let header = Header::from_bytes("Content-Type", "text/html; charset=utf-8").unwrap();
                let _ = request.respond(Response::from_string(html).with_header(header));
            }
            "/health" => {
                let header = Header::from_bytes("Content-Type", "application/json").unwrap();
                let _ = request.respond(
                    Response::from_string(r#"{"status":"ok","service":"breakcore-viz"}"#)
                        .with_header(header),
                );
            }
            _ => {
                let _ = request.respond(Response::from_string("Not Found").with_status_code(404));
            }
        }
    }
}
