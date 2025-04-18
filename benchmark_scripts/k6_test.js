// Generated by Cursor

import http from 'k6/http';
import { check } from 'k6';
import { Counter, Rate, Trend } from 'k6/metrics';

// Custom metrics
const successRate = new Rate('success_rate');
const failureRate = new Rate('failure_rate');
const requestDurationTrend = new Trend('request_duration');
const requestsPerSecCounter = new Counter('requests_per_sec');

// Read concurrency from environment variables with defaults
const BASE_URL = __ENV.BASE_URL || 'http://localhost:8000';
const concurrency = __ENV.CONCURRENCY ? parseInt(__ENV.CONCURRENCY) : 10;
const duration = '30s';
const rampUpTime = '10s';
const targetServer = BASE_URL;
const delay = __ENV.DELAY ? parseInt(__ENV.DELAY) : 0;
const endpoint = __ENV.ENDPOINT ? __ENV.ENDPOINT : '/';
const testType = __ENV.TEST_TYPE ? __ENV.TEST_TYPE : 'concurrency';

// TODO: /img, /imgs, /vid

const THRESHOLDS = {
  http_req_duration: ['p(95)<500'], // 95% of requests must complete below 500ms
  checks: ['rate>0.99'], // 99% of checks must pass
}

export let options = (() => {
  if (testType === 'concurrency') {
    return {
      scenarios: {
        constant_load: {
          executor: 'ramping-vus',
          startVUs: 1,
          stages: [
            { duration: rampUpTime, target: concurrency }, // Ramp-up to the desired concurrency
            { duration: duration, target: concurrency },   // Stay at target concurrency
            { duration: '5s', target: 0 },                 // Ramp-down
          ],
          gracefulRampDown: '5s',
        },
      },
      // thresholds: THRESHOLDS,
    };
  } else if (testType === 'spike') {
    return {
      stages: [
        // Warm-up: Gradually ramp up to baseline load
        { duration: '10s', target: 100 },   // Start with 10 users
        
        // Steady state: Maintain baseline load
        { duration: '10s', target: 100 },   
        
        // Sudden spike: Quick ramp up to high load
        { duration: '10s', target: 10000 },  // Sudden spike to 10000 users
        
        // Sustained peak: Maintain high load
        { duration: '10s', target: 10000 },
        
        // Recovery: Quick drop back to baseline
        { duration: '5s', target: 100 },
        
        // Verify stability: Maintain baseline after spike
        { duration: '10s', target: 100 },
    
        // Sudden spike: Quick ramp up to high load
        { duration: '10s', target: 10000 },  // Sudden spike to 10000 users
        
        // Sustained peak: Maintain high load
        { duration: '10s', target: 10000 },
    
        // Recovery: Quick drop back to baseline
        { duration: '5s', target: 100 },
        
        // Verify stability: Maintain baseline after spike
        { duration: '10s', target: 100 },
      ],
      // thresholds: THRESHOLDS,
    };
  }
})();

export default async function() {
  // Construct full URL
  const url = `${targetServer}${endpoint}`;

  let headers = { };
  
  if (endpoint === '/json') {
    headers['Content-Type'] = 'application/json';
  } else if (endpoint === '/plaintext') {
    headers['Content-Type'] = 'text/plain';
  } else if (endpoint === '/img') {
    headers['Content-Type'] = 'image/jpeg';
  } else if (endpoint === '/helloform') {
    headers['Content-Type'] = 'application/x-www-form-urlencoded';
  } else if (endpoint === '/imgs') {
    headers['Content-Type'] = 'image/png';
  } else if (endpoint === '/vid') {
    headers['Content-Type'] = 'video/mp4';
  } else {
    throw new Error(`Unsupported endpoint: ${endpoint}`);
  }
  
  // Make the request and record the start time
  const startTime = new Date().getTime();
  let response;
  if (endpoint === '/helloform') {
    const payload = `delay=${delay}&message=Hello, world!`;
    response = http.post(url, payload, { headers });
  } else {  
    response = http.get(url, { headers });
  }
  const endTime = new Date().getTime();
  
  // Record metrics
  const duration = endTime - startTime;
  requestDurationTrend.add(duration);
  requestsPerSecCounter.add(1);
  
  // Basic validation based on endpoint
  let success = false;
  
  // check
  if (endpoint === '/json') {
    success = check(response, {
      'status is 200': (r) => r.status === 200,
      'content-type is application/json': (r) => r.headers['Content-Type'] && r.headers['Content-Type'].includes('application/json'),
      'valid json body': (r) => {
        try {
          JSON.parse(r.body);
          return true;
        } catch (e) {
          return false;
        }
      },
    });
  } else if (endpoint === '/plaintext') {
    success = check(response, {
      'status is 200': (r) => r.status === 200,
      'content-type is text/plain': (r) => r.headers['Content-Type'] && r.headers['Content-Type'].includes('text/plain'),
    });
  } else {
    // Generic checks for other endpoints
    success = check(response, {
      'status is 200': (r) => r.status === 200,
    });
  }
  
  // Record success or failure
  successRate.add(success);
  failureRate.add(!success);
}

// Summary output function that will run after the test
export function handleSummary(data) {
  console.log('Test completed with the following configuration:');
  console.log(`- Target Server: ${targetServer}`);
  console.log(`- Concurrency Level: ${concurrency}`);
  console.log(`- Test Duration: ${duration}`);
  console.log(() => {
      if (testType == 'concurrency') {
        return `results_concurrency_${concurrency}_${endpoint.slice(1)}_summary.json`;
      }
      else if (testType == 'spike') {
        return `results_spike_${endpoint.slice(1)}_summary.json`;
      }
    })

  return {
    'stdout': JSON.stringify(data),
    [(() => {
      if (testType === 'concurrency') {
        return `results_concurrency_${concurrency}_${endpoint.slice(1)}_summary.json`;
      }
      else if (testType === 'spike') {
        return `results_spike_${endpoint.slice(1)}_summary.json`;
      }
    })()]: JSON.stringify(data),
  };
}