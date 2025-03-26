import http from 'k6/http';
import { check, sleep } from 'k6';
import { BASE_URL, DURATION, HEADERS, validateJsonResponse } from './config.js';

export let options = {
  stages: DURATION.stages,
  thresholds: {
    http_req_duration: ['p(99)<100'], // 99% of requests must complete below 100ms
    checks: ['rate>0.99'], // 99% of checks must pass
  },
};

// Single DB Query test - fetches a single random record from the database
export default function() {
  // Set JSON-specific headers
  const headers = {
    ...HEADERS,
    'Accept': 'application/json,text/html;q=0.9,application/xhtml+xml;q=0.9,application/xml;q=0.8,*/*;q=0.7',
  };
  
  // Make the request to the DB endpoint
  const url = `${BASE_URL}/db`;
  const response = http.get(url, { headers });
  
  // Verify the response - this mimics the TFB verification logic
  check(response, {
    'status is 200': (r) => r.status === 200,
    'content-type is application/json': (r) => r.headers['Content-Type'] && r.headers['Content-Type'].includes('application/json'),
    'response is valid JSON': (r) => validateJsonResponse(r),
    'has correct structure': (r) => {
      try {
        const body = JSON.parse(r.body);
        
        // Verify that we have an object with 'id' and 'randomNumber' fields
        return (
          typeof body === 'object' &&
          body !== null &&
          typeof body.id === 'number' &&
          typeof body.randomNumber === 'number' &&
          body.id >= 1 &&
          body.id <= 10000 &&
          body.randomNumber >= 1 &&
          body.randomNumber <= 10000
        );
      } catch (e) {
        return false;
      }
    },
  });
} 