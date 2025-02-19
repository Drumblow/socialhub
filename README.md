# SocialHub

A modular social media backend written in Rust.

## Features

- Authentication (JWT-based)
- Media Management
- Social Features (posts, likes, follows)
- Live Streaming Support
- Cache Management
- OpenAPI Documentation (Swagger)

## Project Structure

```
socialhub/
├── core/
│   ├── auth/       # Authentication module
│   ├── common/     # Shared utilities and components
│   ├── media/      # Media handling
│   ├── social/     # Social features
│   └── streaming/  # Streaming capabilities
├── src/
│   └── bin/        # Binary entrypoints
└── tests/          # Integration tests
```

## Quick Start

```bash
# Run the server
cargo run

# Run tests
cargo test

# View API documentation
# After starting the server, visit:
# http://localhost:8080/swagger-ui/
```

## Requirements

- Rust 1.70 or higher
- PostgreSQL 13 or higher
- Redis (optional, for caching)

## License

MIT
