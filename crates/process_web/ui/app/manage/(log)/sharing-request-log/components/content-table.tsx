"use client";
import { Space, Table, Typography } from "antd";
import useSWR from "swr";

import * as SharingRequestLog from "@/api/sharing_request_log";
import { SharingRequestLog as ISharingRequestLog } from "@/api/models/SharingRequestLog";
import { useMainContext } from "@/contexts/main";
import React from "react";
import dayjs from "dayjs";

export default function ContentTable() {
  const { state, dispatch } = useMainContext()!;
  const pagination = state.sharingRequestLog.pagination;

  const { data, isLoading } = useSWR(
    [SharingRequestLog.LIST, pagination],
    ([_, pagination]) => SharingRequestLog.list(pagination)
  );

  const columns: any = [
    {
      title: "id",
      dataIndex: "id",
      width: 50,
    },
    // {
    //   title: "共享配置id",
    //   dataIndex: "data_sharing_config_id",
    //   width: 50
    // },
    {
      title: "共享配置名称",
      dataIndex: "data_sharing_config.name",
      render: (_: string, record: ISharingRequestLog) => {
        return record.data_sharing_config.name;
      },
    },
    {
      title: "日志",
      dataIndex: "log",
      ellipsis: true,
      width: 600,
    },
    {
      title: "更新日期",
      dataIndex: "update_time",
      render: (text: number) => {
        return dayjs(text).format("YYYY-MM-DD HH:mm:ss");
      },
    },
    {
      title: "创建日期",
      dataIndex: "create_time",
      render: (text: number) => {
        return dayjs(text).format("YYYY-MM-DD HH:mm:ss");
      },
    },
    {
      title: "操作",
      width: 150,
      render: (_: any, record: ISharingRequestLog) => {
        return (
          <Space>
            <Typography.Link
              onClick={() => {
                dispatch({
                  type: "sharingRequestLog.setDrawerOpen",
                  drawerOpen: true,
                });
                dispatch({
                  type: "sharingRequestLog.setDrawerData",
                  drawerData: record,
                });
              }}
            >
              查看日志
            </Typography.Link>
          </Space>
        );
      },
    },
  ];

  let dataSource: Array<ISharingRequestLog> = [];
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
          type: "sharingRequestLog.setPagination",
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
