#!/bin/bash

# Update package list and install necessary dependencies
sudo apt-get update
sudo apt-get install -y build-essential libssl-dev pkg-config

# Install Rust using rustup
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Source the cargo environment
source $HOME/.cargo/env

# Install additional Rust components if needed
rustup component add rustfmt clippy

# Install Python and pip for AI server dependencies
sudo apt-get install -y python3 python3-pip

# Install FastAPI and other Python dependencies
pip3 install fastapi uvicorn

# Print completion message
echo "Installation completed successfully. Please restart your terminal or run 'source $HOME/.cargo/env' to start using Rust."