"use client";
import { ConfigProvider, Layout } from "antd";
import { useState } from "react";
import zhCN from "antd/locale/zh_CN";
import { MainHeader } from "./main-header";
import { MainSlider } from "./main-slider";
import { MainContent } from "./main-content";
import dayjs from "dayjs";
import quarterOfYear from "dayjs/plugin/quarterOfYear";
import "dayjs/locale/zh-cn";
import { MainContextProvider } from "@/contexts/main";
import useSWR from "swr";
import { http_post } from "@/api/common";

dayjs.extend(quarterOfYear);

export default function MainLayout({
  children,
}: {
  children: React.ReactNode;
}) {
  const [collapsed, setCollapsed] = useState(false);
  let loading = false;
  const { data, isLoading } = useSWR<any>("/api/auth/authorize",
    () => http_post(
      "/api/auth/authorize",
      {
          body: JSON.stringify({
            client_id: "foo",
            client_secret: "bar"
          })
      }
    )
  )
  loading = isLoading;
  if (!isLoading && data?.data) {
    sessionStorage.setItem("Authorization", `${data.data.token_type} ${data.data.access_token}`)
  }

  return !loading && (
    <MainContextProvider>
      <ConfigProvider locale={zhCN}>
        <Layout style={{ position: "relative" }}>
          <MainHeader />
          <Layout
            hasSider
            style={{
              position: "relative",
              padding: `64px 0 0 ${collapsed ? "80px" : "200px"}`,
            }}
          >
            <MainSlider onCollapse={(value) => setCollapsed(value)} />
            <Layout>
              {/* <MainBreadcrumb /> */}
              <MainContent>{children}</MainContent>
            </Layout>
          </Layout>
        </Layout>
      </ConfigProvider>
    </MainContextProvider>
  );
}
