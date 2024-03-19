'use client'

import { Area, Bar, Column, Pie } from "@antv/g2plot";
import { Card, Divider, Flex, Space, Typography } from "antd";
import React, { useEffect, useRef } from "react";

export default function SectionThreeStat() {
  const chartRef = useRef(null);

  useEffect(() => {
    const area = new Area(chartRef.current!, {
      data: [
        {
          "Date": "2010-01",
          "scales": 1998
        },
        {
          "Date": "2010-02",
          "scales": 1850
        },
        {
          "Date": "2010-03",
          "scales": 1720
        },
        {
          "Date": "2010-04",
          "scales": 1818
        },
        {
          "Date": "2010-05",
          "scales": 1920
        },],
      xField: 'Date',
      yField: 'scales',
      height: 150,
      xAxis: {
        range: [0, 1],
        tickCount: 5,
      },
      areaStyle: () => {
        return {
          fill: 'l(270) 0:#ffffff 0.5:#7ec2f3 1:#1890ff',
        };
      },
    });
    area.render();

  }, [])

  return <Card title="采集数据 共1000条" bordered={false} style={{width: '100%'}}>
    <div ref={chartRef}></div>
  </Card>
}