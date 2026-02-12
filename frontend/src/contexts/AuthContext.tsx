'use client';

import API_URI from '@/constants/api-uri';
import apiClient, { setAccessToken } from '@/lib/api-client';
import { AuthResponse, RegisterResponse, User } from '@/types/auth';
import {
  createContext,
  ReactNode,
  useCallback,
  useContext,
  useEffect,
  useState,
} from 'react';

interface AuthContextType {
  user: User | null;
  isLoading: boolean;
  isAuthenticated: boolean;
  login: (email: string, password: string) => Promise<void>;
  register: (email: string, password: string) => Promise<void>;
  logout: () => Promise<void>;
}

const AuthContext = createContext<AuthContextType | undefined>(undefined);

export function AuthProvider({ children }: { children: ReactNode }) {
  const [user, setUser] = useState<User | null>(null);
  const [isLoading, setIsLoading] = useState(true);

  const isAuthenticated = user !== null;

  // 初期化 (refreshを試行してセッションを復元)
  useEffect(() => {
    const initAuth = async () => {
      try {
        const { data } = await apiClient.post<AuthResponse>(
          API_URI.AUTH_REFRESH
        );
        setAccessToken(data.accessToken);
        const meResponse = await apiClient.get<User>(API_URI.AUTH_ME);
        setUser(meResponse.data);
      } catch {
        // リフレッシュトークンなし or 認証切れ -> 未承認状態
        setAccessToken(null);
        setUser(null);
      } finally {
        setIsLoading(false);
      }
    };
    initAuth();
  }, []);

  // ログイン
  const login = useCallback(async (email: string, password: string) => {
    const { data } = await apiClient.post<AuthResponse>(API_URI.AUTH_LOGIN, {
      email,
      password,
    });
    setAccessToken(data.accessToken);
    const meResponse = await apiClient.get<User>(API_URI.AUTH_ME);
    setUser(meResponse.data)
  }, []);

  // ユーザー登録 -> 自動ログイン
  const register = useCallback(
    async (email: string, password: string) => {
      await apiClient.post<RegisterResponse>(API_URI.AUTH_REGISTER, {
        email,
        password,
      });
      // 登録成功後に自動ログイン
      await login(email, password);
    },
    [login]
  );

  // ログアウト
  const logout = useCallback(async () => {
    try {
      await apiClient.post(API_URI.AUTH_LOGOUT);
    } catch {
      // サーバーエラーでもクライアント側はクリアするためエラー後も処理継続
    }
    setAccessToken(null);
    setUser(null);
  }, []);

  return (
    <AuthContext.Provider
      value={{ user, isLoading, isAuthenticated, login, register, logout }}
    >
      {children}
    </AuthContext.Provider>
  );
}

export function useAuth(): AuthContextType {
  const context = useContext(AuthContext);
  if (!context) {
    throw new Error('useAuth must be used within an AuthProvider');
  }
  return context;
}
