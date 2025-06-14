#!/bin/bash

# Build the Rust agent
cd agent
cargo build --release

# Build the AI server
cd ../ai-server
pip install -r requirements.txt

# Build the dashboard
cd ../dashboard
npm install

# Build the backend API
cd ../backend-api
cargo build --release

# Deployment steps can be added here
echo "Build and deployment completed successfully."