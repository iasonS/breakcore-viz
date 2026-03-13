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
    let html = r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>BREAKCORE VISUALIZER</title>
    <style>
        * { margin: 0; padding: 0; box-sizing: border-box; }
        html, body { width: 100%; height: 100%; background: #000; font-family: 'Courier New', monospace; overflow: hidden; }
        canvas { display: block; width: 100%; height: 100%; }
        #controls { position: fixed; bottom: 20px; left: 20px; z-index: 10; display: flex; gap: 10px; }
        #info { position: fixed; top: 10px; left: 10px; color: #0f0; font-size: 12px; background: rgba(0, 0, 0, 0.8); padding: 10px; border: 1px solid #0f0; max-width: 350px; }
        input[type="file"], button { background: rgba(0, 255, 0, 0.2); border: 1px solid #0f0; color: #0f0; padding: 8px 12px; cursor: pointer; font-family: 'Courier New', monospace; font-size: 12px; }
        button:hover { background: rgba(0, 255, 0, 0.4); }
        #status { position: fixed; bottom: 20px; right: 20px; color: #f0f; font-size: 12px; background: rgba(0, 0, 0, 0.8); padding: 8px; border: 1px solid #f0f; }
    </style>
</head>
<body>
    <canvas id="canvas"></canvas>
    <div id="info">
        <div><strong>BREAKCORE VISUALIZER</strong></div>
        <div id="fps">FPS: --</div>
        <div id="energy">Energy: 0.00</div>
        <div id="chaos">Chaos: 0.00</div>
    </div>
    <div id="controls">
        <input type="file" id="audio-input" accept="audio/*">
        <button id="play-btn">Play</button>
    </div>
    <div id="status">Ready</div>
    <script>
        console.log("Breakcore Visualizer loaded on server");
        document.getElementById('play-btn').onclick = () => alert('WASM visualization coming soon!');
    </script>
</body>
</html>"#;

    tiny_http::Response::from_string(html.to_string()).with_header(
        tiny_http::Header::from_bytes(&b"Content-Type"[..], &b"text/html"[..])
            .unwrap(),
    )
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
