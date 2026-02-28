import { render, RenderOptions } from '@testing-library/react';
import { Theme } from '@radix-ui/themes';

function AllProviders({ children }: { children: React.ReactNode }) {
  return <Theme>{children}</Theme>;
}

const customRender = (
  ui: React.ReactElement,
  options?: Omit<RenderOptions, 'wrapper'>
) => render(ui, { wrapper: AllProviders, ...options });

// Testing Libraryの全exportを再exportし、renderだけカスタム版で上書き
export * from '@testing-library/react';
export {customRender as render }
