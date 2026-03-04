'use client';

import Header from '@/components/Header';
import TodoDeleteDialog from '@/components/TodoDeleteDialog';
import TodoFilters from '@/components/TodoFilters';
import TodoFormDialog from '@/components/TodoFormDialog';
import TodoItem from '@/components/TodoItem';
import API_URI from '@/constants/api-uri';
import { getMessage, Message } from '@/constants/message';
import { useAuth } from '@/contexts/AuthContext';
import { useToast } from '@/contexts/ToastContext';
import apiClient from '@/lib/api-client';
import { TodoFormOutput } from '@/schemas/todo';
import {
  TodoSortValues,
  TodoPriority,
  TodoStatus,
  OrderValues,
  Todo,
  TodoListResponse,
  CreateTodoRequest,
  UpdateTodoRequest,
  UpdateTodoStatusRequest,
} from '@/types/todo';
import { Container, Text, Box, Flex, Button } from '@radix-ui/themes';
import { useCallback, useEffect, useState } from 'react';

export default function Home() {
  const { isLoading } = useAuth();
  const { showToast } = useToast();

  // フィルタ・ソート
  const [status, setStatus] = useState<TodoStatus | '*'>('*');
  const [priority, setPriority] = useState<TodoPriority | '*'>('*');
  const [sort, setSort] = useState<TodoSortValues>('createdAt');
  const [order, setOrder] = useState<OrderValues>('desc');
  const [page, setPage] = useState(1);

  // データ
  const [todos, setTodos] = useState<Todo[]>([]);
  const [total, setTotal] = useState(0);
  const [perPage] = useState(20);

  // ダイアログ
  const [formOpen, setFormOpen] = useState(false);
  const [deleteOpen, setDeleteOpen] = useState(false);
  const [editingTodo, setEditingTodo] = useState<Todo | null>(null);
  const [deletingTodo, setDeletingTodo] = useState<Todo | null>(null);

  // 一覧取得
  const fetchTodos = useCallback(async () => {
    const params: Record<string, string | number> = {
      sort,
      order,
      page,
      perPage,
    };
    if (status !== '*') params.status = status;
    if (priority !== '*') params.priority = priority;

    try {
      const { data } = await apiClient.get<TodoListResponse>(API_URI.TODOS, {
        params,
      });
      setTodos(data.items);
      setTotal(data.total);
    } catch {
      showToast({
        type: 'error',
        title: getMessage(Message.E0005, 'データ取得'),
      });
    }
  }, [status, priority, sort, order, page, perPage, showToast]);

  useEffect(() => {
    if (!isLoading) {
      fetchTodos();
    }
  }, [isLoading, fetchTodos]);

  // 作成
  const handleCreate = async (values: TodoFormOutput) => {
    try {
      await apiClient.post(API_URI.TODOS, {
        title: values.title,
        description: values.description || null,
        dueDate: values.dueDate || null,
        priority: values.priority,
      } as CreateTodoRequest);
      await fetchTodos();
      showToast({ type: 'success', title: getMessage(Message.I0010, 'ToDo') });
    } catch {
      showToast({
        type: 'error',
        title: getMessage(Message.E0005, 'ToDo作成'),
      });
    }
  };

  // 更新
  const handleUpdate = async (values: TodoFormOutput) => {
    if (!editingTodo) return;
    try {
      await apiClient.put(API_URI.TODO_BY_ID(editingTodo.id), {
        title: values.title,
        description: values.description || null,
        dueDate: values.dueDate || null,
        status: values.status,
        priority: values.priority,
      } as UpdateTodoRequest);
      setEditingTodo(null);
      await fetchTodos();
      showToast({ type: 'success', title: getMessage(Message.I0011, 'ToDo') });
    } catch {
      showToast({
        type: 'error',
        title: getMessage(Message.E0005, 'ToDo更新'),
      });
    }
  };

  // 削除
  const handleDelete = async () => {
    if (!deletingTodo) return;
    try {
      await apiClient.delete(API_URI.TODO_BY_ID(deletingTodo.id));
      setDeletingTodo(null);
      await fetchTodos();
      showToast({ type: 'success', title: getMessage(Message.I0012, 'ToDo') });
    } catch {
      showToast({
        type: 'error',
        title: getMessage(Message.E0005, 'ToDo削除'),
      });
    }
  };

  // ステータス変更
  const handleStatusChange = async (id: string, newStatus: TodoStatus) => {
    try {
      await apiClient.patch(API_URI.TODO_STATUS(id), {
        status: newStatus,
      } as UpdateTodoStatusRequest);
      await fetchTodos();
    } catch {
      showToast({
        type: 'error',
        title: getMessage(Message.E0005, 'ステータス変更'),
      });
    }
  };

  // フィルタイベント
  const handleFilterStatusChange = (value: TodoStatus | '*') => {
    setStatus(value);
    setPage(1);
  };
  const handleFilterPriorityChange = (value: TodoPriority | '*') => {
    setPriority(value);
    setPage(1);
  };
  const handleFilterSortChange = (value: TodoSortValues) => {
    setSort(value);
    setPage(1);
  };
  const handleFilterOrderChange = (value: OrderValues) => {
    setOrder(value);
    setPage(1);
  };

  // イベント
  const openCreate = () => {
    setEditingTodo(null);
    setFormOpen(true);
  };
  const openEdit = (todo: Todo) => {
    setEditingTodo(todo);
    setFormOpen(true);
  };
  const openDelete = (todo: Todo) => {
    setDeletingTodo(todo);
    setDeleteOpen(true);
  };

  const totalPages = Math.ceil(total / perPage);

  if (isLoading) {
    return (
      <Container size="3" py="9">
        <Text>読み込み中...</Text>
      </Container>
    );
  }

  return (
    <Flex direction="column" style={{ height: '100vh', overflow: 'hidden' }}>
      <Header />
      <Box style={{ flex: 1, overflow: 'hidden', width: '100%' }}>
        <Flex
          direction="column"
          mx="auto"
          px="4"
          py="6"
          style={{
            maxWidth: 'var(--container-3)',
            height: '100%',
            overflow: 'hidden',
          }}
        >
          <TodoFilters
            status={status}
            priority={priority}
            sort={sort}
            order={order}
            onStatusChange={handleFilterStatusChange}
            onPriorityChange={handleFilterPriorityChange}
            onSortChange={handleFilterSortChange}
            onOrderChange={handleFilterOrderChange}
            onCreateClick={openCreate}
          />

          {todos.length === 0 ? (
            <Box py="9">
              <Text align="center" color="gray" size="3">
                {getMessage(Message.I0009)}
              </Text>
            </Box>
          ) : (
            <>
              <Box pr="2" style={{ flex: 1, overflowY: 'auto'}}>
                {todos.map((todo) => (
                  <TodoItem
                    key={todo.id}
                    todo={todo}
                    onEdit={openEdit}
                    onDelete={openDelete}
                    onStatusChange={handleStatusChange}
                  />
                ))}
              </Box>

              {/* ページネーション */}
              {totalPages > 1 && (
                <Flex justify="center" gap="2" mt="4">
                  <Button
                    variant="soft"
                    size={{ initial: '1', sm: '2' }}
                    disabled={page <= 1}
                    onClick={() => setPage((p) => p - 1)}
                  >
                    前頁
                  </Button>
                  <Text size="2" style={{ lineHeight: '32px' }}>
                    {page} / {totalPages}
                  </Text>
                  <Button
                    variant="soft"
                    size={{ initial: '1', sm: '2' }}
                    disabled={page >= totalPages}
                    onClick={() => setPage((p) => p + 1)}
                  >
                    次頁
                  </Button>
                </Flex>
              )}
            </>
          )}

          <TodoFormDialog
            open={formOpen}
            onOpenChange={setFormOpen}
            onSubmit={editingTodo ? handleUpdate : handleCreate}
            todo={editingTodo}
          />

          <TodoDeleteDialog
            open={deleteOpen}
            onOpenChange={setDeleteOpen}
            onConfirm={handleDelete}
            todoTitle={deletingTodo?.title || ''}
          />
        </Flex>
      </Box>
    </Flex>
  );
}
