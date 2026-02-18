import API_URI from '@/constants/api-uri';
import { AuthResponse } from '@/types/auth';
import axios, { AxiosError, InternalAxiosRequestConfig } from 'axios';

const apiClient = axios.create({
  baseURL: '',
  withCredentials: true, // Cookie 自動送信
  headers: {
    'Content-Type': 'application/json',
  },
});

function createAuthorizationHeader(accessToken: string): string {
  return `Bearer ${accessToken}`;
}

// アクセストークンの管理
let accessToken: string | null = null;

export const setAccessToken = (token: string | null): void => {
  accessToken = token;
};

export const getAccessToken = (): string | null => {
  return accessToken;
};

// リクエストインターセプター
apiClient.interceptors.request.use((config: InternalAxiosRequestConfig) => {
  if (accessToken) {
    config.headers.Authorization = createAuthorizationHeader(accessToken);
  }
  return config;
});

// レスポンスインターセプター
let isRefreshing = false;
let failedQueue: Array<{
  resolve: (value: unknown) => void;
  reject: (reason: unknown) => void;
  config: InternalAxiosRequestConfig;
}> = [];

const processQueue = (error: AxiosError | null): void => {
  failedQueue.forEach(({ resolve, reject, config }) => {
    if (error) {
      reject(error);
    } else {
      resolve(apiClient(config));
    }
  });
  failedQueue = [];
};

apiClient.interceptors.response.use(
  (response) => response,
  async (error: AxiosError) => {
    const originalRequest = error.config;
    if (!originalRequest) return Promise.reject(error);

    // refresh自体の401はそのまま返す
    if (
      error.response?.status === 401 &&
      !originalRequest.url?.includes(API_URI.AUTH_REFRESH)
    ) {
      if (isRefreshing) {
        // 他のリクエストがrefresh中なら待機する
        return new Promise((resolve, reject) => {
          failedQueue.push({ resolve, reject, config: originalRequest });
        });
      }
      isRefreshing = true;
      try {
        const { data } = await apiClient.post<AuthResponse>(API_URI.AUTH_REFRESH);
        setAccessToken(data.accessToken);
        processQueue(null);
        // 元リクエストをリトライ
        originalRequest.headers.Authorization = createAuthorizationHeader(
          data.accessToken
        );
        return apiClient(originalRequest);
      } catch (refreshError) {
        processQueue(refreshError as AxiosError);
        setAccessToken(null);
        // ログイン画面へリダイレクト
        if (typeof window !== 'undefined') {
          window.location.href = '/login';
        }
        return Promise.reject(refreshError);
      } finally {
        isRefreshing = false;
      }
    }

    return Promise.reject(error);
  }
);

export default apiClient;
