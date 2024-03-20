"use client";

import { Area, Bar, Column, Line, Pie } from "@antv/g2plot";
import { Card, Divider, Flex, Space, Typography } from "antd";
import React, { useEffect, useRef } from "react";

export default function SectionFiveStat() {
  const chartRef = useRef(null);
  const cardRef = useRef<any>(null);

  useEffect(() => {
    const parentHeight = cardRef.current!.parentNode!.offsetHeight;

    const line = new Line(chartRef.current!, {
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
      padding: "auto",
      xField: "Date",
      yField: "scales",
      xAxis: {
        // type: 'timeCat',
        tickCount: 5,
      },
    });

    line.render();
  }, []);

  return (
    <Card
      title="同步任务 共运行1000次"
      bordered={false}
      style={{ width: "100%" }}
      ref={cardRef}
    >
      {/* <Space split={<Divider type="vertical" />}>
        <Typography.Text strong>同步任务共运行1000次</Typography.Text>
      </Space> */}
      <div ref={chartRef}></div>
    </Card>
  );
}
