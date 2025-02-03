# Blazing Fast URL Shortener

A high-performance URL shortener built with Rust and Next.js, featuring clean architecture, efficient storage, modern UI, and robust security.

## Features

- ⚡ High-performance URL shortening with Rust backend
- 🎨 Modern, responsive UI built with Next.js and Tailwind CSS
- 🔐 Secure URL encryption
- 🔄 Concurrent request handling
- 💾 Persistent storage using Sled embedded database
- ✨ Clean architecture with proper separation of concerns
- 🛡️ Input validation and error handling
- 📝 Basic logging
- 🔍 Health check endpoint
- 🚀 API versioning
- 🎯 Minimal dependencies

## Architecture

The project follows a full-stack architecture with separate frontend and backend components:

### Backend Components (Rust)

- **Handlers**: HTTP request handlers
- **Services**: Business logic layer
- **Repositories**: Data access layer
- **Middleware**: Request validation and processing

### Frontend Components (Next.js)

- **Pages**: Next.js pages and routing
- **Components**: Reusable UI components
- **Styles**: Tailwind CSS styling
- **API**: Frontend-backend integration

### Directory Structure
```
├── frontend/           # Next.js frontend application
│   ├── src/           # Frontend source code
│   ├── public/        # Static assets
│   └── ...           # Frontend configuration files
├── src/               # Rust backend
│   ├── handlers/     # HTTP request handlers
│   ├── services/     # Business logic
│   ├── repositories/ # Data access layer
│   ├── middleware/   # Request middleware
│   ├── config.rs     # Configuration management
│   └── main.rs       # Application entry point
└── encryption.key    # Encryption key for URL security
```

## Backend Dependencies

Core backend dependencies:
- `actix-web`: Web framework with macros support
- `actix-cors`: CORS middleware
- `sled`: Embedded database
- `nanoid`: URL shortening
- `url`: URL validation
- `serde`: Serialization
- `tokio`: Async runtime
- `tracing`: Logging system
- `ring`: Cryptography
- `base64`: Encoding

## Frontend Dependencies

Core frontend dependencies:
- Next.js 14
- React
- Tailwind CSS
- TypeScript
- ESLint

## Getting Started

### Prerequisites
- Rust 1.70 or higher
- Node.js 18 or higher
- npm or yarn

### Backend Setup
```bash
# Clone the repository
git clone https://github.com/yourusername/blazing-fast-url-shortner.git
cd blazing-fast-url-shortner

# Build the backend
cargo build --release

# Run the backend
cargo run
```

### Frontend Setup
```bash
# Navigate to frontend directory
cd frontend

# Install dependencies
npm install

# Run development server
npm run dev

# For production build
npm run build
npm start
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