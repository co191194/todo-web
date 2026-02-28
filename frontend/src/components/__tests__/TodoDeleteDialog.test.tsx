import { render, screen } from '@/test/test-utils';
import userEvent from '@testing-library/user-event';
import TodoDeleteDialog from '../TodoDeleteDialog';

// テストケース
describe('TodoDeleteDialog', () => {
  const defaultProps = {
    open: true,
    onOpenChange: vi.fn(),
    onConfirm: vi.fn().mockResolvedValue(undefined),
    todoTitle: '買い物リスト',
  };

  beforeEach(() => {
    vi.clearAllMocks();
  });

  it('ダイアログタイトルが表示される', () => {
    render(<TodoDeleteDialog {...defaultProps} />);
    expect(screen.getByText('ToDo削除')).toBeInTheDocument();
  });

  it('削除対象のToDo名が表示される', () => {
    render(<TodoDeleteDialog {...defaultProps} />);
    expect(screen.getByText(/買い物リスト/)).toBeInTheDocument();
  });

  it('削除ボタンクリックでonConfirmが呼ばれる', async () => {
    const user = userEvent.setup();
    render(<TodoDeleteDialog {...defaultProps} />);
    await user.click(screen.getByText('削除'));
    expect(defaultProps.onConfirm).toHaveBeenCalledTimes(1);
  });

  it('open=falseの場合ダイアログが表示されない', () => {
    render(<TodoDeleteDialog {...defaultProps} open={false} />);
    expect(screen.queryByText('ToDo削除')).not.toBeInTheDocument();
  });
});
