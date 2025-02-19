use crate::{models::{User, LoginRequest, AuthResponse}, error::AuthError};

pub struct AuthService;

impl AuthService {
    pub async fn login(request: &LoginRequest) -> Result<AuthResponse, AuthError> {
        // TODO: Implement login logic
        Ok(AuthResponse {
            token: "dummy_token".to_string(),
            user_id: 1,
        })
    }

    pub async fn register(request: &LoginRequest) -> Result<User, String> {
        // TODO: Implement registration logic
        Ok(User {
            id: 1,
            username: request.username.clone(),
            email: "dummy@email.com".to_string(),
            password_hash: "hash".to_string(),
        })
    }
}
