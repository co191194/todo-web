// 認証APIのベースURI
const BASE_URI_AUTH = '/api/auth';

const API_URI = {
  // 認証API
  /** トークンのリフレッシュ */
  AUTH_REFRESH: BASE_URI_AUTH + '/refresh',
  /** ユーザー登録 */
  AUTH_REGISTER: BASE_URI_AUTH + '/register',
  /** カレントユーザー情報を取得 */
  AUTH_ME: BASE_URI_AUTH + '/me',
  /** ログイン */
  AUTH_LOGIN: BASE_URI_AUTH + '/login',
  /** ログアウト */
  AUTH_LOGOUT: BASE_URI_AUTH + '/logout',

} as const satisfies Record<string, string>;

export default API_URI;
