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

        let response = match url {
            "/" => serve_index(),
            "/health" => {
                tiny_http::Response::from_string(r#"{"status":"ok","service":"breakcore-viz"}"#.to_string())
                    .with_header(
                        tiny_http::Header::from_bytes(&b"Content-Type"[..], &b"application/json"[..])
                            .unwrap(),
                    )
            },
            u if u.starts_with("/pkg/") => serve_wasm_artifact(u),
            _ => {
                tiny_http::Response::from_string("<h1>404 Not Found</h1>".to_string())
                    .with_status_code(404)
            }
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
        canvas { display: block; width: 100%; height: 100%; background: #000; }
        #controls { position: fixed; bottom: 20px; left: 20px; z-index: 10; display: flex; gap: 10px; }
        #info { position: fixed; top: 10px; left: 10px; color: #0f0; font-size: 12px; background: rgba(0, 0, 0, 0.8); padding: 10px; border: 1px solid #0f0; max-width: 350px; }
        input[type="file"], button { background: rgba(0, 255, 0, 0.2); border: 1px solid #0f0; color: #0f0; padding: 8px 12px; cursor: pointer; font-family: 'Courier New', monospace; font-size: 12px; }
        button:hover { background: rgba(0, 255, 0, 0.4); }
        #status { position: fixed; bottom: 20px; right: 20px; color: #f0f; font-size: 12px; background: rgba(0, 0, 0, 0.8); padding: 8px; border: 1px solid #f0f; }
    </style>
</head>
<body>
    <canvas id="canvas"></canvas>
    <div id="info"><div><strong>🎵 BREAKCORE VISUALIZER</strong></div><div id="fps">FPS: --</div><div id="energy">Energy: 0.00</div><div id="chaos">Chaos: 0.00</div></div>
    <div id="controls"><input type="file" id="audio-input" accept="audio/*"><button id="play-btn">▶ Play</button></div>
    <div id="status">Ready - Upload audio file</div>
    <script>
        const canvas = document.getElementById('canvas');
        const ctx = canvas.getContext('2d');
        const audioInput = document.getElementById('audio-input');
        const playBtn = document.getElementById('play-btn');

        canvas.width = window.innerWidth;
        canvas.height = window.innerHeight;
        window.addEventListener('resize', () => {
            canvas.width = window.innerWidth;
            canvas.height = window.innerHeight;
        });

        let audioCtx = null;
        let analyser = null;
        let sourceNode = null;
        let audioBuffer = null;
        let isPlaying = false;
        let startTime = 0;

        // Particle system
        const particles = [];
        let frameCount = 0;
        let lastEnergy = 0;

        class Particle {
            constructor(x, y, vx, vy, color, life = 1) {
                this.x = x;
                this.y = y;
                this.vx = vx;
                this.vy = vy;
                this.color = color;
                this.life = life;
                this.trail = [];
            }
            update() {
                this.trail.push({x: this.x, y: this.y});
                if (this.trail.length > 20) this.trail.shift();
                this.x += this.vx;
                this.y += this.vy;
                this.vy += 0.1; // gravity
                this.vx *= 0.98;
                this.life -= 0.01;
            }
            draw(ctx) {
                ctx.globalAlpha = this.life;
                ctx.strokeStyle = this.color;
                ctx.lineWidth = 2;
                ctx.beginPath();
                this.trail.forEach((p, i) => {
                    if (i === 0) ctx.moveTo(p.x, p.y);
                    else ctx.lineTo(p.x, p.y);
                });
                ctx.stroke();
                ctx.fillStyle = this.color;
                ctx.fillRect(this.x - 2, this.y - 2, 4, 4);
                ctx.globalAlpha = 1;
            }
        }

        audioInput.addEventListener('change', (e) => {
            const file = e.target.files[0];
            if (!file) return;

            if (!audioCtx) audioCtx = new (window.AudioContext || window.webkitAudioContext)();

            const reader = new FileReader();
            reader.onload = (event) => {
                audioCtx.decodeAudioData(event.target.result, (buffer) => {
                    audioBuffer = buffer;
                    playBtn.textContent = '▶ Play';
                    document.getElementById('status').textContent = 'Ready to play';
                });
            };
            reader.readAsArrayBuffer(file);
        });

        playBtn.addEventListener('click', () => {
            if (!audioBuffer) return;

            if (audioCtx.state === 'suspended') audioCtx.resume();

            if (isPlaying) {
                sourceNode.stop();
                isPlaying = false;
                playBtn.textContent = '▶ Play';
                document.getElementById('status').textContent = 'Stopped';
            } else {
                sourceNode = audioCtx.createBufferSource();
                sourceNode.buffer = audioBuffer;
                analyser = audioCtx.createAnalyser();
                analyser.fftSize = 2048;
                sourceNode.connect(analyser);
                analyser.connect(audioCtx.destination);
                sourceNode.start(0);
                startTime = performance.now();
                isPlaying = true;
                playBtn.textContent = '⏹ Stop';
                document.getElementById('status').textContent = 'Playing...';
            }
        });

        function hslToRgb(h, s, l) {
            h = h % 360;
            s = Math.max(0, Math.min(100, s)) / 100;
            l = Math.max(0, Math.min(100, l)) / 100;
            const c = (1 - Math.abs(2 * l - 1)) * s;
            const x = c * (1 - Math.abs((h / 60) % 2 - 1));
            const m = l - c / 2;
            let r = 0, g = 0, b = 0;
            if (h < 60) [r, g, b] = [c, x, 0];
            else if (h < 120) [r, g, b] = [x, c, 0];
            else if (h < 180) [r, g, b] = [0, c, x];
            else if (h < 240) [r, g, b] = [0, x, c];
            else if (h < 300) [r, g, b] = [x, 0, c];
            else [r, g, b] = [c, 0, x];
            return `rgb(${Math.round((r+m)*255)}, ${Math.round((g+m)*255)}, ${Math.round((b+m)*255)})`;
        }

        function animate() {
            frameCount++;

            // Audio analysis
            if (analyser && isPlaying) {
                const dataArray = new Uint8Array(analyser.frequencyBinCount);
                analyser.getByteFrequencyData(dataArray);

                let energy = 0;
                for (let i = 0; i < dataArray.length; i++) {
                    energy += dataArray[i];
                }
                energy /= dataArray.length;
                energy /= 255;

                const energyDelta = energy - lastEnergy;
                let chaos = 0;
                for (let i = 1; i < dataArray.length; i++) {
                    if (dataArray[i] - dataArray[i-1] > 30) chaos++;
                }
                chaos /= 10;

                document.getElementById('energy').textContent = `Energy: ${energy.toFixed(2)}`;
                document.getElementById('chaos').textContent = `Chaos: ${chaos.toFixed(2)}`;

                // Spawn particles on transients
                if (energyDelta > 0.1) {
                    const count = Math.floor(energy * 100);
                    const hue = (energy * 360) % 360;
                    for (let i = 0; i < count; i++) {
                        const angle = Math.random() * Math.PI * 2;
                        const speed = 5 + Math.random() * 10 + energy * 5;
                        particles.push(new Particle(
                            canvas.width / 2,
                            canvas.height / 2,
                            Math.cos(angle) * speed,
                            Math.sin(angle) * speed,
                            hslToRgb(hue, 100, 50)
                        ));
                    }
                }

                lastEnergy = energy;
            }

            // Draw
            ctx.fillStyle = 'rgba(0, 0, 0, 0.1)';
            ctx.fillRect(0, 0, canvas.width, canvas.height);

            // Update and draw particles
            for (let i = particles.length - 1; i >= 0; i--) {
                particles[i].update();
                if (particles[i].life <= 0) particles.splice(i, 1);
                else particles[i].draw(ctx);
            }

            // Draw waveform
            if (analyser) {
                const dataArray = new Uint8Array(analyser.frequencyBinCount);
                analyser.getByteFrequencyData(dataArray);

                ctx.strokeStyle = '#0f0';
                ctx.lineWidth = 1;
                ctx.beginPath();
                const sliceWidth = canvas.width / dataArray.length;
                let x = 0;
                for (let i = 0; i < dataArray.length; i++) {
                    const v = dataArray[i] / 255;
                    const y = canvas.height / 2 - (v * canvas.height / 2);
                    if (i === 0) ctx.moveTo(x, y);
                    else ctx.lineTo(x, y);
                    x += sliceWidth;
                }
                ctx.stroke();
            }

            // FPS counter
            if (frameCount % 10 === 0) {
                document.getElementById('fps').textContent = `FPS: ${Math.round(1000/16).toString()}`;
            }

            requestAnimationFrame(animate);
        }

        animate();
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
