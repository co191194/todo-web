import { render, screen } from '@/test/test-utils';
import userEvent from '@testing-library/user-event';
import TodoItem from '../TodoItem';
import { Todo } from '@/types/todo';

// モック
const mockTodo: Todo = {
  id: '1',
  title: 'テストToDo',
  description: '説明文です',
  dueDate: '2026-03-01T00:00:00Z',
  status: 'pending',
  priority: 'high',
  createdAt: '2026-02-23T00:00:00Z',
  updatedAt: '2026-02-23T00:00:00Z',
};

// テストケース
describe('TodoItem', () => {
  const defaultProps = {
    todo: mockTodo,
    onEdit: vi.fn(),
    onDelete: vi.fn(),
    onStatusChange: vi.fn(),
  };

  beforeEach(() => {
    vi.clearAllMocks();
  });

  it('タイトルが表示される', () => {
    render(<TodoItem {...defaultProps} />);
    expect(screen.getByText('テストToDo')).toBeInTheDocument();
  });

  it('説明が表示される', () => {
    render(<TodoItem {...defaultProps} />);
    expect(screen.getByText('説明文です')).toBeInTheDocument();
  });

  it('説明がnullの場合は表示されない', () => {
    const todo = { ...mockTodo, description: null };
    render(<TodoItem {...defaultProps} todo={todo} />);
    expect(screen.queryByText('説明文です')).not.toBeInTheDocument();
  });

  it('優先度バッジが表示される', () => {
    render(<TodoItem {...defaultProps} />);
    expect(screen.getByText('高')).toBeInTheDocument();
  });

  it('期限日が日本語フォーマットで表示される', () => {
    render(<TodoItem {...defaultProps} />);
    expect(screen.getByText(/期限:/)).toBeInTheDocument();
  });

  it('期限日がnullの場合は表示されない', () => {
    const todo = { ...mockTodo, dueDate: null };
    render(<TodoItem {...defaultProps} todo={todo} />);
    expect(screen.queryByText(/期限:/)).not.toBeInTheDocument();
  });

  it('編集ボタンクリックでonEditが呼ばれる', async () => {
    const user = userEvent.setup();
    render(<TodoItem {...defaultProps} />);
    await user.click(screen.getByText('編集'));
    expect(defaultProps.onEdit).toHaveBeenCalledWith(mockTodo);
  });

  it('削除ボタンクリックでonDeleteが呼ばれる', async () => {
    const user = userEvent.setup();
    render(<TodoItem {...defaultProps} />);
    await user.click(screen.getByText('削除'));
    expect(defaultProps.onDelete).toHaveBeenCalledWith(mockTodo);
  });
});
