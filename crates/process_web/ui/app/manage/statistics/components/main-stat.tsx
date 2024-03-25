'use client'

import { Column } from '@antv/g2plot';
import { Card, Divider, Flex, Space, Typography } from "antd";
import React, { useEffect, useRef } from "react";

export default function MainStat() {
  const chartRef = useRef<any>(null);
  const cardRef = useRef<any>(null);

  useEffect(() => {
    const parentHeight = cardRef.current!.parentNode!.offsetHeight;

    const column = new Column(chartRef.current!, {
      data: [
        {
          "type": "总任务",
          "status": "未启用",
          "value": 14500
        },
        {
          "type": "总任务",
          "status": "已启用",
          "value": 14500
        },
        {
          "type": "总任务",
          "status": "运行中",
          "value": 14500
        },
        {
          "type": "总任务",
          "status": "运行成功",
          "value": 14500
        },
        {
          "type": "总任务",
          "status": "运行失败",
          "value": 14500
        },
        {
          "type": "采集任务",
          "status": "未启用",
          "value": 14500
        },
        {
          "type": "采集任务",
          "status": "已启用",
          "value": 14500
        },
        {
          "type": "采集任务",
          "status": "运行中",
          "value": 14500
        },
        {
          "type": "采集任务",
          "status": "运行成功",
          "value": 14500
        },
        {
          "type": "采集任务",
          "status": "运行失败",
          "value": 14500
        },
        {
          "type": "同步任务",
          "status": "未启用",
          "value": 8500
        },
        {
          "type": "同步任务",
          "status": "运行成功",
          "value": 14500
        },
        {
          "type": "同步任务",
          "status": "运行失败",
          "value": 14500
        },
        {
          "type": "同步任务",
          "status": "已启用",
          "value": 8500
        },
        {
          "type": "同步任务",
          "status": "运行中",
          "value": 14500
        },
        {
          "type": "共享任务",
          "status": "未启用",
          "value": 8500
        },
        {
          "type": "共享任务",
          "status": "已启用",
          "value": 8500
        },
      ],
      height: parentHeight - 126,
      xField: 'type',
      yField: 'value',
      seriesField: 'status',
      isGroup: true,
    });

    column.render();
  }, [])

  return <Card ref={cardRef} title="系统情况" bordered={false} style={{width: '100%'}}>
    <Space split={<Divider type="vertical" />}>
      <Typography.Text strong>采集任务 10000</Typography.Text>
      <Typography.Text strong>同步任务 10000</Typography.Text>
      <Typography.Text strong>共享任务 10000</Typography.Text>
    </Space>
    <div ref={chartRef} style={{height: 'calc(100% - 46px)'}}></div>
  </Card>
}