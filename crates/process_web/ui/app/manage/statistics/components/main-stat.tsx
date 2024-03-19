'use client'

import { Column } from '@antv/g2plot';
import { Card, Divider, Flex, Space, Typography } from "antd";
import React, { useEffect, useRef } from "react";

export default function MainStat() {
  const chartRef = useRef(null);

  useEffect(() => {
    const column = new Column(chartRef.current!, {
      data: [
        {
          "city": "石家庄",
          "type": "水果",
          "value": 14500
        },
        {
          "city": "石家庄",
          "type": "米面",
          "value": 8500
        },],
      height: 300,
      xField: 'city',
      yField: 'value',
      seriesField: 'type',
      isGroup: true,
      columnStyle: {
        radius: [20, 20, 0, 0],
      },
    });

    column.render();
  }, [])

  return <Card title="系统情况" bordered={false} style={{width: '100%'}}>
    <Space split={<Divider type="vertical" />}>
      <Typography.Text strong>采集任务 10000</Typography.Text>
      <Typography.Text strong>同步任务 10000</Typography.Text>
      <Typography.Text strong>共享任务 10000</Typography.Text>
    </Space>
    <div ref={chartRef}></div>
  </Card>
}