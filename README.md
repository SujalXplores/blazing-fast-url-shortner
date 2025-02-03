# Blazing Fast URL Shortener

A high-performance URL shortener built with Rust, featuring clean architecture, efficient storage, and robust error handling.

## Features

- ⚡ High-performance URL shortening
- 🔄 Concurrent request handling
- 💾 Persistent storage using Sled embedded database
- ✨ Clean architecture with proper separation of concerns
- 🛡️ Input validation and error handling
- 📝 Basic logging
- 🔍 Health check endpoint
- 🚀 API versioning
- 🎯 Minimal dependencies

## Architecture

The project follows clean architecture principles with the following components:

### Core Components

- **Handlers**: HTTP request handlers
- **Services**: Business logic layer
- **Repositories**: Data access layer
- **Middleware**: Request validation and processing

### Directory Structure
```
src/
├── handlers/       # HTTP request handlers
├── services/      # Business logic
├── repositories/  # Data access layer
├── middleware/    # Request middleware
├── config.rs      # Configuration management
└── main.rs        # Application entry point
```

## API Endpoints

### Create Short URL
```http
POST /api/v1/shorten
Content-Type: application/json

{
    "url": "https://example.com"
}
```

Response:
```json
{
    "original_url": "https://example.com",
    "short_url": "http://localhost:8080/abc123",
    "short_code": "abc123"
}
```

### Access Shortened URL
```http
GET /{short_code}
```
Response: 302 Redirect to original URL

### Health Check
```http
GET /api/v1/health
```

Response:
```json
{
    "status": "ok"
}
```

## Configuration

The application uses environment variables for configuration:

- `SERVER_HOST`: Server host (default: "127.0.0.1")
- `SERVER_PORT`: Server port (default: "8080")
- `SERVER_WORKERS`: Number of worker threads (default: 4)
- `STORAGE_PATH`: Database path (default: "url_db")
- `STORAGE_CACHE_SIZE_MB`: Cache size in MB (default: 64)
- `STORAGE_FLUSH_INTERVAL_MS`: Storage flush interval (default: 1000)
- `RUST_LOG`: Log level (default: "info")

## Performance Features

- Efficient storage using Sled embedded database
- High-throughput mode for better performance
- 64MB default cache for faster access
- Fixed worker thread pool (4 workers by default)
- Periodic data flushing for durability
- Path normalization for consistent handling

## Dependencies

Minimal set of core dependencies:
- `actix-web`: Web framework
- `sled`: Embedded database
- `nanoid`: URL shortening
- `url`: URL validation
- `serde`: Serialization
- `tokio`: Async runtime
- `tracing`: Basic logging

## Getting Started

### Prerequisites
- Rust 1.70 or higher
- Cargo package manager

### Installation
```bash
# Clone the repository
git clone https://github.com/yourusername/blazing-fast-url-shortner.git
cd blazing-fast-url-shortner

# Build the project
cargo build --release
```

### Running
```bash
cargo run
```

### Usage Examples

#### PowerShell
```powershell
# Create a short URL
$body = @{url = 'https://github.com'} | ConvertTo-Json
Invoke-WebRequest -Method POST -Uri 'http://localhost:8080/api/v1/shorten' -ContentType 'application/json' -Body $body | Select-Object -Expand Content | ConvertFrom-Json

# Access shortened URL
Invoke-WebRequest -Method GET -Uri "http://localhost:8080/{short_code}" -MaximumRedirection 0

# Check health
Invoke-WebRequest -Method GET -Uri "http://localhost:8080/api/v1/health" | Select-Object -Expand Content | ConvertFrom-Json
```

#### cURL
```bash
# Create a short URL
curl -X POST -H "Content-Type: application/json" -d '{"url":"https://github.com"}' http://localhost:8080/api/v1/shorten

# Access shortened URL
curl -I http://localhost:8080/{short_code}

# Check health
curl http://localhost:8080/api/v1/health
```

## Error Handling

The application provides clear error responses:
- Invalid URL format
- URL not found
- Storage errors
- Internal server errors

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the LICENSE file for details. 