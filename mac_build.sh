#!/bin/bash

# Create build directory
mkdir -p ./build
cd ./build

# Compile C source file
rustc ../src/main.rs -o git_snapshot

# Check system type
OS_TYPE=$(uname)

# Create startup script content
cat > run_common <<'EOF'
#!/bin/zsh
# Automatically switch to script directory
cd "$(dirname "$0")"
./git_snapshot
echo ""
read "REPLY?Press Enter to close window..."
EOF

# Generate different startup scripts based on system type
if [[ "$OS_TYPE" == "Darwin" ]]; then
    # macOS
    mv run_common run.command
    chmod +x run.command
    echo "[OK] macOS: Created ./build/run.command (double-click to run)"
elif [[ "$OS_TYPE" == "Linux" ]]; then
    # Linux
    mv run_common run.sh
    chmod +x run.sh
    echo "[OK] Linux: Created ./build/run.sh (run in terminal)"
else
    echo "[ERROR] Unrecognized system type: $OS_TYPE"
fi

# Add execute permission to the executable
chmod +x git_snapshot
