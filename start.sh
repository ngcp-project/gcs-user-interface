#!/bin/bash
echo "Starting tile server..."
bun run osm:run &
OSM_PID=$!
echo "Starting tauri app..."
bun run tauri
kill $OSM_PID