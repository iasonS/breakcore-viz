use std::fs;
use std::path::Path;

fn main() {
    let addr = "0.0.0.0:3000";
    let server = tiny_http::Server::http(addr).expect("Failed to start server");

    println!("🎵 Breakcore Visualizer Server");
    println!("Listening on http://0.0.0.0:3000");
    println!("Access: http://localhost:3000");
    println!("");

    for request in server.incoming_requests() {
        let method = request.method();
        let url = request.url();

        eprintln!("{} {}", method, url);

        let response = if url == "/" {
            serve_index()
        } else if url.starts_with("/pkg/") {
            serve_wasm_artifact(url)
        } else if url == "/health" {
            tiny_http::Response::from_string(r#"{"status":"ok","service":"breakcore-viz"}"#.to_string())
                .with_header(
                    tiny_http::Header::from_bytes(&b"Content-Type"[..], &b"application/json"[..])
                        .unwrap(),
                )
        } else {
            tiny_http::Response::from_string("<h1>404 Not Found</h1>".to_string())
                .with_status_code(404)
        };

        let _ = request.respond(response);
    }
}

fn serve_index() -> tiny_http::Response<std::io::Cursor<Vec<u8>>> {
    let paths = &["web/index.html", "index.html", "/app/web/index.html"];

    for path in paths {
        if let Ok(content) = fs::read_to_string(path) {
            return tiny_http::Response::from_string(content).with_header(
                tiny_http::Header::from_bytes(&b"Content-Type"[..], &b"text/html"[..])
                    .unwrap(),
            );
        }
    }

    tiny_http::Response::from_string("<h1>404 Not Found</h1>".to_string())
        .with_status_code(404)
}

fn serve_wasm_artifact(url: &str) -> tiny_http::Response<std::io::Cursor<Vec<u8>>> {
    let base_paths = &["web", "", "/app/web", "/app"];
    let file_name = &url[5..]; // Remove "/pkg/" prefix

    for base in base_paths {
        let file_path = if base.is_empty() {
            format!("pkg/{}", file_name)
        } else {
            format!("{}/pkg/{}", base, file_name)
        };

        if let Ok(content) = fs::read(&file_path) {
            let content_type = if url.ends_with(".js") {
                "application/javascript"
            } else if url.ends_with(".wasm") {
                "application/wasm"
            } else {
                "application/octet-stream"
            };

            return tiny_http::Response::from_data(content).with_header(
                tiny_http::Header::from_bytes(&b"Content-Type"[..], content_type.as_bytes())
                    .unwrap(),
            );
        }
    }

    tiny_http::Response::from_string("<h1>404 Not Found</h1>".to_string())
        .with_status_code(404)
}
