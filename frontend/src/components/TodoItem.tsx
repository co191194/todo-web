'use client';

import { Todo, TodoPriority, TodoStatus } from '@/types/todo';
import { Badge, Box, Button, Card, Flex, Select, Text } from '@radix-ui/themes';

interface TodoItemProps {
  todo: Todo;
  onEdit: (todo: Todo) => void;
  onDelete: (todo: Todo) => void;
  onStatusChange: (id: string, status: TodoStatus) => void;
}

const statusLabels: Record<TodoStatus, string> = {
  pending: '未着手',
  inProgress: '進行中',
  completed: '完了',
};

const priorityLabels: Record<TodoPriority, string> = {
  low: '低',
  medium: '中',
  high: '高',
};

const priorityColors: Record<TodoPriority, 'gray' | 'yellow' | 'red'> = {
  low: 'gray',
  medium: 'yellow',
  high: 'red',
};

export default function TodoItem({
  todo,
  onEdit,
  onDelete,
  onStatusChange,
}: TodoItemProps) {
  return (
    <Card mb="2">
      <Flex
        justify="between"
        align={{ initial: 'stretch', sm: 'start' }}
        direction={{ initial: 'column', sm: 'row' }}
        gap="3"
      >
        <Box style={{ flex: 1 }}>
          <Flex align="center" gap="2" mb="1">
            <Text weight="bold" size="3">
              {todo.title}
            </Text>
            <Badge color={priorityColors[todo.priority]} size="1">
              {priorityLabels[todo.priority]}
            </Badge>
          </Flex>
          {todo.description && (
            <Text size="2" color="gray" mb="1">
              {todo.description}
            </Text>
          )}
          <Flex gap="3" align="center">
            {todo.dueDate && (
              <Text size="1" color="gray">
                期限: {new Date(todo.dueDate).toLocaleDateString('ja-JP')}
              </Text>
            )}
            <Text size="1" color="gray">
              作成: {new Date(todo.createdAt).toLocaleDateString('ja-JP')}
            </Text>
          </Flex>
        </Box>
        <Flex
          direction={{ initial: 'row', sm: 'column' }}
          gap="2"
          align={{ initial: 'center', sm: 'end' }}
          justify={{ initial: 'between', sm: 'start' }}
        >
          <Select.Root
            value={todo.status}
            onValueChange={(value) =>
              onStatusChange(todo.id, value as TodoStatus)
            }
          >
            <Select.Trigger variant="soft" />
            <Select.Content>
              <Select.Item value="pending">{statusLabels.pending}</Select.Item>
              <Select.Item value="inProgress">
                {statusLabels.inProgress}
              </Select.Item>
              <Select.Item value="completed">
                {statusLabels.completed}
              </Select.Item>
            </Select.Content>
          </Select.Root>
          <Flex gap="2">
            <Button variant="soft" size="1" onClick={() => onEdit(todo)}>
              編集
            </Button>
            <Button
              variant="soft"
              size="1"
              color="red"
              onClick={() => onDelete(todo)}
            >
              削除
            </Button>
          </Flex>
        </Flex>
      </Flex>
    </Card>
  );
}
