"use client";
import { message, Popconfirm, Space, Table, Typography } from "antd";
import useSWR from "swr";
import React from "react";

import * as DatasourceList from "@/api/data_source_list";
import { DataSourceList as IDatasourceList } from "@/api/models/DataSourceList";
import { useMainContext } from "@/contexts/main";

export default function ContentTable() {
  const { state, dispatch } = useMainContext()!;
  const pagination = state.dataSourceList.pagination;

  const { data, isLoading, mutate } = useSWR(
    [DatasourceList.LIST, pagination],
    ([_, pagination]) => DatasourceList.list(pagination)
  );

  const columns: any = [
    {
      title: "id",
      dataIndex: "id",
    },
    {
      title: "数据库名称",
      dataIndex: "database_name",
    },
    {
      title: "数据库类型",
      dataIndex: "database_type",
    },
    {
      title: "操作",
      width: 150,
      render: (_: any, record: IDatasourceList) => {
        return (
          <Space>
            <Typography.Link
              onClick={() => {
                dispatch({
                  type: "dataSourceList.setEditFormOpen",
                  editFormOpen: true,
                });
                dispatch({
                  type: "dataSourceList.setEditFormData",
                  editFormData: record,
                });
              }}
            >
              查看
            </Typography.Link>
            <Popconfirm
              title="确定要删除吗？"
              onConfirm={async () => {
                const res = await DatasourceList.del(record.id!);
                if (res.data) {
                  message.success("删除成功");
                  await mutate();
                }
              }}
            >
              <Typography.Link>删除</Typography.Link>
            </Popconfirm>
          </Space>
        );
      },
    },
  ];

  let dataSource: Array<IDatasourceList> = [];
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
          type: "dataSourceList.setPagination",
          pagination: {
            ...pagination,
            current: current as number,
            page_size: pageSize as number,
          },
        });
      }}
    />
  );
}
