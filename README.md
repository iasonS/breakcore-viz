# BREAKCORE VISUALIZER
## Rust + Bevy Implementation

Real-time audio visualizer optimized for breakcore music (160-300 BPM, dense transients, chaotic layering).

### Architecture

- **Bevy ECS**: Entity-component-system framework for game/graphics applications
- **WGSL Shaders**: WebGPU shading language for raymarching, bloom, and feedback effects
- **Web Audio API**: Via wasm-bindgen for WASM target
- **WASM Deployment**: Compiles to WebAssembly for browser

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

### Building

```bash
cargo build --release
```

For WASM (browser):
```bash
cargo install wasm-pack
wasm-pack build --target web --release
```

### Structure

```
src/
├── main.rs          # Bevy app setup + main loop
├── audio.rs         # Audio analysis (FFT, onsets, centroid)
├── particles.rs     # Particle system + physics
├── renderer.rs      # Raymarching core + framebuffer management
└── effects.rs       # Glitch, bloom, feedback

assets/
├── shaders/
│   ├── core.wgsl    # Raymarched twisted form
│   ├── bloom.wgsl   # Gaussian blur + glow
│   └── feedback.wgsl # Recursive frame composition
└── config.toml      # Tunable parameters
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
