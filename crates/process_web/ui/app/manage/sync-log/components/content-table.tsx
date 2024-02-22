"use client";
import { Space, Table, Tag, Typography } from "antd";
import useSWR from "swr";

import * as SyncLog from "@/api/sync_log";
import { SyncLog as ISyncLog } from "@/api/models/SyncLog";
import { useMainContext } from "@/contexts/main";
import {
  CheckCircleOutlined,
  ClockCircleOutlined,
  CloseCircleOutlined,
  SyncOutlined,
} from "@ant-design/icons";
import React from "react";
import dayjs from "dayjs";

export default function ContentTable() {
  const { state, dispatch } = useMainContext()!;
  const pagination = state.syncLog.pagination;

  const { data, isLoading } = useSWR(
    [SyncLog.LIST, pagination],
    ([_, pagination]) => SyncLog.list(pagination)
  );

  const columns: any = [
    {
      title: "id",
      dataIndex: "id",
    },
    {
      title: "同步任务id",
      dataIndex: "sync_config_id",
    },
    {
      title: "同步任务名称",
      dataIndex: "sync_config.name",
      render: (_: string, record: ISyncLog) => {
        return record.sync_config.name;
      },
    },
    {
      title: "运行状态",
      dataIndex: "status",
      width: 120,
      align: "center",
      render: (text: number) => {
        switch (text) {
          case 0:
            return (
              <Tag icon={<ClockCircleOutlined />} color="default">
                等待
              </Tag>
            );
          case 1:
            return (
              <Tag icon={<SyncOutlined spin />} color="processing">
                运行中
              </Tag>
            );
          case 2:
            return (
              <Tag icon={<CheckCircleOutlined />} color="success">
                完成
              </Tag>
            );
          case 3:
            return (
              <Tag icon={<CloseCircleOutlined />} color="error">
                失败
              </Tag>
            );
        }
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
      title: "创建日期",
      dataIndex: "create_time",
      render: (text: number) => {
        return dayjs(text).format("YYYY-MM-DD HH:mm:ss");
      },
    },
    {
      title: "操作",
      width: 150,
      render: (_: any, record: ISyncLog) => {
        return (
          <Space>
            <Typography.Link
              onClick={() => {
                dispatch({
                  type: "syncLog.setDrawerOpen",
                  drawerOpen: true,
                });
                dispatch({
                  type: "syncLog.setDrawerData",
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

  let dataSource: Array<ISyncLog> = [];
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
          type: "syncLog.setPagination",
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
