import { loginSchema, registerSchema } from '../auth';

describe('loginSchema', () => {
  it('正常なデータでバリデーション成功', () => {
    const result = loginSchema.safeParse({
      email: 'test@example.com',
      password: 'password123',
    });
    expect(result.success).toBe(true);
  });

  it('不正なメールアドレスでバリデーション失敗', () => {
    const result = loginSchema.safeParse({
      email: 'invalid-email',
      password: 'password123',
    });
    expect(result.success).toBe(false);
    expect(result.error?.issues.length).toBe(1);
    expect(result.error?.issues[0].path[0]).toBe('email')
  });

  it('空のメールアドレスでバリデーション失敗', () => {
    const result = loginSchema.safeParse({
      email: '',
      password: 'password123',
    });
    expect(result.success).toBe(false);
    expect(result.error?.issues.length).toBe(1);
    expect(result.error?.issues[0].path[0]).toBe('email')
  });

  it('パスワード8文字未満でバリデーション失敗', () => {
    const result = loginSchema.safeParse({
      email: 'test@example.com',
      password: '1234567',
    });
    expect(result.success).toBe(false);
    expect(result.error?.issues.length).toBe(1);
    expect(result.error?.issues[0].path[0]).toBe('password')
  });

  it('パスワード8文字ちょうどでバリデーション成功', () => {
    const result = loginSchema.safeParse({
      email: 'test@example.com',
      password: '12345678',
    });
    expect(result.success).toBe(true);
  });
});

describe('registerSchema', () => {
  it('正常なデータでバリデーション成功', () => {
    const result = registerSchema.safeParse({
      email: 'test@example.com',
      password: 'password123',
      confirmPassword: 'password123',
    });
    expect(result.success).toBe(true);
  });

  it('パスワード不一致でバリデーション失敗', () => {
    const result = registerSchema.safeParse({
      email: 'test@example.com',
      password: 'password123',
      confirmPassword: 'password456',
    });
    expect(result.success).toBe(false);
    expect(result.error?.issues.length).toBe(1);
    expect(result.error?.issues[0].path[0]).toBe('confirmPassword')
  });

  it('不正なメールアドレスでバリデーション失敗', () => {
    const result = registerSchema.safeParse({
      email: 'not-an-email',
      password: 'password123',
      confirmPassword: 'password123',
    });
    expect(result.success).toBe(false);
    expect(result.error?.issues.length).toBe(1);
    expect(result.error?.issues[0].path[0]).toBe('email')
  });

  it('パスワード8文字未満でバリデーション失敗', () => {
    const result = registerSchema.safeParse({
      email: 'test@example.com',
      password: '1234567',
      confirmPassword: '1234567',
    });
    expect(result.success).toBe(false);
    expect(result.error?.issues.length).toBe(1);
    expect(result.error?.issues[0].path[0]).toBe('password')
  });

  it('パスワード8文字ちょうどでバリデーション成功', () => {
    const result = registerSchema.safeParse({
      email: 'test@example.com',
      password: '12345678',
      confirmPassword: '12345678',
    });
    expect(result.success).toBe(true);
  });
});
