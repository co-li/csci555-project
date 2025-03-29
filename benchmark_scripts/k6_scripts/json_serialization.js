import http from 'k6/http';
import { check, sleep } from 'k6';
import { BASE_URL, DURATION, HEADERS, validateJsonResponse } from './config.js';

export let options = {
  stages: DURATION.stages,
  thresholds: {
    http_req_duration: ['p(99)<50'], // 99% of requests must complete below 50ms
    checks: ['rate>0.99'], // 99% of checks must pass
  },
};

// JSON endpoint test - tests serialization of a simple JSON object
export default function() {
  // Set JSON-specific headers
  const headers = {
    ...HEADERS,
    'Accept': 'application/json,text/html;q=0.9,application/xhtml+xml;q=0.9,application/xml;q=0.8,*/*;q=0.7',
  };
  
  // Make the request to the JSON endpoint
  const url = `${BASE_URL}/json`;
  const response = http.get(url, { headers });
  
  check(response, {
    'status is 200': (r) => r.status === 200,
    'content-type is application/json': (r) => r.headers['Content-Type'] && r.headers['Content-Type'].includes('application/json'),
    'response is valid JSON': (r) => validateJsonResponse(r),
    'has correct structure': (r) => {
      try {
        const body = JSON.parse(r.body);
        return body.message === 'Hello, World!';
      } catch (e) {
        return false;
      }
    },
  });
} 