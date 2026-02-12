export type TodoStatus = 'pending' | 'inProgress' | 'completed';
export type TodoPriority = 'low' | 'medium' | 'high';

// ToDo取得レスポンス
export interface Todo {
  id: string;
  title: string;
  description: string | null;
  dueDate: string | null;
  status: TodoStatus;
  priority: TodoPriority;
  createdAt: string;
  updatedAt: string;
}

// ToDo一覧レスポンス
export interface TodoListResponse {
  items: Todo[];
  total: number;
  page: number;
  perPage: number;
}

// ToDo作成リクエスト
export interface CreateTodoRequest {
  title: string;
  description?: string | null;
  dueDate?: string | null;
  status?: TodoStatus;
  priority?: TodoPriority;
}

// ToDo更新リクエスト
export interface UpdateTodoRequest {
  title?: string;
  description?: string | null;
  dueDate?: string | null;
  status?: TodoStatus;
  priority?: TodoPriority;
}

// ステータス変更リクエスト
export interface UpdateTodoStatusRequest {
  status: TodoStatus;
}

// クエリパラメータ
export interface TodoQuery {
  status?: TodoStatus;
  priority?: TodoPriority;
  dueBefore?: string;
  dueAfter?: string;
  sort?: string;
  order?: 'asc' | 'desc';
  page?: number;
  perPage?: number;
}
