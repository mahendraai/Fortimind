# backend-api README

# RustGuard EDR Backend API

This is the backend API for the RustGuard EDR project, implemented in Rust. The API serves as the communication layer between the EDR agent, AI server, and the admin dashboard. It provides endpoints for data retrieval, processing, and management of the EDR system.

## Project Structure

- `src/main.rs`: Entry point for the backend API, initializing the server and defining routes.
- `src/routes/`: Contains route definitions for the API.
- `src/models/`: Holds data models used in the API.
- `src/auth/`: Contains authentication logic for the API.
- `src/db.rs`: Manages database connections and queries for the API.
- `Cargo.toml`: Configuration file for the Rust backend API project.

## Setup Instructions

1. **Clone the Repository**
   ```bash
   git clone https://github.com/yourusername/rustguard-edr.git
   cd rustguard-edr/backend-api
   ```

2. **Build the Project**
   Ensure you have Rust installed. You can build the project using Cargo:
   ```bash
   cargo build
   ```

3. **Run the API**
   After building, you can run the API with:
   ```bash
   cargo run
   ```

4. **Access the API**
   The API will be available at `http://localhost:8000` by default. You can use tools like Postman or curl to interact with the API endpoints.

## API Documentation

Refer to the `docs/api-spec.md` file for detailed API specifications, including available endpoints, request/response formats, and authentication methods.

## Contributing

Contributions are welcome! Please submit a pull request or open an issue for any enhancements or bug fixes.

## License

This project is licensed under the MIT License. See the LICENSE file for more details.