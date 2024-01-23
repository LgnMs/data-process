"use client";
import { Button, Divider, Popconfirm, Space, Table, Typography } from "antd";
import useSWR from "swr";

import * as CollectConfig from "@/api/collect_config";
import { useState } from "react";
import { PaginationPayload } from "@/api/common";
import { CollectConfig as ICollectConfig } from "@/api/models/CollectConfig";

export default function ContentTable() {
  const [pagination, setPagination] = useState<
    PaginationPayload<ICollectConfig>
  >({
    current: 1,
    page_size: 10,
    data: null,
  });

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
      pagination={{
        current: pagination.current,
        pageSize: pagination.page_size,
        total,
      }}
    />
  );
}
