# Testing & Deployment Guide

## Pre-Deployment Checklist

### Local Testing (if Rust is available)

```bash
# 1. Build native server
cargo build --release

# 2. Build WASM visualizer
./build-wasm.sh

# 3. Run server
./target/release/breakcore_viz
# Should output:
#   🎵 Breakcore Visualizer Server
#   Listening on http://localhost:3000

# 4. Test in browser
# Open http://localhost:3000 in Chrome/Firefox
```

### Docker Testing (recommended)

```bash
# Build and run
docker-compose up

# In another terminal, test endpoints
curl http://localhost:8080/health
# Should return: {"status":"ok","service":"breakcore-viz"}

# Check logs
docker-compose logs -f
```

## Server Deployment

### Step 1: Deploy to Server

```bash
chmod +x deploy.sh
./deploy.sh tt@192.168.1.9
```

### Step 2: Verify Deployment

```bash
# Check if service is running
ssh tt@192.168.1.9 'docker-compose -f /home/tt/breakcore-viz/docker-compose.yml ps'

# Check logs
ssh tt@192.168.1.9 'docker-compose -f /home/tt/breakcore-viz/docker-compose.yml logs'

# Test health endpoint
curl http://192.168.1.9:8080/health
```

### Step 3: Access Visualizer

- **URL**: http://192.168.1.9:8080
- **Local network only**: Accessible from your home network
- **Test audio files needed**: MP3 or WAV files

## Testing with Breakcore Tracks

The visualizer was designed for these artists. Use these to validate behavior:

### Test Track 1: Venetian Snares - "Rossz Csillag Alatt Született"
- **Expected behavior**: Smooth morphing, consistent patterns, recursive spirals
- **Tests**: Core form scaling, steady particle spawning, hue rotation
- **Duration**: 6+ minutes for extended testing

### Test Track 2: Sewerslvt - "Drowned in the Sun"
- **Expected behavior**: Glitch triggering, VHS corruption effect, intense color cycling
- **Tests**: Glitch engine, color effects, transient responsiveness
- **Duration**: ~3 minutes, very chaotic

### Test Track 3: Machine Girl - "USA"
- **Expected behavior**: Dense particles, high chaos metric, rapid color changes
- **Tests**: Particle density scaling, chaos-driven effects, FPS stability
- **Duration**: ~2 minutes, ultra-fast

### Test Track 4: Igorrr - "Aloha"
- **Expected behavior**: Extreme camera shake on kicks, form scaling, color inversion
- **Tests**: Screen shake amplitude, energy scaling, contrast effects
- **Duration**: ~4 minutes, dynamic intensity

## Performance Validation

While visualizing, check:

- **FPS**: Monitor with browser DevTools (F12 > Performance)
  - Target: 60 FPS @ 1280x720
  - Acceptable: 30+ FPS at peak chaos

- **Memory**: Watch with DevTools Memory tab
  - Initial: ~50-100 MB
  - After 5 min: should not exceed 200 MB
  - If growing: particle cleanup may have issues

- **Network**: Minimal impact
  - WASM module loaded once (~500KB)
  - Audio frames: real-time analysis only
  - No external API calls (unless added later)

## Troubleshooting

### "Cannot connect to server"
```bash
# Check if service is running
ssh tt@192.168.1.9 'docker ps'

# Restart
ssh tt@192.168.1.9 'cd /home/tt/breakcore-viz && docker-compose restart'
```

### "Visualizer not rendering"
1. Check browser console (F12)
2. Verify WASM module loaded: Network tab should show `.wasm` files
3. Try uploading audio file
4. Check browser supports WebGL: https://webglreport.com

### "Audio not playing"
1. Browser may require user gesture (click play button)
2. Check audio file format (MP3/WAV supported by browser)
3. Check browser Web Audio API permissions
4. Try test with different audio file

### "Poor performance / high CPU"
1. Reduce browser window size (rendering is raster-based)
2. Close other browser tabs
3. Check GPU acceleration enabled (DevTools > Settings)
4. Use release build of server (`cargo build --release`)

## Integration with Homepage

Once validated, integrate into your homepage dashboard:

1. Create `iframe` to `http://192.168.1.9:8080`
2. Or embed as `/breakcore` route
3. Set viewport to `1280x720` for optimal sizing
4. Consider lazy-loading for homepage performance

## Rollback

If issues occur:

```bash
ssh tt@192.168.1.9 'cd /home/tt/breakcore-viz && docker-compose down'
# Or use previous Docker image if available
docker-compose down --volumes
```

## Next Steps

After successful deployment:

1. ✅ Add to homepage as feature
2. ⏳ Enhance audio analysis for better precision
3. ⏳ Add microphone input option
4. ⏳ Implement raymarched 3D form
5. ⏳ Optimize particle rendering with WebGL instancing
