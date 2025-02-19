# Social Module

Core social networking features for SocialHub.

## Features

- Post creation and management
- Like/Unlike functionality
- Follow/Unfollow users
- Comment system
- Feed generation
- Content moderation

## API Endpoints

```
POST   /social/posts
GET    /social/posts/{id}
DELETE /social/posts/{id}
POST   /social/posts/{id}/like
POST   /social/users/{id}/follow
GET    /social/feed
```

## Usage Example

```rust
use socialhub_social::{SocialService, Post};

async fn create_new_post(user_id: i32, content: String) {
    let social = SocialService::new();
    let post = social.create_post(content, user_id).await;
}
```

## Architecture

```
social/
├── models/
│   ├── post.rs
│   ├── like.rs
│   └── follow.rs
├── handlers/
├── service.rs
└── error.rs
```

## Testing

```bash
cargo test -p socialhub-social
```
