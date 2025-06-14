# API Specifications for RustGuard EDR

## Overview
This document outlines the API specifications for the RustGuard EDR project, detailing the available endpoints, request/response formats, and authentication mechanisms.

## Base URL
The base URL for the API is:
```
http://<your-server-address>/api/v1
```

## Endpoints

### 1. Process Monitoring

#### GET /processes
- **Description**: Retrieve a list of currently monitored processes.
- **Response**:
  - **200 OK**: Returns a JSON array of process objects.
  - **Example**:
    ```json
    [
      {
        "pid": 1234,
        "name": "example.exe",
        "status": "running"
      },
      ...
    ]
    ```

### 2. File Monitoring

#### GET /files
- **Description**: Retrieve a list of monitored files and their statuses.
- **Response**:
  - **200 OK**: Returns a JSON array of file objects.
  - **Example**:
    ```json
    [
      {
        "path": "/path/to/file.txt",
        "last_modified": "2023-10-01T12:00:00Z",
        "status": "monitored"
      },
      ...
    ]
    ```

### 3. Network Monitoring

#### GET /network/connections
- **Description**: Retrieve a list of active network connections.
- **Response**:
  - **200 OK**: Returns a JSON array of connection objects.
  - **Example**:
    ```json
    [
      {
        "local_address": "192.168.1.2:12345",
        "remote_address": "93.184.216.34:80",
        "status": "established"
      },
      ...
    ]
    ```

### 4. AI Inference

#### POST /ai/inference
- **Description**: Perform inference using the AI model.
- **Request**:
  - **Body**: JSON object containing the data for inference.
  - **Example**:
    ```json
    {
      "data": [0.1, 0.2, 0.3, ...]
    }
    ```
- **Response**:
  - **200 OK**: Returns the inference result.
  - **Example**:
    ```json
    {
      "prediction": "malware"
    }
    ```

## Authentication
- All endpoints require an API key for authentication.
- Include the API key in the request header:
  ```
  Authorization: Bearer <your-api-key>
  ```

## Error Handling
- The API returns standard HTTP status codes for error handling.
- **Example Error Response**:
  ```json
  {
    "error": "Unauthorized",
    "message": "Invalid API key"
  }
  ```

## Conclusion
This API specification serves as a guide for developers to interact with the RustGuard EDR system. Ensure to follow the defined formats and authentication methods for successful integration.