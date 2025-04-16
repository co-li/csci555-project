import http from 'k6/http';
import { check, sleep } from 'k6';
import { BASE_URL, DURATION, HEADERS, THRESHOLDS } from './config.js';

export let options = {
  stages: DURATION.stages,
  thresholds: THRESHOLDS,
};

// Plaintext test - tests raw HTTP request/response performance
export default function() {
  // Set plaintext-specific headers
  const headers = {
    ...HEADERS,
    'Accept': 'text/plain,text/html;q=0.9,application/xhtml+xml;q=0.9,application/xml;q=0.8,*/*;q=0.7',
  };
  
  // Make the request to the plaintext endpoint
  const url = `${BASE_URL}/plaintext`;
  const response = http.get(url, { headers });
  
  // Verify the response - this mimics the TFB verification logic
  check(response, {
    'status is 200': (r) => r.status === 200,
    'content-type is text/plain': (r) => r.headers['Content-Type'] && r.headers['Content-Type'].includes('text/plain'),
    'body is exactly Hello, World!': (r) => r.body === 'Hello, World!',
    'content-length header matches body length': (r) => {
      const contentLength = r.headers['Content-Length'];
      return contentLength && parseInt(contentLength) === 'Hello, World!'.length;
    }
  });

  
} 