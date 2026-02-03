import type { Metadata } from "next";
import { Theme } from "@radix-ui/themes";
import "@radix-ui/themes/styles.css";
import "./globals.css";

export const metadata: Metadata = {
  title: "ToDo App",
  description: "A simple ToDo management application",
};

export default function RootLayout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  return (
    <html lang="ja">
      <body>
        <Theme accentColor="blue" grayColor="slate" radius="medium">
          {children}
        </Theme>
      </body>
    </html>
  );
}
