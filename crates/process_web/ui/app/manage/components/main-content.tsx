import { Layout, theme } from "antd";

const { Content } = Layout;

export function MainContent({ children }: { children: React.ReactNode }) {
  return (
    <Content
      style={{
        padding: 0,
        margin: 0,
        minHeight: "calc(100vh - 64px)",
      }}
    >
      {children}
    </Content>
  );
}
