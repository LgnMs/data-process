import React from "react";
import { Inter } from "next/font/google";
import Script from "next/script";

import "./globals.css";

const inter = Inter({ subsets: ["latin"] });

export default async function RootLayout({
  children,
}: {
  children: React.ReactNode;
}) {

  return (
    <html lang="zh-CN">
      <body className={inter.className}>{children}</body>
      {process.env.USE_REMOTE_AUTH === 'true' && (
        <Script id="remote-auth" src="/remote-auth.js" />
      )}
    </html>
  );
}
