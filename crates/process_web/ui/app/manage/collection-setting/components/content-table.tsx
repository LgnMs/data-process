"use client";
import { Button, Divider, Popconfirm, Space, Table, Typography } from "antd";
import useSWR from "swr";

import * as CollectConfig from "@/api/collect_config";
import {Dispatch, SetStateAction, useState} from "react";
import { PaginationPayload } from "@/api/common";
import { CollectConfig as ICollectConfig } from "@/api/models/CollectConfig";
import {ICommonCollectionSettingProps} from "@/app/manage/collection-setting/page";
import { useMainContext } from "@/contexts/main";

interface IContentTableProps extends ICommonCollectionSettingProps {}
export default function ContentTable() {
  const { state, dispatch } = useMainContext()!;
  const pagination = state.collectConfig.pagination;

  const { data, isLoading } = useSWR(
    [CollectConfig.LIST, pagination],
    ([url, pagination]) => CollectConfig.list(pagination)
  );

  const columns = [
    {
      title: "id",
      dataIndex: "id",
    },
    {
      title: "名称",
      dataIndex: "name",
    },
    {
      title: "描述",
      dataIndex: "desc",
    },
    {
      title: "操作",
      width: 130,
      render: (_: any, record: ICollectConfig) => {
        return (
          <Space>
            <Typography.Link>查看</Typography.Link>
            <Popconfirm title="确定要删除吗？">
              <Typography.Link>删除</Typography.Link>
            </Popconfirm>
          </Space>
        );
      },
    },
  ];

  let dataSource: Array<ICollectConfig> = [];
  let total = 0;
  if (data?.data) {
    dataSource = data.data.list;
    total = data.data.total;
  }

  return (
    <Table
      size="small"
      bordered
      loading={isLoading}
      columns={columns}
      dataSource={dataSource}
      rowKey="id"
      pagination={{
        current: pagination.current,
        pageSize: pagination.page_size,
        total,
      }}
      onChange={({ current, pageSize }) => {
        dispatch({
          type: 'collectConfig.setPagination',
          pagination: {
            ...pagination,
            current: current as number,
            page_size: pageSize as number,
          }
        })
      }}
    />
  );
}
