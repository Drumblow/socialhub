# Auth Module

Authentication and authorization module for SocialHub.

## Features

- JWT-based authentication
- User registration and login
- Session management
- Role-based authorization
- Password hashing with bcrypt
- MFA support
- Rate limiting
- Audit logging

## Configuration

```toml
[auth]
jwt_secret = "your-secret-key"
token_expiration = 3600  # in seconds
hash_rounds = 12        # bcrypt rounds
rate_limit = 10         # requests per minute
```

## Dependencies

```toml
[dependencies]
jsonwebtoken = "8.1"
bcrypt = "0.10"
tokio = { version = "1.0", features = ["full"] }
serde = { version = "1.0", features = ["derive"] }
```

## API Endpoints

```
POST /auth/login
POST /auth/register
POST /auth/logout
GET  /auth/verify
POST /auth/refresh
POST /auth/mfa/enable
POST /auth/mfa/verify
```

## Usage Example

```rust
use socialhub_auth::{AuthService, LoginRequest};

async fn login_user() {
    let auth = AuthService::new();
    let result = auth.login(&LoginRequest {
        username: "user@example.com".to_string(),
        password: "password123".to_string()
    }).await;
}
```

## Error Handling

```rust
pub enum AuthError {
    InvalidCredentials,
    UserNotFound,
    TokenExpired,
    RateLimitExceeded,
    MfaRequired,
}
```

## Testing

```bash
cargo test -p socialhub-auth
```

## Security Considerations

- Passwords are always hashed using bcrypt
- Tokens are signed with HS256
- Rate limiting prevents brute force
- MFA adds additional security layer
- All sensitive operations are logged
