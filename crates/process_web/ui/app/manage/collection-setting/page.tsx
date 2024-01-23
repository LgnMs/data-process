"use client";
import { Layout, theme } from "antd";
import HeaderForm from "./components/header-form";
import ContentTable from "./components/content-table";
import TableContainer from "@/app/manage/components/table-container";
import { PaginationPayload } from "@/api/common";
import { CollectConfig } from "@/api/models/CollectConfig";
import useSWR from "swr";
import { list, LIST } from "@/api/collect_config";

const { Header, Content } = Layout;

export function useCollectionSetting(
  pagination: PaginationPayload<CollectConfig>
) {
  const { data, isLoading, mutate } = useSWR(
    [LIST, pagination],
    ([url, pagination]) => list(pagination)
  );

  return {
    collectionSettings: data,
    isLoading,
    mutate,
  };
}

export default function pages() {
  const {
    token: { colorBgContainer },
  } = theme.useToken();
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
        <HeaderForm
          onAddClick={function (): void {
            throw new Error("Function not implemented.");
          }}
          onSearch={function (name: string): void {
            throw new Error("Function not implemented.");
          }}
        />
      </Header>
      <Content>
        <TableContainer>
          <ContentTable />
        </TableContainer>
      </Content>
    </Layout>
  );
}
