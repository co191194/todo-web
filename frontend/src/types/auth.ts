// ユーザー情報
export interface User {
  id: string;
  email: string;
}

// ログインリクエスト
export interface LoginRequest {
  email: string;
  password: string;
}

// ユーザー登録リクエスト
export interface RegisterRequest {
  email: string;
  password: string;
}

// 認証レスポンス（login, refresh）
export interface AuthResponse {
  accessToken: string;
  tokenType: string;
  expiresIn: number;
}

// ユーザー登録レスポンス
export interface RegisterResponse {
  id: string;
  email: string;
  createdAt: string;
}
