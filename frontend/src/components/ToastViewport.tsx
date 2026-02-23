'use client';

import * as RadixToast from '@radix-ui/react-toast';
import { Cross2Icon } from '@radix-ui/react-icons';
import { ToastItem, ToastType } from '@/types/toast';

// type に応じたスタイルのマップ
const typeStyles: Record<ToastType, string> = {
  success: 'border-l-4 border-l-green-600 bg-white',
  error: 'border-l-4 border-l-red-600 bg-white',
  info: 'border-l-4 border-l-blue-600 bg-white',
};

// type に応じたデフォルトduration
const typeDuration: Record<ToastType, number> = {
  success: 4000,
  error: 8000,
  info: 5000,
};

interface ToastViewportProps {
  toast: ToastItem;
  onClose: () => void;
}

export default function ToastViewport({ toast, onClose }: ToastViewportProps) {
  return (
    <RadixToast.Root
      duration={typeDuration[toast.type]}
      onOpenChange={(open) => {
        if (!open) onClose();
      }}
      className={`toast-root ${typeStyles[toast.type]}`}
    >
      <div className="flex items-start justify-between gap-2">
        <div>
          <RadixToast.Title className="text-sm font-medium text-gray-900">
            {toast.title}
          </RadixToast.Title>
          {toast.description && (
            <RadixToast.Description className="mt-1 text-xs text-gray-500">
              {toast.description}
            </RadixToast.Description>
          )}
        </div>
        <RadixToast.Close
          className="text-gray-400 hover:text-gray-600"
          aria-label="閉じる"
        >
          <Cross2Icon />
        </RadixToast.Close>
      </div>
    </RadixToast.Root>
  );
}
