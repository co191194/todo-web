'use client';

import {
  OrderValues,
  TodoPriority,
  TodoSortValues,
  TodoStatus,
} from '@/types/todo';
import { Box, Button, Flex, Select, Text } from '@radix-ui/themes';

interface TodoFiltersProps {
  status: TodoStatus | '*';
  priority: TodoPriority | '*';
  sort: string;
  order: OrderValues;
  onStatusChange: (value: TodoStatus | '*') => void;
  onPriorityChange: (value: TodoPriority | '*') => void;
  onSortChange: (value: TodoSortValues) => void;
  onOrderChange: (value: OrderValues) => void;
  onCreateClick: () => void;
}

export default function TodoFilters({
  status,
  priority,
  sort,
  order,
  onStatusChange,
  onPriorityChange,
  onSortChange,
  onOrderChange,
  onCreateClick,
}: TodoFiltersProps) {
  return (
    <Flex direction="column" gap="3" mb="4">
      <Flex justify="between" align="center">
        <Text size="5" weight="bold">
          ToDo一覧
        </Text>
        <Button onClick={onCreateClick}>新規作成</Button>
      </Flex>
      <Flex gap="3" wrap="wrap" direction="row">
        <Box style={{ flex: '1 1 calc(50% - 6px)', minWidth: '140px' }}>
          <Text as="label" size="1" color="gray" style={{ display: 'block', marginBottom: '2px' }}>
            ステータス
          </Text>
          <Select.Root value={status} onValueChange={onStatusChange}>
            <Select.Trigger placeholder="すべて" style={{ width: '100%' }} />
            <Select.Content>
              <Select.Item value="*">すべて</Select.Item>
              <Select.Item value="pending">未着手</Select.Item>
              <Select.Item value="inProgress">進行中</Select.Item>
              <Select.Item value="completed">完了</Select.Item>
            </Select.Content>
          </Select.Root>
        </Box>
        <Box style={{ flex: '1 1 calc(50% - 6px)', minWidth: '140px' }}>
          <Text as="label" size="1" color="gray" style={{ display: 'block', marginBottom: '2px' }}>
            優先度
          </Text>
          <Select.Root value={priority} onValueChange={onPriorityChange}>
            <Select.Trigger placeholder="すべて" style={{ width: '100%' }} />
            <Select.Content>
              <Select.Item value="*">すべて</Select.Item>
              <Select.Item value="low">低</Select.Item>
              <Select.Item value="medium">中</Select.Item>
              <Select.Item value="high">高</Select.Item>
            </Select.Content>
          </Select.Root>
        </Box>
        <Box style={{ flex: '1 1 calc(50% - 6px)', minWidth: '140px' }}>
          <Text as="label" size="1" color="gray" style={{ display: 'block', marginBottom: '2px' }}>
            ソート
          </Text>
          <Select.Root value={sort} onValueChange={onSortChange}>
            <Select.Trigger style={{ width: '100%' }} />
            <Select.Content>
              <Select.Item value="createdAt">作成日</Select.Item>
              <Select.Item value="dueDate">期限日</Select.Item>
              <Select.Item value="priority">優先度</Select.Item>
            </Select.Content>
          </Select.Root>
        </Box>
        <Box style={{ flex: '1 1 calc(50% - 6px)', minWidth: '140px' }}>
          <Text as="label" size="1" color="gray" style={{ display: 'block', marginBottom: '2px' }}>
            順序
          </Text>
          <Select.Root value={order} onValueChange={onOrderChange}>
            <Select.Trigger style={{ width: '100%' }} />
            <Select.Content>
              <Select.Item value="desc">降順</Select.Item>
              <Select.Item value="asc">昇順</Select.Item>
            </Select.Content>
          </Select.Root>
        </Box>
      </Flex>
    </Flex>
  );
}
