# breakcore-viz

Audio-reactive WebGL2 visualizer. Drop an audio file, fly through an infinite field of raymarched shapes toward a massive black hole.

## Architecture

Single-page app: `static/index.html` (~1,877 lines, all JS/GLSL inline). Served by a Rust binary (`src/main.rs`) using `tiny_http`.

### Rendering Pipeline
1. **Scene shader** (half-res) — raymarches 6 SDF layers: tunnel wall objects (morphing shapes), wall debris, corridor dust (hyperspace sparks), energy rings (mid-distance torus), megastructures (colossal background geometry), grand attractor (multi-ring black hole with gravitational lensing)
2. **Post shader** (full-res) — chromatic aberration, bloom, feedback trail with zoom punch, gravity wave distortion, radial blur on impact, flash-frame color inversion, CRT effects, color grading, vignette
3. **Canvas 2D overlay** — starfield with gravitational lensing, hyperspace tunnel lines, spectrum ring (breathing, inward+outward spikes), time-domain waveform scope, radial frequency landscape, wireframe shapes, bass arcs, energy tendrils, particles, shock rings, reality tears (edge + center), geometric mandalas

### Audio → Visuals
Web Audio API FFT → 4 bands (sub/low/mid/high) → onset detection → drives everything. Song section detection (breakdown/buildup/drop/sustain) via rolling energy history buffer.

### Song Section Detection
Rolling 5-second energy buffer tracks average, trend, and ratio. Classifies into:
- **breakdown**: low energy → slow camera, clear fog, vast space
- **buildup**: rising trend → gradual acceleration, fog closes in
- **drop**: energy spike → blast forward, triple kick impulse, phase change
- **sustain**: normal reactive mode

### Camera
Forward-flying along Z. Speed modulated by energy, evolution phase, and song section. Sin/cos XY drift + mouse offset. Kick impulses add forward bursts. Camera shake from kick/snare/chaos.

### Impact System
Hard kicks (>0.6 with delta >0.02) trigger `impactIntensity` with fast decay (0.85/frame). Drives: gravity wave UV distortion, radial zoom blur, feedback zoom punch, chromatic aberration boost, flash-frame color inversion (>0.7 threshold).

### Black Hole
Fixed world Z position, camera approaches and passes through. On pass-through: flash, respawn 50-90 units ahead, force evolution phase change. Accretion disk: tilted, 5 ring layers (inner hot, main, mid, outer, halo). Gravitational lensing warps both ray direction (GLSL) and 2D starfield positions (Canvas).

### Evolution System
6 phases with target params (rep period, fog, speed, tunnel blend, obj scale, color grade). Song-section aware: drops force phase changes, breakdowns freeze them, buildups slow progression.

### SDF Shapes
4 morphing shapes (torus, crystal, jellyfish, blackhole) with smooth-step transitions. Megastructures: 4 types (monolith, ring gate, cross, arch) at 40-unit periods.

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

Open `static/index.html` directly in browser for local testing. Add `?debug` for HUD overlay. Add `?demo` for synthetic audio mode (no file needed).
# test
