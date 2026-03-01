'use client';

import { todoFormSchema, TodoFormValues } from '@/schemas/todo';
import { Todo } from '@/types/todo';
import { zodResolver } from '@hookform/resolvers/zod';
import {
  Box,
  Button,
  Dialog,
  Flex,
  Select,
  Text,
  TextArea,
  TextField,
} from '@radix-ui/themes';
import { useEffect } from 'react';
import { useForm } from 'react-hook-form';

interface TodoFormDialogProps {
  open: boolean;
  onOpenChange: (open: boolean) => void;
  onSubmit: (values: TodoFormValues) => Promise<void>;
  todo?: Todo | null;
}

const defaultValues = {
  title: '',
  description: '',
  dueDate: '',
  status: 'pending',
  priority: 'medium',
} as const satisfies TodoFormValues;

export default function TodoFormDialog({
  open,
  onOpenChange,
  onSubmit,
  todo,
}: TodoFormDialogProps) {
  const isEdit = !!todo;

  const {
    register,
    handleSubmit,
    reset,
    setValue,
    watch,
    formState: { errors, isSubmitting },
  } = useForm<TodoFormValues>({
    resolver: zodResolver(todoFormSchema),
    defaultValues,
  });

  // ダイアログが開いた時に編集モードなら既存値をセット
  useEffect(() => {
    if (open && isEdit) {
      reset({
        title: todo.title,
        description: todo.description ?? '',
        dueDate: todo.dueDate ? todo.dueDate.slice(0, 16) : '',
        status: todo.status,
        priority: todo.priority,
      });
    } else if (open) {
      reset(defaultValues);
    }
  }, [open, isEdit, todo, reset]);

  const handleFormSubmit = async (values: TodoFormValues): Promise<void> => {
    await onSubmit(values);
    onOpenChange(false);
  };

  return (
    <Dialog.Root open={open} onOpenChange={onOpenChange}>
      <Dialog.Content style={{ maxWidth: 'min(480px, 90vw)' }}>
        <Dialog.Title>{isEdit ? 'ToDo編集' : 'ToDo作成'}</Dialog.Title>
        <form onSubmit={handleSubmit(handleFormSubmit)}>
          <Flex direction="column" gap="3" mt="3">
            <Box>
              <Text as="label" size="2" weight="medium">
                タイトル *
              </Text>
              <TextField.Root {...register('title')} placeholder="やること" />
              {errors.title && (
                <Text color="red" size="1">
                  {errors.title.message}
                </Text>
              )}
            </Box>

            <Box>
              <Text as="label" size="2" weight="medium">
                説明
              </Text>
              <TextArea {...register('description')} placeholder="詳細をメモ" />
            </Box>

            <Box>
              <Text as="label" size="2" weight="medium">
                期限
              </Text>
              <TextField.Root type="datetime-local" {...register('dueDate')} />
            </Box>

            <Flex gap="3" direction={{ initial: 'column', xs: 'row' }}>
              <Box style={{ flex: 1 }}>
                <Text as="label" size="2" weight="medium">
                  優先度
                </Text>
                <Select.Root
                  value={watch('priority')}
                  onValueChange={(v) => setValue('priority', v as any)}
                >
                  <Select.Trigger style={{ width: '100%' }} />
                  <Select.Content>
                    <Select.Item value="low">低</Select.Item>
                    <Select.Item value="medium">中</Select.Item>
                    <Select.Item value="high">高</Select.Item>
                  </Select.Content>
                </Select.Root>
              </Box>
              {isEdit && (
                <Box style={{ flex: 1 }}>
                  <Text as="label" size="2" weight="medium">
                    ステータス
                  </Text>
                  <Select.Root
                    value={watch('status')}
                    onValueChange={(v) => setValue('status', v as any)}
                  >
                    <Select.Trigger style={{ width: '100%' }} />
                    <Select.Content>
                      <Select.Item value="pending">未着手</Select.Item>
                      <Select.Item value="inProgress">進行中</Select.Item>
                      <Select.Item value="completed">完了</Select.Item>
                    </Select.Content>
                  </Select.Root>
                </Box>
              )}
            </Flex>

            <Flex gap="3" justify="end" mt="2">
              <Dialog.Close>
                <Button variant="soft" color="gray">
                  キャンセル
                </Button>
              </Dialog.Close>
              <Button type="submit" disabled={isSubmitting}>
                {isSubmitting ? '保存中...' : isEdit ? '更新' : '作成'}
              </Button>
            </Flex>
          </Flex>
        </form>
      </Dialog.Content>
    </Dialog.Root>
  );
}
