# AI Server for RustGuard EDR

This directory contains the implementation of the AI server for the RustGuard EDR project. The AI server is built using FastAPI and is responsible for handling inference requests using machine learning models.

## Project Structure

- **app/**
  - **main.py**: The entry point for the FastAPI application. It sets up the server and defines the API routes.
  - **models/**: This directory contains the machine learning and deep learning model files used by the AI server.
  - **inference.py**: This file contains the logic for performing inference using the models defined in the models directory.

## Setup Instructions

1. **Clone the Repository**
   ```bash
   git clone https://github.com/yourusername/rustguard-edr.git
   cd rustguard-edr/ai-server
   ```

2. **Install Dependencies**
   Ensure you have Python 3.7 or higher installed. Then, install the required packages using pip:
   ```bash
   pip install -r requirements.txt
   ```

3. **Run the Server**
   Start the FastAPI server:
   ```bash
   uvicorn app.main:app --host 0.0.0.0 --port 8000
   ```

4. **Access the API**
   Open your browser and navigate to `http://localhost:8000/docs` to access the interactive API documentation.

## Usage

The AI server provides endpoints for model inference. You can send requests to these endpoints to get predictions based on the input data.

## Contributing

Contributions are welcome! Please submit a pull request or open an issue for any enhancements or bug fixes.

## License

This project is licensed under the MIT License. See the LICENSE file for more details.