"use client";
import { Space, Table, Tag, Typography } from "antd";
import useSWR from "swr";

import * as CollectLog from "@/api/collect_log";
import { CollectLog as ICollectLog } from "@/api/models/CollectLog";
import { useMainContext } from "@/contexts/main";
import {
  CheckCircleOutlined,
  ClockCircleOutlined,
  CloseCircleOutlined,
  SyncOutlined
} from "@ant-design/icons";
import React from "react";
import dayjs from "dayjs";

export default function ContentTable() {
  const { state, dispatch } = useMainContext()!;
  const pagination = state.collectLog.pagination;

  const { data, isLoading} = useSWR(
    [CollectLog.LIST, pagination],
    ([_, pagination]) => CollectLog.list(pagination)
  );

  const columns: any = [
    {
      title: "id",
      dataIndex: "id",
    },
    {
      title: "采集配置id",
      dataIndex: "collect_config_id",
    },
    {
      title: "采集配置名称",
      dataIndex: "collect_config.name",
      render: (_: string, record: ICollectLog) => {
        return record.collect_config.name
      }
    },
    {
      title: "运行状态",
      dataIndex: "status",
      width: 120,
      align: "center",
      render: (text: number) => {
        switch (text) {
          case 0:
            return <Tag icon={<ClockCircleOutlined />} color="default">
              等待
            </Tag>
          case 1:
            return <Tag icon={<SyncOutlined spin />} color="processing">
              运行中
            </Tag>
          case 2:
            return <Tag icon={<CheckCircleOutlined />} color="success">
              完成
            </Tag>
          case 3:
            return <Tag icon={<CloseCircleOutlined />} color="error">
              失败
            </Tag>

        }
      }
    },
    {
      title: "更新日期",
      dataIndex: "update_time",
      render: (text: number) => {
        return dayjs(text).format("YYYY-MM-DD HH:mm:ss")
      }
    },
    {
      title: "创建日期",
      dataIndex: "create_time",
      render: (text: number) => {
        return dayjs(text).format("YYYY-MM-DD HH:mm:ss")
      }
    },
    {
      title: "操作",
      width: 150,
      render: (_: any, record: ICollectLog) => {
        return (
          <Space>
            <Typography.Link
              onClick={() => {
                dispatch({
                  type: "collectLog.setDrawerOpen",
                  drawerOpen: true,
                });
                dispatch({
                  type: "collectLog.setDrawerData",
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

  let dataSource: Array<ICollectLog> = [];
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
          type: "collectLog.setPagination",
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
