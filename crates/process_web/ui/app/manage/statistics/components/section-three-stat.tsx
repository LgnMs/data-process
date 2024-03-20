"use client";

import { Area, Bar, Column, Pie } from "@antv/g2plot";
import { Card, Divider, Flex, Space, Typography } from "antd";
import React, { useEffect, useRef } from "react";

export default function SectionThreeStat() {
  const chartRef = useRef(null);
  const cardRef = useRef<any>(null);

  useEffect(() => {
    const parentHeight = cardRef.current!.parentNode!.offsetHeight;

    const area = new Area(chartRef.current!, {
      data: [
        {
          Date: "2010-01",
          scales: 1998,
        },
        {
          Date: "2010-02",
          scales: 1850,
        },
        {
          Date: "2010-03",
          scales: 1720,
        },
        {
          Date: "2010-04",
          scales: 1818,
        },
        {
          Date: "2010-05",
          scales: 1920,
        },
      ],
      height: parentHeight - 105,
      xField: "Date",
      yField: "scales",
      xAxis: {
        range: [0, 1],
        tickCount: 5,
      },
      areaStyle: () => {
        return {
          fill: "l(270) 0:#ffffff 0.5:#7ec2f3 1:#1890ff",
        };
      },
    });
    area.render();
  }, []);

  return (
    <Card
      title="采集任务 共采集1000条数据"
      bordered={false}
      style={{ width: "100%" }}
      ref={cardRef}
    >
      {/* <Space split={<Divider type="vertical" />}>
        <Typography.Text strong>系统共采集1000条数据</Typography.Text>
      </Space> */}
      <div ref={chartRef}></div>
    </Card>
  );
}
