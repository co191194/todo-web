import { getMessage, Message } from '@/constants/message';
import { TodoPriority, TodoStatus } from '@/types/todo';
import { DateTime } from 'luxon';
import { z } from 'zod';

export const todoFormSchema = z.object({
  title: z
    .string()
    .min(1, getMessage(Message.E0002, 'タイトル', '1'))
    .max(255, getMessage(Message.E0007, 'タイトル', '255')),
  description: z.string().optional().or(z.literal('')),
  dueDate: z
    .string()
    .optional()
    .or(z.literal(''))
    .transform((val) => {
      if (!val) return undefined;
      const dt = DateTime.fromISO(val, { zone: 'local' });
      return dt.isValid ? dt.toUTC().toISO() : undefined;
    }),
  status: z
    .enum<TodoStatus[]>(['completed', 'inProgress', 'pending'])
    .optional(),
  priority: z.enum<TodoPriority[]>(['low', 'medium', 'high']).optional(),
});

export type TodoFormInput = z.input<typeof todoFormSchema>;

export type TodoFormOutput = z.output<typeof todoFormSchema>;
