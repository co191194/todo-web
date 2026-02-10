
use crate::{
    config::Config,
    error::{AppError, AppResult},
    models::{auth::LoginRequest, user::User},
};
use bcrypt::{hash, verify, DEFAULT_COST};
use chrono::{Duration, Utc};
use jsonwebtoken::{Algorithm, EncodingKey, Header, encode};
use sha2::{Digest, Sha256};
use uuid::Uuid;

use crate::{
    models::auth::{AuthResponse, Claims, RegisterRequest},
    repositories::{
        token_repository::TokenRepository,
        user_repository::UserRepository,
    },
};

#[derive(Clone)]
pub struct AuthService {
    user_repo: UserRepository,
    token_repo: TokenRepository,
    config: Config,
    encoding_key: EncodingKey,
}

impl AuthService {
    pub fn new(
        user_repo: UserRepository,
        token_repo: TokenRepository,
        config: Config,
    ) -> AppResult<Self> {
        let key_data = std::fs::read(&config.jwt_private_key_path)
            .map_err(|e| AppError::Internal(format!("Failed to read private key: {}", e)))?;
        let encoding_key = EncodingKey::from_rsa_pem(&key_data)
            .map_err(|e| AppError::Internal(format!("Invalid private key: {}", e)))?;

        Ok(Self {
            user_repo,
            token_repo,
            config,
            encoding_key,
        })
    }

    /// ユーザー登録
    pub async fn register(&self, req: RegisterRequest) -> AppResult<User> {
        // メールの重複チェック
        if self.user_repo.find_by_email(&req.email).await?.is_some() {
            return Err(AppError::Conflict("Email already exists".into()));
        }

        // パスワードをハッシュ化
        let password_hash = hash(req.password, DEFAULT_COST)
            .map_err(|e| AppError::Internal(format!("Hash error: {}", e)))?;

        // DBに保存
        let user = self.user_repo.create(&req.email, &password_hash).await?;

        Ok(user)
    }

    /// ログイン処理
    pub async fn login(&self, req: LoginRequest) -> AppResult<(AuthResponse, String)> {
        let user = self
            .user_repo
            .find_by_email(&req.email)
            .await?
            .ok_or(AppError::Auth("Invalid email or password".into()))?;

        // パスワードの照合
        let valid = verify(&req.password, &user.password_hash)
            .map_err(|e| AppError::Internal(format!("Verify error: {}", e)))?;
        if !valid {
            return Err(AppError::Auth("Invalid email or password".into()));
        }

        self.generate_tokens(user.id, &user.email).await
    }

    /// リフレッシュトークンでアクセストークンを再発行（ローテーション）
    pub async fn refresh(&self, refresh_token: &str) -> AppResult<(AuthResponse, String)> {
        let token_hash = Self::hash_token(refresh_token);

        // DBから有効期限内のものを検索
        let stored = self
            .token_repo
            .find_by_token_hash(&token_hash)
            .await?
            .ok_or_else(|| AppError::Auth("Invalid or expired refresh token".into()))?;

        // 旧トークンを削除（ローテーション）
        self.token_repo.delete_by_token_hash(&token_hash).await?;

        let user = self
            .user_repo
            .find_by_id(stored.user_id)
            .await?
            .ok_or_else(|| AppError::Auth("User not found".into()))?;

        // 新しいトークンペアを発行
        self.generate_tokens(user.id, &user.email).await
    }

    /// ログアウト（リフレッシュトークンを無効化）
    pub async fn logout(&self, refresh_token: &str) -> AppResult<()> {
        let token_hash = Self::hash_token(refresh_token);
        self.token_repo.delete_by_token_hash(&token_hash).await?;
        Ok(())
    }

    /// トークンペア生成
    async fn generate_tokens(
        &self,
        user_id: Uuid,
        email: &str,
    ) -> AppResult<(AuthResponse, String)> {
        // Access Token (JWT RS256)
        let now = Utc::now();
        let exp = now + Duration::minutes(self.config.jwt_access_expires_in);
        let claims = Claims {
            sub: user_id,
            email: email.to_string(),
            exp: exp.timestamp() as usize,
            iat: now.timestamp() as usize,
        };

        let access_token = encode(
            &Header::new(Algorithm::RS256), 
            &claims, 
            &self.encoding_key,
        )
        .map_err(|e| AppError::Internal(format!("JWT encode error: {}", e)))?;
        
        // Refresh Token (ランダムUUID -> SHA256ハッシュにしてDBに保存)
        let refresh_token_raw = Uuid::new_v4().to_string();
        let refresh_token_hash = Self::hash_token(&refresh_token_raw);
        let refresh_expires_at = now + Duration::days(self.config.jwt_refresh_expires_in);
        
        self.token_repo
            .create(user_id, &refresh_token_hash, refresh_expires_at)
            .await?;
        
        let auth_response = AuthResponse {
            access_token,
            token_type: "Bearer".to_string(),
            expires_in: self.config.jwt_access_expires_in * 60, // 秒に変換
        };
        
        Ok((auth_response, refresh_token_raw))
    }

    /// リフレッシュトークンをSHA256でハッシュ化
    /// セキュリティの観点からDBにはハッシュ値を保存する
    fn hash_token(token: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(token.as_bytes());
        format!("{:x}", hasher.finalize())
    }
}
