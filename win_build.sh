#!/bin/bash

set -e

# Create build directory
mkdir -p ./build
cd ./build

# Compile git_snapshot.c to Windows executable
echo "[BUILD] Compiling git_snapshot.exe ..."
rustc ../src/main.rs -o git_snapshot.exe

echo "[OK] Windows build completed: build/windows/git_snapshot.exe"
