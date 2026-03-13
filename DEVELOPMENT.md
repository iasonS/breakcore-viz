# Development Guide

## Quick Start

```bash
cd breakcore-viz
cargo build --release
```

## Development Workflow

### 1. Running Locally
```bash
cargo run
```

### 2. Building for Browser (WASM)
```bash
chmod +x build-wasm.sh
./build-wasm.sh
cd web
python3 -m http.server 8000
# Visit http://localhost:8000
```

### 3. Building Docker Image
```bash
docker build -t breakcore-viz .
docker run -p 8080:3000 breakcore-viz
```

Or with docker-compose:
```bash
docker-compose up
```

## Modifying Parameters

All visual parameters are in `assets/config.toml`. Changes take effect on next load.

Key parameters to tweak:

```toml
[audio]
kick_onset_threshold = 0.15    # Lower = more sensitive kicks
hat_onset_threshold = 0.10     # Lower = more sensitive hats

[effects]
glitch_intensity_decay = 0.80  # Lower = longer glitch trails
bloom_threshold = 0.5          # Higher = less bloom
feedback_decay = 0.52          # Higher = more ghosting

[camera]
shake_kick_amplitude = 1.2     # Higher = more violent shake
shake_chaos_amplitude = 0.3    # Higher = more jitter at high chaos
```

## Shader Development

Shaders are in `assets/shaders/*.wgsl` (WebGPU Shading Language).

Hot-reloading is supported in debug builds:
1. Edit shader file
2. Rerun app
3. Changes apply immediately

Key shaders:

- **core.wgsl**: Raymarched form (128 steps, distance field)
- **bloom.wgsl**: 9-tap Gaussian blur + bright pixel extraction
- **feedback.wgsl**: Recursive zoom with frame composition

## Audio Analysis Tuning

In `src/audio.rs`, adjust onset detection sensitivity:

```rust
// Current thresholds in AudioAnalyzer::update()
let kick_onset = total_energy - prev > 0.15 && self.kick_energy > 0.4;
let hat_onset = total_energy - prev > 0.10 && self.hatEnergy > 0.3;
```

Lower values = more onsets detected (more visual reactions)
Higher values = less sensitive (fewer reactions)

## Performance Profiling

Run with debug info:
```bash
RUST_BACKTRACE=1 cargo run --release
```

Watch for:
- Particle count (top-left corner)
- FPS drop at high chaos (>8 simultaneous onsets)
- Memory growth over 10+ minutes

## Testing with Breakcore Tracks

Recommended test tracks:
- **Venetian Snares** - "Rossz Csillag Alatt Született" (recursive, steady BPM)
- **Sewerslvt** - "Drowned in the Sun" (glitchy, chaotic)
- **Machine Girl** - "USA" (dense, fast onsets)
- **Igorrr** - "Aloha" (extreme contrast, heavy kicks)

Each reveals different aspects:
1. Venetian Snares → Smooth morphing, consistent patterns
2. Sewerslvt → Glitch triggering, hue cycling
3. Machine Girl → Particle density, chaos scaling
4. Igorrr → Camera shake violence, form scaling

## Debugging

Common issues:

**Shader not compiling**:
- Check WGSL syntax (WebGPU, not GLSL)
- Use line numbers from error message

**Particles not visible**:
- Check particle spawn counts in config.toml
- Verify color mapping (HSL values 0-360, 0-100, 0-100)

**Audio not working**:
- Browser may need user gesture (click play button first)
- Check browser console for Web Audio API errors
- Verify audio context is resumed: `audioCtx.resume()`

**Poor performance**:
- Reduce max_particles in config.toml
- Reduce raymarching_steps (currently 120)
- Disable bloom/feedback effects in renderer.rs

## Deployment

### Quick Deploy to Server

```bash
# Make sure you have SSH access to your server
chmod +x deploy.sh
./deploy.sh tt@192.168.1.9

# Access at: http://192.168.1.9:8080
```

### Manual Docker Deployment

```bash
# Build
docker build -t breakcore-viz .

# Run locally
docker run -p 8080:3000 \
  -v $(pwd)/assets:/app/assets:ro \
  -e RUST_LOG=info \
  breakcore-viz

# Or use docker-compose (recommended)
docker-compose up -d
```

### Deployment Checklist

Before pushing to production:

- [ ] Test with 5+ different breakcore tracks
- [ ] Verify particle count stays under 3000
- [ ] Check glitch effect timing matches beats
- [ ] Confirm visual effects scale with energy correctly
- [ ] Test fullscreen and various resolutions
- [ ] Validate performance (target: 60 FPS at 1280x720)
- [ ] Confirm no audio latency issues
- [ ] Test with both local files and streaming audio

## Next Development Goals

Priority order:

1. **Render pipeline integration** (5-7 days)
   - Connect WebGPU rendering
   - Link shader parameters to audio data
   - Implement framebuffer composition

2. **Audio API integration** (2-3 days)
   - Web Audio API wrapper
   - Real FFT analysis
   - Proper sample rate handling

3. **Performance optimization** (1-2 days)
   - Profile with WebGPU timing queries
   - Compute shader particles (if needed)
   - Memory pooling for particles

4. **Advanced features** (3-5 days)
   - Multiple form morphing states
   - Procedural geometry variation
   - Real-time parameter animation

5. **Release prep** (2-3 days)
   - Documentation polish
   - Example configurations for different styles
   - Deployment scripts

---

**Questions?** Check README.md for architecture details or PROJECT_STATUS.md for current progress.
