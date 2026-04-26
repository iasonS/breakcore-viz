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

  const sectionColors = {
    breakdown: { hue: 5000, sat: 200 },
    buildup: { hue: 8000, sat: 220 },
    drop: { hue: 43000, sat: 150 },
    sustain: { hue: 50000, sat: 180 },
  };

  const sectionColor = sectionColors[section] || sectionColors.sustain;

  const lights = {};

  lights[LIGHT_BIG] = {
    on: true,
    bri: Math.max(100, Math.round(energy * 254)),
    hue: sectionColor.hue,
    sat: sectionColor.sat,
    transitiontime: 1,
  };

  const bassEnergy = Math.min(1, (sub * 2 + low) / 2);
  const kickIntensity = Math.max(0, Math.min(1, kick * 1.5));
  const pcHue = kickIntensity > 0.7 ? 3000 : 6000;
  const pcSat = 220 + (kickIntensity * 34);

  lights[LIGHT_PC] = {
    on: true,
    bri: Math.max(50, Math.round((bassEnergy * 0.7 + kickIntensity * 0.3) * 254)),
    hue: Math.round(pcHue),
    sat: Math.round(Math.min(254, pcSat)),
    transitiontime: 1,
  };

  const trebleEnergy = Math.min(1, (mid * 0.7 + high * 1.3) / 2);
  lights[LIGHT_SCREEN] = {
    on: true,
    bri: Math.max(50, Math.round(trebleEnergy * 254)),
    hue: 44000 + Math.round(trebleEnergy * 6000),
    sat: Math.round(150 + trebleEnergy * 104),
    transitiontime: 1,
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
