use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(
        // Streaming routes
        socialhub_streaming::handlers::stream_video,
        socialhub_streaming::handlers::stream_audio,
        socialhub_streaming::handlers::start_live,
        socialhub_streaming::handlers::stop_stream,
        
        // Auth routes
        socialhub_auth::handlers::login,
        socialhub_auth::handlers::register,
        socialhub_auth::handlers::logout,
        
        // Social routes
        socialhub_social::handlers::create_post,
        socialhub_social::handlers::get_post,
        socialhub_social::handlers::like_post,
        socialhub_social::handlers::follow_user,
        
        // Media routes
        socialhub_media::handlers::upload,  // Changed from upload_media to upload
        socialhub_media::handlers::get_media,
        socialhub_media::handlers::update_metadata
    ),
    components(
        schemas(
            // Streaming schemas
            socialhub_streaming::models::StreamType,
            socialhub_streaming::handlers::StreamRequest,
            
            // Auth schemas
            socialhub_auth::models::LoginRequest,
            socialhub_auth::models::RegisterRequest,
            socialhub_auth::models::AuthResponse,
            
            // Social schemas
            socialhub_social::models::Post,
            socialhub_social::models::Like,
            socialhub_social::handlers::CreatePostRequest,
            
            // Media schemas
            socialhub_media::models::Media,
            socialhub_media::handlers::UploadRequest,
            socialhub_media::handlers::MetadataUpdate
        )
    ),
    tags(
        (name = "streaming", description = "Live streaming endpoints"),
        (name = "auth", description = "Authentication endpoints"),
        (name = "social", description = "Social features endpoints"),
        (name = "media", description = "Media management endpoints")
    ),
    security(
        ("bearer_token" = [])
    )
)]
pub struct ApiDoc;
