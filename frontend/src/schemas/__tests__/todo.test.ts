import { TodoPriority, TodoStatus } from '@/types/todo';
import { todoFormSchema } from '../todo';

describe('todoFormSchema', () => {
  it('タイトルのみで成功', () => {
    const result = todoFormSchema.safeParse({
      title: 'テストToDo',
    });
    expect(result.success).toBe(true);
  });

  it('全フィールド指定で成功', () => {
    const result = todoFormSchema.safeParse({
      title: 'テストToDo',
      description: '詳細を説明',
      dueDate: '2026-03-01T10:00',
      status: 'inProgress',
      priority: 'high',
    });
    expect(result.success).toBe(true);
  });

  it('タイトル空文字でバリデーション失敗', () => {
    const result = todoFormSchema.safeParse({
      title: '',
    });
    expect(result.success).toBe(false);
    expect(result.error?.issues.length).toBe(1);
    expect(result.error?.issues[0].path[0]).toBe('title')
  });

  it('タイトル255文字ちょうどで成功', () => {
    const result = todoFormSchema.safeParse({
      title: 'a'.repeat(255),
    });
    expect(result.success).toBe(true);
  });

  it('タイトル256文字でバリデーション失敗', () => {
    const result = todoFormSchema.safeParse({
      title: 'a'.repeat(256),
    });
    expect(result.success).toBe(false);
    expect(result.error?.issues.length).toBe(1);
    expect(result.error?.issues[0].path[0]).toBe('title')
  });

  it('description空文字で成功', () => {
    const result = todoFormSchema.safeParse({
      title: 'テスト',
      description: '',
    });
    expect(result.success).toBe(true);
  });

  it('dueDate空文字で成功', () => {
    const result = todoFormSchema.safeParse({
      title: 'テスト',
      dueDate: '',
    });
    expect(result.success).toBe(true);
  });

  it('不正なstatusでバリデーション失敗', () => {
    const result = todoFormSchema.safeParse({
      title: 'テスト',
      status: 'invalid',
    });
    expect(result.success).toBe(false);
    expect(result.error?.issues.length).toBe(1);
    expect(result.error?.issues[0].path[0]).toBe('status')
  });

  it('不正なpriorityでバリデーション失敗', () => {
    const result = todoFormSchema.safeParse({
      title: 'テスト',
      priority: 'invalid',
    });
    expect(result.success).toBe(false);
    expect(result.error?.issues.length).toBe(1);
    expect(result.error?.issues[0].path[0]).toBe('priority')
  });

  // 有効なenumの全パターン
  describe('status enum', () => {
    it.each(['pending', 'inProgress', 'completed'] as TodoStatus[])(
      '%s で成功',
      (status) => {
        const result = todoFormSchema.safeParse({ title: 'テスト', status });
        expect(result.success).toBe(true);
      }
    );
  });

  describe('priority enum', () => {
    it.each(['low', 'medium', 'high'] as TodoPriority[])(
      '%s で成功',
      (priority) => {
        const result = todoFormSchema.safeParse({ title: 'テスト', priority });
        expect(result.success).toBe(true);
      }
    );
  });
});
