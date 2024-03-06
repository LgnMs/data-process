import React from "react";
import MainLayout from "@/app/manage/components/main-layout";
import StyledComponentsRegistry from "@/lib/antd-registry";

export default function RootLayout({
  children,
}: {
  children: React.ReactNode;
}) {
  return (
    <StyledComponentsRegistry>
      <MainLayout>{children}</MainLayout>
    </StyledComponentsRegistry>
  );
}
