'use client';

import { getMessage, Message } from '@/constants/message';
import { AlertDialog, Button, Flex } from '@radix-ui/themes';

interface TodoDeleteDialogProps {
  open: boolean;
  onOpenChange: (open: boolean) => void;
  onConfirm: () => Promise<void>;
  todoTitle: string;
}

export default function TodoDeleteDialog({
  open,
  onOpenChange,
  onConfirm,
  todoTitle,
}: TodoDeleteDialogProps) {
  const handleConfirm = async (): Promise<void> => {
    await onConfirm();
    onOpenChange(false);
  };

  return (
    <AlertDialog.Root
      open={open}
      onOpenChange={onOpenChange}
    >
      <AlertDialog.Content maxWidth="400px">
        <AlertDialog.Title>ToDo削除</AlertDialog.Title>
        <AlertDialog.Description>
          {getMessage(Message.I0008, todoTitle)}
        </AlertDialog.Description>
        <Flex gap="3" justify="end" mt="4">
          <AlertDialog.Cancel>
            <Button variant="soft" color="gray">キャンセル</Button>
          </AlertDialog.Cancel>
          <AlertDialog.Action>
            <Button color="red" onClick={handleConfirm}>削除</Button>
          </AlertDialog.Action>
        </Flex>
      </AlertDialog.Content>
    </AlertDialog.Root>
  );
}
