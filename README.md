# BREAKCORE VISUALIZER
## Rust + WASM Implementation

Real-time audio visualizer optimized for breakcore music (160-300 BPM, dense transients, chaotic layering).

### Architecture

- **HTTP Server** (Rust): Serves web UI and WASM artifacts (port 3000)
- **WASM Visualizer** (WebAssembly): Browser-based visualization with WebGL/Canvas
- **Web Audio API**: Real-time audio analysis from file upload or microphone
- **Particle System**: Physics-based particles with trails and color mapping
- **Audio Analysis**: Per-band energy tracking, onset detection, spectral centroid

### Features

✅ Per-band audio analysis (kick, mid, hat)
✅ Onset detection + chaos metric (onsets/second)
✅ Spectral centroid hue mapping
✅ Raymarched 3D form with energy-reactive morphing
✅ Feedback loops (ghosting/trails with zoom recursion)
✅ Multi-layer particle systems (kick burst, hat spray, peak white)
✅ Aggressive glitch engine (interlace + chromatic aberration)
✅ Bloom/glow post-processing
✅ Energy-scaled scanlines + vignette
✅ Violent camera shake on transients
✅ Data-driven config (TOML parameters)

### Quick Start

**Docker (recommended):**
```bash
docker-compose up
# Visit http://localhost:8080
```

**From source:**
```bash
# Build WASM visualizer
./build-wasm.sh

# Run HTTP server (serves web UI + WASM)
cargo build --release
./target/release/breakcore_viz
# Visit http://localhost:3000
```

### File Structure

```
breakcore-viz/
├── src/
│   ├── main.rs           # HTTP server + WASM entry point
│   ├── audio.rs          # Audio analysis logic
│   ├── particles.rs      # Particle system
│   ├── renderer.rs       # Render state management
│   ├── effects.rs        # Effect engines (glitch, bloom, feedback)
│   └── wasm_app.rs       # WASM module for browser
├── web/
│   ├── index.html        # Web UI with audio controls
│   └── pkg/              # Built WASM artifacts (generated)
├── assets/
│   ├── config.toml       # Tunable parameters
│   └── shaders/          # WGSL shaders (for future raymarching)
├── Dockerfile            # Multi-stage Docker build
├── docker-compose.yml    # Service orchestration
├── build-wasm.sh         # WASM build script
└── deploy.sh             # Server deployment helper
```

### Parameters

All visual/audio parameters are tunable via `assets/config.toml`:

- **Audio**: FFT band ranges, onset thresholds, energy smoothing
- **Renderer**: Raymarching steps, form scale, camera FOV
- **Particles**: Spawn counts, physics, colors
- **Effects**: Glitch intensity, bloom threshold, feedback decay
- **Camera**: Base position, energy-responsive zoom, shake amplitude
- **Colors**: HSL color space for spectral mapping

### Audio Analysis

- **Kick Band**: 20-100Hz, triggers red/magenta particles + screen shake
- **Mid Band**: 100-500Hz, modulates form complexity
- **Hat Band**: 500-2000Hz, triggers cyan particles + color shifts
- **Chaos Metric**: Onsets per second, drives glitch intensity + density
- **Spectral Centroid**: Frequency-weighted average, controls hue rotation

### Validation

Built with validation against actual breakcore videos:
- Venetian Snares (recursive, mathematical aesthetics)
- Sewerslvt (glitch, VHS corruption, neon on black)
- Machine Girl (chaotic, high-density transients)

### License

MIT
