import { getMessage, Message } from '@/constants/message';
import { z } from 'zod';

const MSG_ERROR_FORMAT_EMAIL = getMessage(Message.E0001, 'メールアドレス');
const MSG_ERROR_MIN_PASSWORD = getMessage(Message.E0002, 'パスワード', '8');

export const loginSchema = z.object({
  email: z.email(MSG_ERROR_FORMAT_EMAIL),
  password: z.string().min(8, MSG_ERROR_MIN_PASSWORD),
});

export type LoginFormValues = z.infer<typeof loginSchema>;

export const registerSchema = z
  .object({
    email: z.email(MSG_ERROR_FORMAT_EMAIL),
    password: z.string().min(8, MSG_ERROR_MIN_PASSWORD),
    confirmPassword: z.string(),
  })
  .refine((data) => data.password === data.confirmPassword, {
    error: getMessage(Message.E0003, 'パスワード'),
    path: ['confirmPassword'],
  });

export type RegisterFormValues = z.infer<typeof registerSchema>;
