# breakcore-viz

Audio-reactive WebGL2 visualizer with breakcore edit aesthetic. Monochrome (blacks/whites/silvers), high contrast, angular crystalline geometry. Drop an audio file, fly through an infinite field of raymarched shapes toward a massive black hole.

## Architecture

Single-page app: `static/index.html` (~2,750 lines, all JS/GLSL inline). Served by a Rust binary (`src/main.rs`) using `tiny_http`.

### Rendering Pipeline
1. **Scene shader** (half-res, 40 steps) — raymarches 2 always-on SDFs (tunnel walls, grand attractor) + 8-slot spawn pool (7 spawn types: generative cluster, crystal burst, ring gate, void sphere, monolith, debris shrapnel, energy bloom). Inactive slots early-out.
2. **Post shader** (full-res) — chromatic aberration, bloom, feedback trail with zoom punch, gravity wave distortion, radial blur on impact, BH pass-through white flash (uFlash), flash-frame color inversion, CRT effects, S-curve contrast grading (audio-driven), desaturation, vignette
3. **Canvas 2D overlay** — monochrome starfield with gravitational lensing, angular spectrum spikes (40 filled triangles), star polygon wireframes, bass arcs, angular debris particles (triangles), shock rings, reality tears, radiating line burst on peaks

### Audio → Visuals
Web Audio API FFT → 4 bands (sub/low/mid/high) → onset detection → drives everything. Song section detection (breakdown/buildup/drop/sustain) via rolling energy history buffer.

### Song Section Detection
Rolling 5-second energy buffer tracks average, trend, and ratio. Classifies into:
- **breakdown**: low energy → slow camera, clear fog, vast space
- **buildup**: rising trend → gradual acceleration, fog closes in
- **drop**: energy spike → blast forward, triple kick impulse, phase change
- **sustain**: normal reactive mode

### Camera (v9 — Motion Fix)
Forward-flying along Z. Speed modulated by energy, evolution phase, and song section. Sin/cos XY drift (max ±1.5, energy-gated) + mouse offset (0.8x/0.6y, reduced from 2.0/1.5). Kick impulses add forward bursts. GLSL shake: low-frequency (3.5-8Hz), energy-gated (quiet=still), halved amplitudes. LookAt wobble reduced to ±0.12/±0.08. Max total jitter ~±1.8 units (was ±4.38).

### Impact System
Hard kicks (>0.6 with delta >0.02) trigger `impactIntensity` with fast decay (0.85/frame). Drives: gravity wave UV distortion, radial zoom blur, feedback zoom punch, chromatic aberration boost, flash-frame color inversion (>0.7 threshold).

### Black Hole (v9 — Flash + Rebuild)
Fixed world Z position, camera approaches and passes through. On pass-through: white flash (uFlash uniform, two-phase decay: 0.96 above 0.5, 0.92 below, ~1.3s total), respawn 50-90 units ahead, queue timed rebuild sequence (void spheres → debris → energy bloom → generative cluster → monolith → ring gate over 2.5s). Accretion disk: tilted, 5 ring layers (inner hot, main, mid, outer, halo). Gravitational lensing warps both ray direction (GLSL) and 2D starfield positions (Canvas).

### Evolution System (v8 — Audio-Driven)
No more fixed phases or deterministic cycling. All evolution parameters (rep period, fog density, speed, tunnel blend, obj scale, corridor radius, contrast) are derived continuously from audio features (energy, chaos, kick, spectral centroid). Section overrides (breakdown/buildup/drop) apply on top. Smooth lerp prevents jarring transitions.

### Anti-Staleness System (v7)
Prevents visual repetitiveness over long playback:
- **Audio novelty curve**: Offline analysis computes per-frame novelty (cosine similarity of 16-bin spectral fingerprints against 4s window) and repetition (beat-period fingerprint comparison). Real-time fallback from spectral flux.
- **Progressive reveal**: 4-tier system (0s/30s/60s/120s) gates available shapes, camera drift patterns, and color grades. Shapes are randomly selected (not linear cycling).
- **Visual novelty injection**: When audio is repetitive (low novelty >3s or high repetition >2s), `injectionLevel` ramps up to speed shape/phase cycling, shift camera drift frequencies, rotate fog hue, and boost camera roll.
- **Autonomous drift**: During repetitive sections, slow sinusoidal modulations on fog density, corridor radius, object scale, and hue drift operate independently of audio.
- **Camera drift**: Now derived continuously from spectral centroid and energy (no preset patterns). Amplitude scales with energy.

### SDF Shapes (v9 — Spawn Pool)
Always-on: tunnel walls (parametric generative SDF) + grand attractor. 8-slot spawn pool with 7 types: (1) generative cluster, (2) crystal burst, (3) ring gate, (4) void sphere, (5) monolith, (6) debris shrapnel, (7) energy bloom. Audio events spawn objects with weighted random type selection and rate limiting. Life-based scale envelope (smoothstep grow-in 0-15%, shrink-out 85-100%). Objects killed when expired or camera passes by >10 units. Removed: debrisSDF, dustSDF, energyRingSDF, megastructureSDF (replaced by spawn types).

### Monochrome Aesthetic (v8)
Near-monochrome palette: blacks, whites, silvers, cool greys. No HSL color — all surfaces use luminance-based coloring. Post shader crushes contrast with S-curve (`smoothstep(0.05, 0.95, col)`), low saturation (0.3), and audio-driven contrast grading. Overlay elements all use white/silver/grey with varying alpha.

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
