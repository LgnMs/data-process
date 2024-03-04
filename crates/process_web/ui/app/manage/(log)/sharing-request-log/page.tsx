"use client";
import { Layout, theme } from "antd";
import HeaderForm from "./components/header-form";
import ContentTable from "./components/content-table";
import TableContainer from "@/app/manage/components/table-container";
import DrawerInfo from "./components/drawer-info";
import { useMainContext } from "@/contexts/main";

const { Header, Content } = Layout;

export default function pages() {
  const {
    token: { colorBgContainer },
  } = theme.useToken();
  const { state, dispatch } = useMainContext()!;

  return (
    <Layout>
      <Header
        style={{
          backgroundColor: colorBgContainer,
          height: 52,
          padding: "12px 16px",
          borderLeft: "1px solid #f5f5f5",
        }}
      >
        <HeaderForm />
      </Header>
      <Content>
        <TableContainer>
          <ContentTable />
        </TableContainer>
      </Content>

      <DrawerInfo
        open={state.sharingRequestLog.drawerOpen}
        close={() => {
          dispatch({
            type: "sharingRequestLog.setDrawerOpen",
            drawerOpen: false,
          });
        }}
      />
    </Layout>
  );
}
