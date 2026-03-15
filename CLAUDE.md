# breakcore-viz

Audio-reactive WebGL2 visualizer. Drop an audio file, fly through an infinite field of raymarched shapes.

## Architecture

Single-page app: `static/index.html` (all JS/GLSL inline). Served by a Rust binary (`src/main.rs`) using `tiny_http`.

### Rendering Pipeline
1. **Scene shader** (half-res) — raymarches domain-repeated SDFs: field objects (period ~6), debris spheres (period ~3.7), architectural crosses (period ~15), optional tunnel mode
2. **Post shader** (full-res) — chromatic aberration, bloom, feedback trail, CRT effects, color grading, vignette
3. **Canvas 2D overlay** — starfield streaks, spectrum rings, wireframe shapes, particles, shock rings, reality tears

### Audio → Visuals
Web Audio API FFT → 4 bands (sub/low/mid/high) → onset detection → drives: shape deformation, camera speed, fog density, particle spawns, color hue

### Camera
Forward-flying along Z. Speed = base + energy * boost. Sin/cos X/Y drift. Mouse offsets look direction. Kick impulses add forward bursts.

### Evolution System
6 phases cycle over time (beat-triggered advance). Each phase targets: repetition period, fog density, camera speed multiplier, tunnel blend, object scale, color grade. Smooth lerp between targets.

## Key Files

- `static/index.html` — everything (shaders, audio, overlay, render loop)
- `src/main.rs` — Rust HTTP server (tiny_http, serves index.html + /health)
- `docker-compose.yml` — container config, port 8086 → 3000
- `deploy.sh` — deploys to tt@192.168.1.9:/home/tt/breakcore-viz

## Deploy

```bash
bash deploy.sh
# → http://192.168.1.9:8086
```

## Dev

Open `static/index.html` directly in browser for local testing. Add `?debug` for HUD overlay.
