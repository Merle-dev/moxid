#!/usr/bin/env bash

# Build the Docker image
docker build -t tauri-builder .

# Run the build and copy output
docker run --rm \
  -v $(pwd):/app \
  tauri-builder \
  cargo tauri build
