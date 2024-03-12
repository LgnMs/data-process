import React from "react";
import { Inter } from "next/font/google";
import Script from "next/script";

import "./globals.css";
import { getConfig } from "@/lib/getConfig";

const inter = Inter({ subsets: ["latin"] });

export default async function RootLayout({
  children,
}: {
  children: React.ReactNode;
}) {
  const config = await getConfig();
  console.log(config)

  return (
    <html lang="zh-CN">
      <body className={inter.className}>{children}</body>
      {config.USE_REMOTE_AUTH === true && (
        <Script id="remote-auth" src="/remote-auth.js" />
      )}
    </html>
  );
}
