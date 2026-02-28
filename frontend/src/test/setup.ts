import '@testing-library/jest-dom/vitest';

// jsdomにはResizeObserverが未実装のためモックを定義
// Radix UI の Select / ScrollArea が内部で使用する
global.ResizeObserver = class ResizeObserver {
  observe() {}
  unobserve() {}
  disconnect() {}
};
