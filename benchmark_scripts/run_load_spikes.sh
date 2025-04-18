#!/bin/bash

# Default values
SERVER_URL="http://localhost:8000"

# Help function
show_help() {
  echo "Usage: $0 [options]"
  echo "Options:"
  echo "  -h, --help                   Show this help message"
  echo "  -s, --server <url>           Target server URL (default: http://localhost:8000)"
  echo "  -o, --output-dir <dir>       Output directory for results (default: ./results/YYYY-MM-DD_HH-MM-SS)"
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
echo "=== Web Server Load Spike Test ==="
echo "Target Server:     $SERVER_URL"
echo "==================================="

export BASE_URL="$SERVER_URL"

# Run k6 test
k6 run --out json="$OUTPUT_DIR/load_spikes_results.json" benchmark_scripts/k6_scripts/load_spikes.js


# Notify completion
echo "Load spike test completed. Results saved in $OUTPUT_DIR."