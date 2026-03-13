mod audio;
mod particles;
mod renderer;
mod effects;

#[cfg(target_arch = "wasm32")]
pub mod wasm_app;

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    // HTTP server for serving the web UI and WASM visualizer
    use std::fs;
    use std::path::Path;

    let addr = "0.0.0.0:3000";
    let server = tiny_http::Server::http(addr).expect("Failed to start server");

    println!("🎵 Breakcore Visualizer Server");
    println!("Listening on http://{}:3000", "localhost");
    println!("");
    println!("Access points:");
    println!("  Web UI: http://localhost:3000");
    println!("  API: http://localhost:3000/api/audio");
    println!("");

    for request in server.incoming_requests() {
        let method = request.method();
        let url = request.url();

        // Log request
        eprintln!("{} {}", method, url);

        let response = if url == "/" {
            // Serve index.html
            match fs::read_to_string("web/index.html") {
                Ok(content) => {
                    tiny_http::Response::from_string(content)
                        .with_header(tiny_http::Header::from_bytes(&b"Content-Type"[..], &b"text/html"[..]).unwrap())
                }
                Err(_) => tiny_http::Response::from_string("<h1>404 Not Found</h1>".to_string())
                    .with_status_code(404),
            }
        } else if url.starts_with("/pkg/") {
            // Serve WASM artifacts
            let file_path = format!("web{}", url);
            if Path::new(&file_path).exists() {
                match fs::read(&file_path) {
                    Ok(content) => {
                        let content_type = if url.ends_with(".js") {
                            "application/javascript"
                        } else if url.ends_with(".wasm") {
                            "application/wasm"
                        } else {
                            "application/octet-stream"
                        };

                        tiny_http::Response::from_data(content)
                            .with_header(
                                tiny_http::Header::from_bytes(
                                    &b"Content-Type"[..],
                                    content_type.as_bytes(),
                                )
                                .unwrap(),
                            )
                    }
                    Err(_) => tiny_http::Response::from_string("<h1>404 Not Found</h1>".to_string())
                        .with_status_code(404),
                }
            } else {
                tiny_http::Response::from_string("<h1>404 Not Found</h1>".to_string())
                    .with_status_code(404)
            }
        } else if url == "/api/audio" && *method == tiny_http::Method::Post {
            // Audio analysis endpoint
            tiny_http::Response::from_string(r#"{"status":"ok"}"#.to_string())
                .with_header(tiny_http::Header::from_bytes(&b"Content-Type"[..], &b"application/json"[..]).unwrap())
        } else if url == "/health" {
            // Health check
            tiny_http::Response::from_string(r#"{"status":"ok","service":"breakcore-viz"}"#.to_string())
                .with_header(tiny_http::Header::from_bytes(&b"Content-Type"[..], &b"application/json"[..]).unwrap())
        } else {
            tiny_http::Response::from_string("<h1>404 Not Found</h1>".to_string())
                .with_status_code(404)
        };

        let _ = request.respond(response);
    }
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn run_visualizer() {
    // WASM entry point for browser visualization
    web_sys::console::log_1(&"Breakcore Visualizer loaded".into());
}
