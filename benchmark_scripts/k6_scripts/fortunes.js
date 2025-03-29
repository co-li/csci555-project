import http from 'k6/http';
import { check, sleep } from 'k6';
import { BASE_URL, DURATION, HEADERS } from './config.js';

export let options = {
  stages: DURATION.stages,
  thresholds: {
    http_req_duration: ['p(99)<150'], // 99% of requests must complete below 150ms
    checks: ['rate>0.99'], // 99% of checks must pass
  },
};

// Fortune test - fetches fortune messages from DB and renders HTML
export default function() {
  // Set HTML-specific headers
  const headers = {
    ...HEADERS,
    'Accept': 'text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8',
  };
  
  // Make the request to the fortunes endpoint
  const url = `${BASE_URL}/fortunes`;
  const response = http.get(url, { headers });
  
  // Verify the response - this mimics the TFB verification logic
  check(response, {
    'status is 200': (r) => r.status === 200,
    'content-type is text/html': (r) => r.headers['Content-Type'] && r.headers['Content-Type'].includes('text/html'),
    'has fortune table': (r) => r.body.includes('<table>'),
    'has expected structure': (r) => {
      // Basic HTML validation - proper doctype and structure
      const body = r.body.toString();
      
      // Check if it's a complete HTML document with a table
      return (
        body.includes('<!DOCTYPE html>') && 
        body.includes('<html>') && 
        body.includes('<head>') && 
        body.includes('<title>Fortunes</title>') && 
        body.includes('<body>') && 
        body.includes('<table>') && 
        body.includes('<tr><th>id</th><th>message</th></tr>') && 
        body.includes('</table>') && 
        body.includes('</body>') && 
        body.includes('</html>')
      );
    },
    'has appropriate fortune count': (r) => {
      // Count the number of <tr> elements (should be at least 12 - the default fortunes plus "Additional fortune added at request time")
      const body = r.body.toString();
      const trMatches = body.match(/<tr>/g);
      return trMatches && trMatches.length >= 12;
    }
  });
} 