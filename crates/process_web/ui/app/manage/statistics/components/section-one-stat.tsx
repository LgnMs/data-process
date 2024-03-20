"use client";

import { Column, Pie } from "@antv/g2plot";
import { Card, Divider, Flex, Space, Typography } from "antd";
import React, { useEffect, useRef } from "react";

export default function SectionOneStat() {
  const chartRef = useRef(null);
  const cardRef = useRef<any>(null);



  useEffect(() => {
    const parentHeight = cardRef.current!.parentNode!.offsetHeight;

    const data = [
      { type: "采集", value: 27 },
      { type: "共享", value: 25 },
      { type: "同步", value: 18 },
    ];
    const piePlot = new Pie(chartRef.current!, {
      appendPadding: 10,
      data,
      height: parentHeight - 126,
      angleField: "value",
      colorField: "type",
      radius: 0.9,
      label: {
        type: "inner",
        offset: "-30%",
        content: ({ percent }) => `${(percent * 100).toFixed(0)}%`,
        style: {
          fontSize: 14,
          textAlign: "center",
        },
      },
      interactions: [{ type: "element-active" }],
    });

    piePlot.render();
  }, []);

  return (
    <Card ref={cardRef} title="任务占比" bordered={false} style={{ width: "100%" }}>
      <Typography.Text strong>现有任务 125条</Typography.Text>
      <div ref={chartRef}></div>
    </Card>
  );
}
