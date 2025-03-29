import http from 'k6/http';
import { check, sleep } from 'k6';
import { BASE_URL, DURATION, HEADERS, validateJsonResponse, getRandomInt } from './config.js';

export let options = {
  stages: DURATION.stages,
  thresholds: {
    http_req_duration: ['p(99)<300'], // Update operations are allowed more time as they include writes
    checks: ['rate>0.99'], // 99% of checks must pass
  },
};

// Data Updates test - updates random records in the database
export default function() {
  // Set JSON-specific headers
  const headers = {
    ...HEADERS,
    'Accept': 'application/json,text/html;q=0.9,application/xhtml+xml;q=0.9,application/xml;q=0.8,*/*;q=0.7',
  };
  
  const queryCount = getRandomInt(1, 20);
  
  // Make the request to the updates endpoint
  const url = `${BASE_URL}/updates?q=${queryCount}`;
  const response = http.get(url, { headers });
  
  check(response, {
    'status is 200': (r) => r.status === 200,
    'content-type is application/json': (r) => r.headers['Content-Type'] && r.headers['Content-Type'].includes('application/json'),
    'response is valid JSON': (r) => validateJsonResponse(r),
    'has correct structure': (r) => {
      try {
        const body = JSON.parse(r.body);
        
        // Verify that we have an array with the right number of entries
        if (!Array.isArray(body) || body.length !== queryCount) {
          return false;
        }
        
        // Verify each object in the array
        return body.every(item => (
          typeof item === 'object' &&
          item !== null &&
          typeof item.id === 'number' &&
          typeof item.randomNumber === 'number' &&
          item.id >= 1 &&
          item.id <= 10000 &&
          item.randomNumber >= 1 &&
          item.randomNumber <= 10000
        ));
      } catch (e) {
        return false;
      }
    },
  });
} 