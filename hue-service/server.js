const http = require('http');

const HUE_BRIDGE_IP = process.env.HUE_BRIDGE_IP || '192.168.1.10';
const HUE_API_KEY = process.env.HUE_API_KEY || '';
const LIGHT_BIG = process.env.HUE_LIGHT_BIG || '1';
const LIGHT_PC = process.env.HUE_LIGHT_PC || '2';
const LIGHT_SCREEN = process.env.HUE_LIGHT_SCREEN || '3';
const PORT = 3001;

const rateLimitMap = new Map();

function getRateLimit(key) {
  const now = Date.now();
  const bucket = rateLimitMap.get(key) || { tokens: 10, lastRefill: now };
  const timePassed = (now - bucket.lastRefill) / 1000;
  bucket.tokens = Math.min(10, bucket.tokens + timePassed * 10);
  bucket.lastRefill = now;
  rateLimitMap.set(key, bucket);
  return bucket;
}

function callHueLight(lightId, state) {
  return new Promise((resolve) => {
    const body = JSON.stringify(state);
    const path = `/api/${HUE_API_KEY}/lights/${lightId}/state`;
    const options = {
      hostname: HUE_BRIDGE_IP,
      port: 80,
      path,
      method: 'PUT',
      headers: { 'Content-Type': 'application/json', 'Content-Length': Buffer.byteLength(body) },
    };

    const req = http.request(options, (res) => {
      let data = '';
      res.on('data', (chunk) => {
        data += chunk;
      });
      res.on('end', () => {
        resolve(data);
      });
    });

    req.on('error', (err) => {
      console.error(`Hue error (light ${lightId}):`, err.message);
      resolve(null);
    });

    req.setTimeout(2000, () => req.destroy());
    req.write(body);
    req.end();
  });
}

function mapAudioToHueState(audioData) {
  const { sub, low, mid, high, energy, kick, section, anticipation = 0, timeToNextDrop = 999, bpm = 0 } = audioData;
  const time = Date.now() / 1000;

  // Only process if there's actual audio (energy > threshold)
  if (energy < 0.05) {
    return null; // Skip update - no audio playing
  }

  const tremblePhase = Math.sin(time * 8) * 0.5 + 0.5; // 8Hz trembling (faster)
  const isLowEnergy = energy < 0.35;

  // Anticipation boost: builds drama 3s before a drop
  const anticipationBoost = anticipation * anticipation; // Squared for more drama
  const effectiveEnergy = Math.min(1, energy + anticipationBoost * 0.3); // Boost energy by anticipation

  const sectionColors = {
    breakdown: { hue: 5000, sat: 200 },
    buildup: { hue: 8000, sat: 220 },
    drop: { hue: 43000, sat: 150 },
    sustain: { hue: 50000, sat: 180 },
  };

  const sectionColor = sectionColors[section] || sectionColors.sustain;
  const lights = {};

  // Light 1 (Big room): Section mood + energy intensity + anticipation buildup
  const bigBriBoost = anticipationBoost * 80; // Anticipation adds up to 80 brightness

  if (isLowEnergy && anticipation < 0.5) {
    // LOW ENERGY (not anticipating): Active trembling, more color variation
    const fearTremble = tremblePhase * 60 + 50; // 50-110 range
    const hueWander = Math.sin(time * 2) * 3000; // Slow hue wander
    lights[LIGHT_BIG] = {
      on: true,
      bri: Math.round(fearTremble + bigBriBoost),
      hue: Math.round((sectionColor.hue + hueWander) % 65536),
      sat: Math.round(sectionColor.sat * (0.7 + tremblePhase * 0.3)),
      transitiontime: 0,
    };
  } else {
    // HIGH ENERGY or ANTICIPATING DROP: Dramatic, saturated, big hue shifts
    const hueShift = (effectiveEnergy > 0.7 ? effectiveEnergy * 10000 : effectiveEnergy * 5000) + (anticipation * 5000);
    lights[LIGHT_BIG] = {
      on: true,
      bri: Math.round(80 + effectiveEnergy * 170 + bigBriBoost), // Anticipation adds brightness
      hue: Math.round((sectionColor.hue + hueShift) % 65536),
      sat: Math.round(sectionColor.sat * (0.8 + effectiveEnergy * 0.2 + anticipation * 0.2)),
      transitiontime: 0,
    };
  }

  // Light 2 (Left desk): Bass-dominant, responsive to kicks
  const bassEnergy = Math.min(1, (sub * 2.5 + low * 1.5) / 2);
  const kickIntensity = Math.max(0, Math.min(1, kick * 2.0));

  if (isLowEnergy) {
    // LOW: Active bass rumble, pulsing brightness
    lights[LIGHT_PC] = {
      on: true,
      bri: Math.round(60 + bassEnergy * 70 + tremblePhase * 40), // 60-170 range
      hue: 4000 + (bassEnergy * 2000), // Orange to red-orange with bass
      sat: 220 + (tremblePhase * 34), // Pulsing saturation
      transitiontime: 0,
    };
  } else {
    // HIGH: Dramatic kick response, red when pounding
    lights[LIGHT_PC] = {
      on: true,
      bri: Math.round(100 + bassEnergy * 150 + kickIntensity * 50),
      hue: 3000 + (kickIntensity * 2000), // Red to orange on kicks
      sat: 240,
      transitiontime: 0,
    };
  }

  // Light 3 (Screen): Full spectrum response
  const fullEnergy = Math.min(1, (sub * 0.5 + low * 0.8 + mid * 1.2 + high * 1.5) / 2);

  if (isLowEnergy) {
    // LOW: Active cool glow, pulsing
    const colorWander = Math.cos(time * 1.5) * 2000; // Color oscillates
    lights[LIGHT_SCREEN] = {
      on: true,
      bri: Math.round(70 + fullEnergy * 70 + tremblePhase * 40), // 70-180 range
      hue: Math.round((46000 + colorWander) % 65536), // Purple oscillates
      sat: Math.round(160 + tremblePhase * 94), // Pulsing saturation 160-254
      transitiontime: 0,
    };
  } else {
    // HIGH: Dramatic spectrum sweep, saturated
    const screenHue = 44000 + (fullEnergy * 10000); // Blue to magenta
    lights[LIGHT_SCREEN] = {
      on: true,
      bri: Math.round(100 + fullEnergy * 150),
      hue: Math.round(screenHue % 65536),
      sat: Math.round(200 + fullEnergy * 54),
      transitiontime: 0,
    };
  }

  return lights;
}

const server = http.createServer(async (req, res) => {
  res.setHeader('Access-Control-Allow-Origin', '*');
  res.setHeader('Access-Control-Allow-Methods', 'GET, POST, OPTIONS');
  res.setHeader('Access-Control-Allow-Headers', 'Content-Type');

  if (req.method === 'OPTIONS') {
    res.writeHead(200);
    res.end();
    return;
  }

  if (req.method === 'POST' && req.url === '/sync') {
    const bucket = getRateLimit('sync');
    if (bucket.tokens < 1) {
      res.writeHead(429, { 'Content-Type': 'application/json' });
      res.end(JSON.stringify({ error: 'rate limited' }));
      return;
    }
    bucket.tokens -= 1;

    let body = '';
    req.on('data', (chunk) => {
      body += chunk;
    });
    req.on('end', async () => {
      try {
        const audioData = JSON.parse(body);
        const hueStates = mapAudioToHueState(audioData);

        if (!hueStates) {
          res.writeHead(200, { 'Content-Type': 'application/json' });
          res.end(JSON.stringify({ ok: true, skipped: 'energy too low' }));
          return;
        }

        console.log(`[${new Date().toISOString()}] energy=${audioData.energy.toFixed(3)} kick=${audioData.kick.toFixed(3)} section=${audioData.section}`);
        console.log('  big:', hueStates[LIGHT_BIG]);
        console.log('  left:', hueStates[LIGHT_PC]);
        console.log('  screen:', hueStates[LIGHT_SCREEN]);

        const promises = Object.entries(hueStates).map(([lightId, state]) => callHueLight(lightId, state));
        await Promise.all(promises);

        res.writeHead(200, { 'Content-Type': 'application/json' });
        res.end(JSON.stringify({ ok: true }));
      } catch (err) {
        console.error('Sync error:', err.message);
        res.writeHead(400, { 'Content-Type': 'application/json' });
        res.end(JSON.stringify({ error: err.message }));
      }
    });
    return;
  }

  if (req.url === '/health' && req.method === 'GET') {
    res.writeHead(200, { 'Content-Type': 'application/json' });
    res.end(JSON.stringify({ status: 'ok', service: 'hue-service' }));
    return;
  }

  res.writeHead(404);
  res.end();
});

server.listen(PORT, () => {
  console.log(`Hue service running on port ${PORT}`);
  console.log(`Bridge: ${HUE_BRIDGE_IP}`);
  console.log(`Lights: big=${LIGHT_BIG}, pc=${LIGHT_PC}, screen=${LIGHT_SCREEN}`);
});
