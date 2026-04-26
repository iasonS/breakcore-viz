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
  const { sub, low, mid, high, energy, kick, section } = audioData;
  const time = Date.now() / 1000;
  const tremblePhase = Math.sin(time * 8) * 0.5 + 0.5; // 8Hz trembling for low energy

  const sectionColors = {
    breakdown: { hue: 5000, sat: 200 },
    buildup: { hue: 8000, sat: 220 },
    drop: { hue: 43000, sat: 150 },
    sustain: { hue: 50000, sat: 180 },
  };

  const sectionColor = sectionColors[section] || sectionColors.sustain;

  const lights = {};

  // Light 1 (Big room): Overall energy + section mood, with dynamic hue shift
  const bigHue = sectionColor.hue + Math.round(energy * 5000); // Hue shifts with energy
  const bigBri = energy < 0.2
    ? Math.round(50 + tremblePhase * 30) // Tremble when energy is low
    : Math.max(80, Math.round(energy * 254));

  lights[LIGHT_BIG] = {
    on: true,
    bri: bigBri,
    hue: Math.round(bigHue % 65536),
    sat: Math.round(sectionColor.sat * (0.7 + energy * 0.3)),
    transitiontime: 0,
  };

  // Light 2 (Left desk): Bass-dominant, very responsive to kicks
  const bassEnergy = Math.min(1, (sub * 2.5 + low * 1.5) / 2);
  const kickIntensity = Math.max(0, Math.min(1, kick * 2.0));
  const pcHue = 3000 + (kickIntensity * 3000); // Red to orange on kicks
  const pcBri = bassEnergy < 0.15
    ? Math.round(30 + tremblePhase * 40)
    : Math.round((bassEnergy * 0.5 + kickIntensity * 0.5) * 254);

  lights[LIGHT_PC] = {
    on: true,
    bri: Math.max(30, pcBri),
    hue: Math.round(pcHue),
    sat: 240,
    transitiontime: 0,
  };

  // Light 3 (Screen): Full spectrum response (bass + treble), cool colors
  const fullEnergy = Math.min(1, (sub * 0.5 + low * 0.8 + mid * 1.2 + high * 1.5) / 2);
  const screenHue = 44000 + Math.round(fullEnergy * 8000); // Blue to magenta
  const screenBri = fullEnergy < 0.15
    ? Math.round(40 + tremblePhase * 50)
    : Math.round(fullEnergy * 254);

  lights[LIGHT_SCREEN] = {
    on: true,
    bri: Math.max(40, screenBri),
    hue: Math.round(screenHue % 65536),
    sat: Math.round(180 + fullEnergy * 74),
    transitiontime: 0,
  };

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

        console.log('Sync:', audioData, '→', hueStates);

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
