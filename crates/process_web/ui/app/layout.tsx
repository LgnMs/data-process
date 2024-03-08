import React from "react";
import { Inter } from "next/font/google";
import type { Metadata } from "next";

import "./globals.css";

const inter = Inter({ subsets: ["latin"] });

export async function generateMetadata() {
  const config = await fetch(`http://localhost:${process.env.PORT}/config.json`, { cache: 'no-store' }).then(res => res.json());

  return {
    title: config.title,
    description: config.description,
  }
}

export default function RootLayout({
  children,
}: {
  children: React.ReactNode;
}) {
  return (
    <html lang="zh-CN">
      <body className={inter.className}>{children}</body>
    </html>
  );
}
