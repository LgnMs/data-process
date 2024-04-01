'use client'
import { useEffect, useRef } from "react";
import useSWR from "swr";
import * as Statistics from "@/api/statistics";
import { Card, Flex, Space, Statistic, Table } from "antd";
import dayjs from "dayjs";

export default function SharingInfoCard() {
  const { data, isLoading } = useSWR(
    [Statistics.SHARING_TASK_INFO],
    ([]) =>
      Statistics.sharing_task_info({
        date: [dayjs().subtract(1, "year").valueOf(), dayjs().valueOf()],
      })
  );
  const columns: any = [
    {
      title: "名称",
      dataIndex: "name",
    },
    {
      title: "调用次数",
      dataIndex: "num_items",
    },
  ]

  let dataSource = [];

  if (data?.data) {
    dataSource = data.data.rank_list.slice(0, 7)
  }

  return <Card
      bordered={false}
      style={{ width: "100%" }}
      styles={{ body: { padding: "20px 24px 8px" } }}
      loading={isLoading}
      title="共享接口调用情况"
    >
      <Space direction="vertical" style={{width: '100%', height: 400}}>
        <Flex justify="space-between" style={{width: '60%', paddingLeft: 12}}>
          <Statistic title="用户数量" value={data?.data?.user_number} />
          <Statistic title="用户平均调用次数" value={data?.data?.avg_num_user_calls_api} />
        </Flex>
        <Table
          size="small"
          bordered
          loading={isLoading}
          columns={columns}
          dataSource={dataSource}
          rowKey="data_sharing_config_id"
          pagination={false}
        />
      </Space>
    </Card>
}