"use client";
import { Layout, theme } from "antd";
import HeaderForm from "./components/header-form";
import ContentTable from "./components/content-table";
import TableContainer from "@/app/manage/components/table-container";
import EditForm from "@/app/manage/collection-setting/components/edit-form";
import { useMainContext } from "@/contexts/main";

const { Header, Content } = Layout;

export interface ICommonCollectionSettingProps {}

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

      <EditForm
        open={state.collectConfig.editFormOpen}
        close={() => {
          dispatch({
            type: "collectConfig.setEditFormOpen",
            editFormOpen: false,
          });
        }}
      />
    </Layout>
  );
}
