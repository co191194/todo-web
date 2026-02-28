import { render, screen } from '@/test/test-utils';
import userEvent from '@testing-library/user-event';
import TodoFilters from '../TodoFilters';

describe('TodoFilters', () => {
  const defaultProps = {
    status: '*' as const,
    priority: '*' as const,
    sort: 'createdAt',
    order: 'desc' as const,
    onStatusChange: vi.fn(),
    onPriorityChange: vi.fn(),
    onSortChange: vi.fn(),
    onOrderChange: vi.fn(),
    onCreateClick: vi.fn(),
  };

  beforeEach(() => {
    vi.clearAllMocks();
  });

  it('ToDo一覧の見出しが表示される', () => {
    render(<TodoFilters {...defaultProps} />);
    expect(screen.getByText('ToDo一覧')).toBeInTheDocument();
  });

  it('新規作成ボタンが表示される', () => {
    render(<TodoFilters {...defaultProps} />);
    expect(screen.getByText('新規作成')).toBeInTheDocument();
  });

  it('新規作成ボタンクリックでonCreateClickが呼ばれる', async () => {
    const user = userEvent.setup();
    render(<TodoFilters {...defaultProps} />);
    await user.click(screen.getByText('新規作成'));
    expect(defaultProps.onCreateClick).toHaveBeenCalledTimes(1);
  });

  it('ステータスラベルが表示される', () => {
    render(<TodoFilters {...defaultProps} />);
    expect(screen.getByText('ステータス')).toBeInTheDocument();
  });

  it('優先度ラベルが表示される', () => {
    render(<TodoFilters {...defaultProps} />);
    expect(screen.getByText('優先度')).toBeInTheDocument();
  });

  it('ソートラベルが表示される', () => {
    render(<TodoFilters {...defaultProps} />);
    expect(screen.getByText('ソート')).toBeInTheDocument();
  });

  it('順序ラベルが表示される', () => {
    render(<TodoFilters {...defaultProps} />);
    expect(screen.getByText('順序')).toBeInTheDocument();
  });
});
