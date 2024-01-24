"use client";
import { Layout, theme } from "antd";
import HeaderForm from "./components/header-form";
import ContentTable from "./components/content-table";
import TableContainer from "@/app/manage/components/table-container";
import { PaginationPayload } from "@/api/common";
import {CollectConfig as ICollectConfig, CollectConfig} from "@/api/models/CollectConfig";
import useSWR from "swr";
import { list, LIST } from "@/api/collect_config";
import {Dispatch, SetStateAction, useState} from "react";

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

export interface ICommonCollectionSettingProps {
    pagination: PaginationPayload<ICollectConfig>,
    setPagination: Dispatch<SetStateAction<PaginationPayload<ICollectConfig>>>
}

export default function pages() {
      const {
        token: { colorBgContainer },
      } = theme.useToken();

    const [pagination, setPagination] = useState<
        PaginationPayload<ICollectConfig>
    >({
        current: 1,
        page_size: 10,
        data: null,
    });

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
            <HeaderForm pagination={pagination} setPagination={setPagination}/>
          </Header>
          <Content>
            <TableContainer>
              <ContentTable pagination={pagination} setPagination={setPagination}/>
            </TableContainer>
          </Content>
        </Layout>
      );
}
