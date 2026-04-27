# breakcore-viz

Audio-reactive WebGL2 visualizer. Drop an audio file, fly through an infinite field of raymarched shapes toward a massive black hole. Monochrome aesthetic with earned red accent on hardest impacts.

## Architecture

Single-page app: `static/index.html` (~2,490 lines, all JS/GLSL inline). Served by a Rust binary (`src/main.rs`) using `tiny_http`.

### Rendering Pipeline
1. **Scene shader** (variable-res: half normally, quarter during hitlag) — raymarches tunnel walls (generative parametric SDF), grand attractor (black hole), and 8-slot spawn pool (lifecycle-managed objects)
2. **Post shader** (full-res) — chromatic aberration (asymmetric R push on impact), bloom, feedback trail with zoom punch, gravity wave distortion, radial blur on impact, S-curve contrast grading, vignette, BH flash with red bleed
3. **Canvas 2D overlay** — starfield with gravitational lensing, hyperspace tunnel lines, spectrum ring, waveform scope, radial frequency landscape, wireframe shapes, bass arcs, energy tendrils, particles (5% red at peak energy), shock rings, reality tears, geometric mandalas, style meter

### Audio -> Visuals
Web Audio API FFT -> 4 bands (sub/low/mid/high) -> percussion gating -> onset detection -> drives everything. Song section detection (breakdown/buildup/drop/sustain) via rolling energy history buffer. Offline analysis mode for pre-computed section data.

### ULTRAKILL Impact System
- **Hitlag**: Hard kicks (>0.7) freeze scene for 2-3 frames; BH pass freezes for 6 frames
- **Impulse shake**: Instant random displacement + 0.82 exponential decay (replaces old sine shake)
- **FOV spring**: Underdamped spring snaps wide on kicks, settles back to 1.5 base
- **Pixelation**: Scene renders at quarter-res during hitlag for PS1 crunch
- **Red accent**: 99% monochrome; red earned only on hardest hits (smoothstep 0.85-1.0)

### Spawn Pool System
8-slot uniform-driven pool (`uSpawnA[8]`, `uSpawnB[8]`). Objects spawn based on audio events with weighted random selection, making each playthrough unique.

7 spawn types:
| Type | Name | Lifetime | Trigger |
|------|------|----------|---------|
| 1 | Generative cluster | 4-8s | Hard kick |
| 2 | Crystal burst | 2-4s | Hard kick / snare |
| 3 | Ring gate | 8-15s | Drop transition |
| 4 | Void sphere | 3-5s | BH pass rebuild |
| 5 | Monolith | 10-20s | Breakdown |
| 6 | Debris shrapnel | 1-3s | Snare onset |
| 7 | Energy bloom | 3-6s | Hat cluster |

### Generative Parametric SDF
N-fold symmetry with audio-driven parameters: symmetry (3-8), spike amplitude, fracture, twist, hollow, angular blend. Replaces old 4 morphing shapes.

### Song Section Detection
Rolling 5-second energy buffer tracks average, trend, and ratio. Classifies into:
- **breakdown**: low energy -> slow camera, clear fog, vast space, monolith spawns
- **buildup**: rising trend -> gradual acceleration, fog closes in
- **drop**: energy spike -> blast forward, ring gate spawns, force evolution change
- **sustain**: normal reactive mode

### Camera
Forward-flying along Z. Speed modulated by energy, evolution, and song section. Reduced drift (0.5 + energy * 1.0) + mouse offset (0.8x/0.6x). Kick impulses add forward bursts. JS impulse shake (no GLSL shake).

### Black Hole
Fixed world Z, camera approaches and passes through. On pass-through: 6-frame hitlag -> white-out flash with red bleed -> timed rebuild sequence (void spheres -> debris -> bloom -> geometry -> architecture over 2.5s). Respawn 50-90 units ahead. Accretion disk with gravitational lensing.

### Evolution System
Audio-driven continuous parameter mapping (fog density, rep period, speed, corridor radius, object scale). No discrete phases — parameters smoothly track energy levels.

### Style Meter
6 ranks: STATIC -> RISING -> VOLATILE -> UNHINGED -> BREAKCORE -> ULTRAKILL. Tracks smoothed energy with 2.5s grace period before downranking. Canvas 2D overlay, bottom-center, shakes on kicks.

## Philips Hue Lights Integration

Sidecar Node.js service (`hue-service/`) syncs lights to analyzed audio in real-time.

### Architecture
- Frontend sends offline pre-analyzed audio data to hue-service every 150ms (~6.7fps)
- hue-service maps energy → brightness, frequencies → colors per light
- Lights respond to *future* audio (offline analysis) not real-time, so they react ahead of beats
- Rate-limited to 10 requests/sec to Hue Bridge

### Light Mapping
| Light | ID | Position | Responds To | Low Energy Behavior | High Energy Behavior |
|-------|----|---------|----|-----|-----|
| Big | 1 | Room | Overall energy + section mood | Fearful muted trembling | Dramatic hue shifts + saturation |
| PC Left | 2 | Desk | Bass (sub+low) + kicks | Subtle bass rumble | Red-orange on impacts |
| Screen | 3 | Display | Full spectrum (sub+low+mid+high) | Cool fearful glow | Magenta saturation + brightness |

### Config (in `.env` on server)
```env
HUE_BRIDGE_IP=192.168.1.10
HUE_API_KEY=<username-from-bridge>
HUE_LIGHT_BIG=1
HUE_LIGHT_PC=2
HUE_LIGHT_SCREEN=3
```

### Energy States
- **Low energy (< 0.25)**: 6Hz subtle trembling, desaturated colors, 30–80 brightness range
- **High energy (≥ 0.25)**: Dramatic changes, saturated colors, 80–254 brightness range
- **Idle (energy < 0.05)**: No updates sent (lights stay still)

## Key Files

- `static/index.html` — everything (shaders, audio, overlay, render loop, Hue sync)
- `src/main.rs` — Rust HTTP server (tiny_http, serves index.html + /health)
- `hue-service/server.js` — Node.js Hue lights sync service
- `hue-service/Dockerfile` — lightweight Alpine Node image
- `docker-compose.yml` — orchestrates both Rust and Node services
- `deploy.sh` — deploys to tt@192.168.1.9:/home/tt/breakcore-viz

## Deploy

```bash
bash deploy.sh
# -> http://192.168.1.9:8086 (visualizer)
# -> http://192.168.1.9:8087/sync (Hue service, internal)
```

Ensure `.env` has `HUE_BRIDGE_IP`, `HUE_API_KEY`, and light IDs before deploying.

## Dev

**Browser testing**: Open `static/index.html` directly. Add `?debug` for HUD. Add `?demo` for synthetic audio.

**Console logs**: Look for `[HUE]` messages showing energy/kick/section when music plays.

**Hue service logs**: `docker logs breakcore-viz-hue-service-1` shows exact light state updates.
