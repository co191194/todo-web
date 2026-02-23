/**
 * Toastの種類
 */
export type ToastType = 'success' | 'error' | 'info';

/**
 * Toastのデータ定義
 */
export interface ToastItem {
  id: string;
  title: string;
  description?: string;
  type: ToastType;
}
