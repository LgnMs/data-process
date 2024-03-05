"use client";
import { message, Popconfirm, Space, Table, Typography } from "antd";
import useSWR from "swr";
import React from "react";

import * as DataSharingConfig from "@/api/data_sharing_config";
import { DataSharingConfig as IDataSharingConfig } from "@/api/models/DataSharingConfig";
import { useMainContext } from "@/contexts/main";
import dayjs from "dayjs";

export default function ContentTable() {
  const { state, dispatch } = useMainContext()!;
  const pagination = state.dataSharingConfig.pagination;

  const { data, isLoading, mutate } = useSWR(
    [DataSharingConfig.LIST, pagination],
    ([_, pagination]) => DataSharingConfig.list(pagination)
  );

  const columns: any = [
    {
      title: "id",
      dataIndex: "id",
    },
    {
      title: "名称",
      dataIndex: "name",
    },
    {
      title: "api",
      render: (_: any, record: IDataSharingConfig) => {
        return `data_sharing_config/get_data/${record.id}`;
      },
    },
    {
      title: "更新日期",
      dataIndex: "update_time",
      render: (text: number) => {
        return dayjs(text).format("YYYY-MM-DD HH:mm:ss");
      },
    },
    {
      title: "操作",
      width: 150,
      render: (_: any, record: IDataSharingConfig) => {
        return (
          <Space>
            <Typography.Link
              onClick={() => {
                dispatch({
                  type: "dataSharingConfig.setEditFormOpen",
                  editFormOpen: true,
                });
                dispatch({
                  type: "dataSharingConfig.setEditFormData",
                  editFormData: record,
                });
              }}
            >
              查看
            </Typography.Link>
            <Popconfirm
              title="确定要删除吗？"
              onConfirm={async () => {
                const res = await DataSharingConfig.del(record.id!);
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

  let dataSource: Array<IDataSharingConfig> = [];
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
          type: "dataSharingConfig.setPagination",
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
