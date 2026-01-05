# xync-server

A fast, secure REST API server for syncing bookmarks and notes across devices. Built with Rust and Axum.

## Features

- **User Authentication** - JWT-based auth with Argon2 password hashing
- **Bookmarks** - Save, organize, and sync bookmarks with automatic preview generation
- **Notes** - Create and sync notes across devices
- **Tags & Categories** - Organize bookmarks with tags and hierarchical categories
- **API Documentation** - Interactive Swagger UI
- **Observability** - OpenTelemetry tracing, Prometheus metrics, health checks

## Quick Start

### Prerequisites

- Rust 1.75+
- Docker (for PostgreSQL)

### Setup

```bash
# Clone the repository
git clone https://github.com/yourusername/xync-server.git
cd xync-server

# Copy environment file
cp .env.example .env

# Start PostgreSQL
docker-compose up -d

# Run the server
cargo run
```

The server starts at `http://127.0.0.1:3000`

- **Swagger UI**: http://127.0.0.1:3000/swagger-ui/
- **Metrics**: http://127.0.0.1:3000/metrics
- **Health**: http://127.0.0.1:3000/health/live

## API Endpoints

All endpoints except `/auth/register` and `/auth/login` require a JWT token in the `Authorization: Bearer <token>` header.

### Authentication
| Method | Endpoint | Description |
|--------|----------|-------------|
| POST | `/api/auth/register` | Register a new user |
| POST | `/api/auth/login` | Login and get JWT token |
| GET | `/api/auth/me` | Get current user info |

### Bookmarks
| Method | Endpoint | Description |
|--------|----------|-------------|
| GET | `/api/bookmarks` | List all bookmarks |
| POST | `/api/bookmarks` | Create a bookmark |
| GET | `/api/bookmarks/{id}` | Get a bookmark |
| PUT | `/api/bookmarks/{id}` | Update a bookmark |
| DELETE | `/api/bookmarks/{id}` | Delete a bookmark |
| POST | `/api/bookmarks/preview` | Fetch URL preview |

### Notes
| Method | Endpoint | Description |
|--------|----------|-------------|
| GET | `/api/notes` | List all notes |
| POST | `/api/notes` | Create a note |
| GET | `/api/notes/{id}` | Get a note |
| PUT | `/api/notes/{id}` | Update a note |
| DELETE | `/api/notes/{id}` | Delete a note |

### Tags & Categories
| Method | Endpoint | Description |
|--------|----------|-------------|
| GET | `/api/tags` | List all tags |
| POST | `/api/tags` | Create a tag |
| GET | `/api/categories` | List all categories |
| POST | `/api/categories` | Create a category |

## Configuration

| Variable | Description | Default |
|----------|-------------|---------|
| `DATABASE_URL` | PostgreSQL connection string | Required |
| `JWT_SECRET` | Secret key for JWT signing | Required |
| `JWT_EXPIRATION_HOURS` | Token expiration time | 24 |
| `SERVER_HOST` | Server bind address | 127.0.0.1 |
| `SERVER_PORT` | Server port | 3000 |
| `OTLP_ENDPOINT` | OpenTelemetry endpoint | Optional |
| `SERVICE_NAME` | Service name for tracing | xync-server |
| `JSON_LOGS` | Enable JSON log format | false |

## Development

```bash
# Run tests
cargo test

# Run unit tests only (no Docker required)
cargo test --lib

# Check code
cargo check

# Format code
cargo fmt

# Lint
cargo clippy
```

## Observability

### Health Checks
- `GET /health/live` - Liveness probe
- `GET /health/ready` - Readiness probe (checks database)

### Metrics
Prometheus metrics available at `GET /metrics`:
- `http_requests_total` - Request count by method, path, status
- `http_request_duration_seconds` - Request latency histogram

### Distributed Tracing
Enable OpenTelemetry tracing by setting `OTLP_ENDPOINT`:

```bash
# Example with Jaeger
docker run -d --name jaeger \
  -e COLLECTOR_OTLP_ENABLED=true \
  -p 16686:16686 \
  -p 4317:4317 \
  jaegertracing/all-in-one:latest

OTLP_ENDPOINT=http://localhost:4317 cargo run
```

View traces at http://localhost:16686

## Tech Stack

- **Runtime**: [Tokio](https://tokio.rs/)
- **Web Framework**: [Axum](https://github.com/tokio-rs/axum)
- **Database**: PostgreSQL with [SQLx](https://github.com/launchbadge/sqlx)
- **Auth**: JWT with [jsonwebtoken](https://github.com/Keats/jsonwebtoken)
- **Docs**: [utoipa](https://github.com/juhaku/utoipa) + Swagger UI
- **Tracing**: [OpenTelemetry](https://opentelemetry.io/)
- **Metrics**: [Prometheus](https://prometheus.io/)

## License

MIT
