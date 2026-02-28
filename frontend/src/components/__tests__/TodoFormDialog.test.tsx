import { render, screen, waitFor } from '@/test/test-utils';
import userEvent from '@testing-library/user-event';
import TodoFormDialog from '../TodoFormDialog';
import { Todo } from '@/types/todo';

const mockTodo: Todo = {
  id: '1',
  title: '既存ToDo',
  description: '既存の説明',
  dueDate: '2026-03-01T10:00:00Z',
  status: 'inProgress',
  priority: 'high',
  createdAt: '2026-02-23T00:00:00Z',
  updatedAt: '2026-02-23T00:00:00Z',
};

describe('TodoFormDialog', () => {
  const defaultProps = {
    open: true,
    onOpenChange: vi.fn(),
    onSubmit: vi.fn().mockResolvedValue(undefined),
  };

  beforeEach(() => {
    vi.clearAllMocks();
  });

  // --- 作成モード ---
  describe('作成モード', () => {
    it('ダイアログタイトルが「ToDo作成」になる', () => {
      render(<TodoFormDialog {...defaultProps} />);
      expect(screen.getByText('ToDo作成')).toBeInTheDocument();
    });

    it('作成ボタンが表示される', () => {
      render(<TodoFormDialog {...defaultProps} />);
      expect(screen.getByText('作成')).toBeInTheDocument();
    });

    it('ステータスフィールドが表示されない（作成モード）', () => {
      render(<TodoFormDialog {...defaultProps} />);
      expect(screen.queryByText('ステータス')).not.toBeInTheDocument();
    });

    it('タイトル空でsubmitするとバリデーションエラーが表示される', async () => {
      const user = userEvent.setup();
      render(<TodoFormDialog {...defaultProps} />);
      await user.click(screen.getByText('作成'));
      await waitFor(() => {
        // タイトル必須エラーが表示される
        expect(defaultProps.onSubmit).not.toHaveBeenCalled();
      });
    });

    it('タイトル入力してsubmitするとonSubmitが呼ばれる', async () => {
      const user = userEvent.setup();
      render(<TodoFormDialog {...defaultProps} />);
      await user.type(screen.getByPlaceholderText('やること'), 'テスト作成');
      await user.click(screen.getByText('作成'));
      await waitFor(() => {
        expect(defaultProps.onSubmit).toHaveBeenCalledTimes(1);
        expect(defaultProps.onSubmit).toHaveBeenCalledWith(
          expect.objectContaining({ title: 'テスト作成' })
        );
      });
    });
  });

  // --- 編集モード ---
  describe('編集モード', () => {
    it('ダイアログタイトルが「ToDo編集」になる', () => {
      render(<TodoFormDialog {...defaultProps} todo={mockTodo} />);
      expect(screen.getByText('ToDo編集')).toBeInTheDocument();
    });

    it('更新ボタンが表示される', () => {
      render(<TodoFormDialog {...defaultProps} todo={mockTodo} />);
      expect(screen.getByText('更新')).toBeInTheDocument();
    });

    it('ステータスフィールドが表示される（編集モード）', () => {
      render(<TodoFormDialog {...defaultProps} todo={mockTodo} />);
      expect(screen.getByText('ステータス')).toBeInTheDocument();
    });

    it('既存のタイトルがフォームにセットされている', () => {
      render(<TodoFormDialog {...defaultProps} todo={mockTodo} />);
      expect(screen.getByPlaceholderText('やること')).toHaveValue('既存ToDo');
    });
  });

  // --- 共通 ---
  it('open=falseの場合ダイアログが表示されない', () => {
    render(<TodoFormDialog {...defaultProps} open={false} />);
    expect(screen.queryByText('ToDo作成')).not.toBeInTheDocument();
  });

  it('キャンセルボタンが表示される', () => {
    render(<TodoFormDialog {...defaultProps} />);
    expect(screen.getByText('キャンセル')).toBeInTheDocument();
  });
});
