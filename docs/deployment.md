# Deployment Instructions for RustGuard EDR

This document provides instructions for deploying the RustGuard EDR project in various environments. 

## Prerequisites

Before deploying, ensure that the following prerequisites are met:

- Rust programming language installed (version 1.50 or higher)
- Cargo package manager (comes with Rust)
- Python 3.7 or higher (for the AI server)
- Node.js and npm (for the dashboard)
- Access to a suitable database (if using the backend API)

## Deployment Steps

### 1. Clone the Repository

Clone the RustGuard EDR repository from GitHub:

```bash
git clone https://github.com/yourusername/rustguard-edr.git
cd rustguard-edr
```

### 2. Build the EDR Agent

Navigate to the `agent` directory and build the Rust agent:

```bash
cd agent
cargo build --release
```

### 3. Set Up the AI Server

Navigate to the `ai-server` directory and install the required Python packages:

```bash
cd ai-server
pip install -r requirements.txt
```

Run the FastAPI application:

```bash
uvicorn app.main:app --host 0.0.0.0 --port 8000
```

### 4. Deploy the Dashboard

Navigate to the `dashboard` directory and install the required npm packages:

```bash
cd dashboard
npm install
```

Build the dashboard for production:

```bash
npm run build
```

You can serve the built files using a static file server or deploy them to a web server of your choice.

### 5. Set Up the Backend API

Navigate to the `backend-api` directory and build the Rust API:

```bash
cd backend-api
cargo build --release
```

Run the backend API:

```bash
./target/release/backend-api
```

### 6. Configure the Environment

Ensure that all necessary environment variables are set for the agent, AI server, and backend API. This may include database connection strings, API keys, and other configuration settings.

### 7. Start the EDR Agent

Run the EDR agent:

```bash
./target/release/agent
```

### 8. Verify the Deployment

- Access the AI server at `http://localhost:8000`.
- Access the dashboard at the configured URL.
- Ensure that the backend API is reachable and functioning as expected.

## Troubleshooting

If you encounter issues during deployment, check the following:

- Ensure all dependencies are installed correctly.
- Review logs for any error messages.
- Verify network configurations and firewall settings.

## Conclusion

Following these steps will help you successfully deploy the RustGuard EDR project in your environment. For further assistance, refer to the documentation in the `docs` directory or reach out to the project maintainers.