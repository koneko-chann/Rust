use jwt_simple::{prelude::*, reexports::anyhow::anyhow};
use std::{str::FromStr, sync::LazyLock};

static JWT_SECRET: LazyLock<String> = LazyLock::new(|| {
    std::env::var("JWT_SECRET").unwrap_or_else(|_| "default_secret_key".to_string())
});
pub fn create_jwt_token<T: ToString>(unique: T) -> String {
    println!("Using JWT_SECRET: {}", JWT_SECRET.as_str());
    let key = HS256Key::from_bytes(JWT_SECRET.as_bytes());
    let claims = Claims::create(Duration::from_days(7)).with_subject(unique.to_string());
    let token = key
        .authenticate(claims)
        .expect("Failed to create JWT token");
    token
}
pub fn verify_jwt_token<T: FromStr>(token: &str) -> Result<T, jwt_simple::Error> {
    let key = HS256Key::from_bytes(JWT_SECRET.as_bytes());
    let claims: JWTClaims<NoCustomClaims> = key.verify_token(token, None)?;
    let user_id_str = claims
        .subject
        .ok_or(anyhow!("Missing subject in token claims"))?;
    let user_id = user_id_str
        .parse::<T>()
        .map_err(|_| anyhow!("Invalid user ID in token claims"))?;
    Ok(user_id)
}
pub fn hash_password(password: &str) -> Result<String, bcrypt::BcryptError> {
    let hashed = bcrypt::hash(password, bcrypt::DEFAULT_COST)?;
    Ok(hashed)
}
pub fn verify_password(password: &str, hashed: &str) -> Result<bool, bcrypt::BcryptError> {
    let is_valid = bcrypt::verify(password, hashed)?;
    Ok(is_valid)
}
//test functions
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_jwt_token_creation_and_verification() {
        let user_id = "jabsdibasidub";
        let token = create_jwt_token(user_id);
        println!("Generated Token: {}", token);
        let verified_user_id =
            verify_jwt_token::<String>(&token).expect("Token verification failed");
        assert_eq!(verified_user_id.to_string(), user_id);
    }
    #[test]
    fn test_password_hashing_and_verification() {
        let password = "my_secure_password";
        let hashed = hash_password(password).expect("Password hashing failed");
        let is_valid = verify_password(password, &hashed).expect("Password verification failed");
        assert!(is_valid, "Password should be valid");
        let is_invalid =
            verify_password("wrong_password", &hashed).expect("Password verification failed");
        assert!(!is_invalid, "Password should be invalid");
    }
}
