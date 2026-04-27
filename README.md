# Breakcore Visualizer

Audio-reactive WebGL2 visualizer for breakcore music. Drop an audio file, fly through an infinite tunnel of raymarched shapes toward a massive black hole. **Now with Philips Hue lights sync!**

## Features

✅ **Real-time audio analysis** — FFT bands (sub/low/mid/high), onset detection, spectral centroid  
✅ **WebGL2 rendering** — Raymarched tunnel, generative parametric SDF, spawn pool with 7 object types  
✅ **ULTRAKILL-inspired impact system** — Hitlag, impulse shake, FOV spring, pixelation on hard kicks  
✅ **Style meter** — 6 ranks (STATIC → RISING → VOLATILE → UNHINGED → BREAKCORE → ULTRAKILL)  
✅ **Canvas 2D overlay** — Starfield, spectrum ring, waveform scope, particles, geometric mandalas  
✅ **Offline analysis mode** — Pre-compute energy/frequencies for entire song on load  
✅ **Song section detection** — Breakdown/buildup/drop/sustain classification  
✅ **Philips Hue integration** — Lights respond to analyzed audio in real-time  
✅ **Voice synthesis** — Narration with sam-js  

## Quick Start

### Docker (recommended)

```bash
docker-compose up
# Visit http://localhost:8086
```

### Local development

```bash
# Requires Rust (build Rust server)
cargo build --release
./target/release/breakcore-viz
# Visit http://localhost:3000
```

Open `static/index.html` directly in browser for development (no server needed for testing).

## Deploy

```bash
bash deploy.sh
# Deploys to tt@192.168.1.9:/home/tt/breakcore-viz
# Visit http://192.168.1.9:8086
```

## Hue Lights Setup

Set env vars in `.env` on deployment server:

```env
HUE_BRIDGE_IP=192.168.1.10
HUE_API_KEY=<your-api-key>
HUE_LIGHT_BIG=1          # Room light (overall energy + section mood)
HUE_LIGHT_PC=2           # Left desk (bass-responsive, kick pulses)
HUE_LIGHT_SCREEN=3       # Screen light (treble-responsive, cool colors)
```

The hue-service sidecar (Node.js) syncs lights to the visualizer's offline-analyzed audio data, not real-time. Lights react ahead of beats with:
- **Low energy**: Subtle fearful trembling, muted colors, low brightness
- **High energy**: Dramatic color shifts, saturated hues, intense brightness

## Architecture

```
[Browser: WebGL2 + Canvas + Web Audio API]
     ↓ (offline pre-analyzed audio data)
[Hue Service: Node.js, maps energy → light state]
     ↓ (HTTP API calls)
[Hue Bridge] → [Hue Lights]
```

### Frontend (static/index.html)

- **Size**: ~3,600 lines (JS + GLSL inline)
- **Audio**: Web Audio API FFT (2048 bins), 4-band filtering, onset detection
- **Rendering**: WebGL2 scene shader (half-res), post-processing (full-res), Canvas 2D overlay
- **Offline Analysis**: Full FFT pass over entire audio file on load, pre-computes energy for all frames
- **Hue Sync**: Sends offline data to hue-service every 150ms (~6.7fps)

### Backend (src/main.rs)

- **Server**: Rust `tiny_http`, serves index.html + `/health` endpoint
- **Port**: 3000 (Docker: 8086 external)
- **Endpoints**:
  - `GET /` — serves index.html
  - `GET /health` — `{"status":"ok"}`

### Hue Service (hue-service/server.js)

- **Port**: 3001 (Docker: 8087 external)
- **Endpoints**:
  - `POST /sync` — receives `{sub, low, mid, high, energy, kick, section}`, controls lights
  - `GET /health` — `{"status":"ok"}`
- **Dependencies**: Node.js stdlib only (no npm)
- **Rate limiting**: 10 commands/sec to Hue Bridge

## Audio → Visuals

**Frequency bands** (Hz):
- Sub: 20–100 (kicks)
- Low: 100–500 (bass)
- Mid: 500–2000 (snare, vocals)
- High: 2000–16000 (hats, cymbal wash)

**Energy calculation** (weighted):
```
energy = min(1, (sub*2 + low*1.2 + mid*0.5 + high*0.3) / 2)
```
Sub-bass dominates; melodic wash is suppressed.

**Impact system** (on hard kicks):
- Hitlag: 2–6 frame freeze
- Scene pixelation: quarter-res render
- FOV spring: snap wide, exponential settle
- Flash: white-out with red bleed
- Chromatic aberration boost

## Song Sections

Detected via rolling 5-second energy buffer:

| Section | Energy | Trend | Camera | Fog | Spawns |
|---------|--------|-------|--------|-----|--------|
| Breakdown | Low | Flat | Slow | Clear | Monoliths |
| Buildup | Low→High | Rising | Gradual accel | Closes | Generative clusters |
| Drop | High | Spike | Blast forward | Dense | Ring gates |
| Sustain | Normal | Stable | Reactive | Normal | Varied |

## Development

**Query params**:
- `?debug` — Show HUD (energy, chaos, band levels, style rank)
- `?demo` — Synthetic audio mode (no file upload needed)

**Browser console logs**:
- `[HUE]` — Shows energy/kick/section every sync
- Hue service logs show exact light values being sent

## Testing with Breakcore

Recommended tracks:
- Venetian Snares — mathematical recursion, tight precision
- Sewerslvt — glitch, VHS corruption, neon-on-black aesthetic
- Machine Girl — chaotic, dense transients, harsh textures

## File Structure

```
breakcore-viz/
├── src/
│   └── main.rs              # Rust HTTP server
├── static/
│   └── index.html           # WebGL2 visualizer + shaders
├── hue-service/
│   ├── server.js            # Hue lights sync service
│   └── Dockerfile
├── docker-compose.yml       # Orchestrates Rust + Node services
├── Dockerfile               # Rust server image
├── deploy.sh                # SSH deploy helper
├── CLAUDE.md                # AI codebase documentation
└── README.md                # This file
```

## License

MIT
