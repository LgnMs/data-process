import React from "react";
import Script from "next/script";

import "./globals.css";

export default async function RootLayout({
  children,
}: {
  children: React.ReactNode;
}) {
  return (
    <html lang="zh-CN">
      <body>{children}</body>
      {process.env.USE_REMOTE_AUTH === "true" && (
        <Script id="remote-auth" src="/remote-auth.js" />
      )}
    </html>
  );
}
