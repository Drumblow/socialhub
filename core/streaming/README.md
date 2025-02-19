# Streaming Module

Live streaming capabilities for SocialHub.

## Features

- Live video streaming
- Live audio streaming
- Real-time chat during streams
- Stream recording
- Viewer statistics
- Quality adaptation

## API Endpoints

```
POST   /stream/live
POST   /stream/{id}/stop
GET    /stream/{id}/info
GET    /stream/video/{id}
GET    /stream/audio/{id}
```

## Stream Types

- Video Streaming (H.264/AVC)
- Audio Streaming (AAC)
- Screen Sharing
- Multi-bitrate streaming

## Usage Example

```rust
use socialhub_streaming::{StreamingService, StreamType};

async fn start_new_stream(user_id: i32) {
    let streaming = StreamingService::new();
    let stream = streaming.start_stream(user_id, StreamType::Video).await;
}
```

## Performance Considerations

- Buffer size: 1MB
- Max bitrate: 6000kbps
- Latency target: <3s
- Max concurrent viewers: 1000/stream

## Testing

```bash
cargo test -p socialhub-streaming
```
