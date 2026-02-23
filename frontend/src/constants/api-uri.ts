/**  認証APIのベースURI */
const BASE_URI_AUTH = '/api/auth';

/** ToDoAPIのベースURI */
const BASE_URI_TODOS = '/api/todos';

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

  // ToDo API
  /** ToDo一覧取得・作成 */
  TODOS: BASE_URI_TODOS,
  /** ToDo詳細取得・更新・削除 */
  TODO_BY_ID: (id: string) => `${BASE_URI_TODOS}/${id}`,
  /** ToDoステータス変更 */
  TODO_STATUS: (id: string) => `${BASE_URI_TODOS}/${id}/status`,
} as const satisfies Record<string, string | Function>;

export default API_URI;
