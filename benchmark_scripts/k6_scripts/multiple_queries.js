import http from 'k6/http';
import { check, sleep } from 'k6';
import { BASE_URL, DURATION, HEADERS, validateJsonResponse, getRandomInt } from './config.js';

export let options = {
  stages: DURATION.stages,
  thresholds: {
    http_req_duration: ['p(99)<200'], // 99% of requests must complete below 200ms
    checks: ['rate>0.99'], // 99% of checks must pass
  },
};

// Multiple DB Queries test - fetches multiple random records from the database
export default function() {
  // Set JSON-specific headers
  const headers = {
    ...HEADERS,
    'Accept': 'application/json,text/html;q=0.9,application/xhtml+xml;q=0.9,application/xml;q=0.8,*/*;q=0.7',
  };
  
  // Randomly select a query count between 1-500, as per TFB
  // TFB uses specific query levels: 1, 5, 10, 15, 20 but we'll randomize to simulate varied traffic
  const queryCount = getRandomInt(1, 20);
  
  // Make the request to the queries endpoint
  const url = `${BASE_URL}/queries?q=${queryCount}`;
  const response = http.get(url, { headers });
  
  // Verify the response - this mimics the TFB verification logic
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