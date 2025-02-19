# Media Module

Media handling and processing for SocialHub.

## Features

- File upload support (images, videos, audio)
- Content type validation
- File size limits
- Metadata management
- Streaming capabilities

## Supported Formats

- Images: JPEG, PNG, GIF
- Video: MP4, MPEG
- Audio: MP3, WAV

## API Endpoints

```
POST   /media/upload
GET    /media/{id}
DELETE /media/{id}
PUT    /media/{id}/metadata
GET    /media/{id}/metadata
```

## Usage Example

```rust
use socialhub_media::MediaService;
use actix_multipart::Multipart;

async fn upload_file(payload: Multipart) {
    let media = MediaService::new();
    let result = media.upload_media(payload).await;
}
```
