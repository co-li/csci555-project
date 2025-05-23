#!/bin/bash

# Generated by Cursor

# Default values
SERVER_URL="http://localhost:8000"
DURATION="30s"
RAMP_UP="10s"
CONCURRENCY_LEVEL=100
ENDPOINT="/json"
MODE="concurrency"
DELAY=0
OUTPUT_DIR="output"
ITERATIONS=1

# 10 50 100 200 500 1000

# Help function
function show_help {
  echo "Usage: $0 [options]"
  echo "Options:"
  echo "  -h, --help                   Show this help message"
  echo "  -s, --server <url>           Target server URL (default: http://localhost:8000)"
  echo "  -d, --delay <delay>    "
  echo "  -c, --concurrency <level>    concurrency level (default: 100)"
  echo "  -e, --endpoint <endpoint>    String of endpoint to test (default: /json)"
  echo "  -m, --mode <mode>             Test mode (default: concurrency)"
  echo "  -i, --iteration <num>        Iteration number (default: 1)"
  echo ""
}

# Parse command line arguments
while [[ $# -gt 0 ]]; do
  key="$1"
  case $key in
    -h|--help)
      show_help
      exit 0
      ;;
    -s|--server)
      SERVER_URL="$2"
      shift 2
      ;;
    -d|--delay)
      DELAY="$2"
      shift 2
      ;;
    -c|--concurrency)
      CONCURRENCY_LEVEL="$2"
      shift 2
      ;;
    -e|--endpoint)
      ENDPOINT="$2"
      shift 2
      ;;
    -m|--mode)
      MODE="$2"
      shift 2
      ;;
    -i|--iteration)
      ITERATIONS="$2"
      shift 2
      ;;
    *)
      echo "Unknown option: $1"
      show_help
      exit 1
      ;;
  esac
done

# Create output directory
mkdir -p "$OUTPUT_DIR"

# Print test configuration
echo "=== Web Server Framework Concurrency Test ==="
echo "Target Server:     $SERVER_URL"
echo "Concurrency Levels: ${CONCURRENCY_LEVEL}"
echo "Endpoint:         $ENDPOINT"
echo "Test Mode:        $MODE"
echo "Delay:           $DELAY"
echo "Iterations:       $ITERATIONS"
echo "==============================================="

# Set environment variables for k6
export BASE_URL="$SERVER_URL"
export CONCURRENCY="$CONCURRENCY_LEVEL"
export ENDPOINT="$ENDPOINT"
export TEST_TYPE="$MODE"
export DELAY="$DELAY"

# Run k6 test
if [[ "$MODE" == "spike" ]]; then
  k6 run --out json="${MODE}_${ENDPOINT:1}.json" benchmark_scripts/k6_test.js
else
  for ((i=1; i<=$ITERATIONS; i++))
  do
    echo "Running iteration $i of $ITERATIONS..."
    export ITERATION="$i"
    k6 run benchmark_scripts/k6_test.js
    sleep 1
  done
fi

# Wait a moment between tests
sleep 1
