export const BASE_URL = 'http://localhost:8000';

export const DURATION = {
  stages: [
    // Primer - short test to establish connections
    { duration: '5s', target: 8 },
    // Warmup - ramp up to highest concurrency
    { duration: '5s', target: 512 },
    // Actual test at specified concurrency levels
    { duration: '15s', target: 16 },
    { duration: '15s', target: 32 },
    { duration: '15s', target: 64 },
    { duration: '15s', target: 128 },
    { duration: '15s', target: 256 },
    { duration: '15s', target: 512 },
  ],
};

// Common headers
export const HEADERS = {
  'Host': 'localhost',
  'Connection': 'keep-alive',
};

export const THRESHOLDS = {
  http_req_duration: ['p(95)<500'], // 95% of requests must complete below 500ms
  checks: ['rate>0.99'], // 99% of checks must pass
}

export const CONCURRENCY_LEVELS = [16, 32, 64, 128, 256, 512];

export function validateJsonResponse(response) {
  if (response.status !== 200) {
    console.log(`Error: Status code ${response.status}`);
    return false;
  }
  
  if (!response.headers['Content-Type'] || !response.headers['Content-Type'].includes('application/json')) {
    console.log(`Error: Invalid Content-Type ${response.headers['Content-Type']}`);
    return false;
  }
  
  try {
    let body = JSON.parse(response.body);
    return true;
  } catch (e) {
    console.log(`Error parsing JSON: ${e}`);
    return false;
  }
}

export function getRandomInt(min, max) {
  min = Math.ceil(min);
  max = Math.floor(max);
  return Math.floor(Math.random() * (max - min + 1)) + min;
}
