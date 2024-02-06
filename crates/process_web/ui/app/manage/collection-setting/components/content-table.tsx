"use client";
import { message, Popconfirm, Space, Table, Tag, Typography } from "antd";
import useSWR from "swr";

import * as CollectConfig from "@/api/collect_config";
import { CollectConfig as ICollectConfig } from "@/api/models/CollectConfig";
import { useMainContext } from "@/contexts/main";
import { CheckCircleOutlined, ClockCircleOutlined, CloseCircleOutlined, SyncOutlined } from "@ant-design/icons";
import React from "react";
import LabelTips from "@/app/manage/components/label-tips";

export default function ContentTable() {
  const { state, dispatch } = useMainContext()!;
  const pagination = state.collectConfig.pagination;

  const { data, isLoading, mutate } = useSWR(
    [CollectConfig.LIST, pagination],
    ([url, pagination]) => CollectConfig.list(pagination)
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
      title: "缓存表",
      dataIndex: "cache_table_name",
    },
    {
      title: "描述",
      dataIndex: "desc",
    },
    {
      title: <LabelTips
        tips={`设置执行周期后启用`}
      >
        状态
      </LabelTips>,
      width: 120,
      align: "center",
      render: (_:any, record: ICollectConfig) => {
        if (record.cron) {
          return <Tag icon={<CheckCircleOutlined />} color="success">
            启用
          </Tag>
        } else {
          return <Tag icon={<ClockCircleOutlined />} color="default">
            停用
          </Tag>
        }
      }
    },
    {
      title: "操作",
      width: 150,
      render: (_: any, record: ICollectConfig) => {
        return (
          <Space>
            <Typography.Link
              onClick={() => {
                dispatch({
                  type: "collectConfig.setEditFormOpen",
                  editFormOpen: true,
                });
                dispatch({
                  type: "collectConfig.setEditFormData",
                  editFormData: record,
                });
              }}
            >
              查看
            </Typography.Link>
            <Typography.Link
              onClick={async () => {
                await CollectConfig.execute(record.id!);
                message.success("执行成功");
              }}
            >
              立即执行
            </Typography.Link>
            <Popconfirm
              title="确定要删除吗？"
              onConfirm={async () => {
                const res = await CollectConfig.del(record.id!);
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
          type: "collectConfig.setPagination",
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
