# 🛡️ AI Powered Sandboxed EDR CopyRight By Mahendra Ribadiya  
Founder of https://www.quantsafe-ai.com

AI-powered, sandboxed EDR written in Rust for cross-platform endpoint protection (Linux & Windows).

## 🧠 Features

- Process, file, and network monitoring
- Sandboxed threat analysis
- AI-driven threat detection (ONNX/PyTorch)
- Cross-platform Rust Agent
- Dashboard + Backend for remote control

## 🏗️ Repo Structure

- `agent/`: Core Rust EDR Agent
- `ai-server/`: Optional Python AI inference backend
- `dashboard/`: Admin UI
- `backend-api/`: Central log collection + alerting
- `models/`: Trained AI models (ONNX)
- `docs/`: Technical architecture, sandboxing, API docs

## 🚀 Quickstart (Agent)

```bash
cd agent
cargo build --release
./target/release/Fortimind-agent
