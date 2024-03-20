"use client";

import { Bar, Column, Pie } from "@antv/g2plot";
import { Card, Divider, Flex, Space, Typography } from "antd";
import React, { useEffect, useRef } from "react";

export default function SectionTwoStat() {
  const chartRef = useRef(null);
  const cardRef = useRef<any>(null);

  useEffect(() => {
    const parentHeight = cardRef.current!.parentNode!.offsetHeight;

    const data = [
      { year: "1951 年", value: 38 },
      { year: "1952 年", value: 52 },
      { year: "1956 年", value: 61 },
      { year: "1957 年", value: 145 },
      { year: "1958 年", value: 48 },
    ];
    const bar = new Bar(chartRef.current!, {
      data,
      height: parentHeight - 126,
      xField: "value",
      yField: "year",
      seriesField: "year",
      legend: {
        position: "top-left",
      },
    });

    bar.render();
  }, []);

  return (
    <Card ref={cardRef} title="常用任务" bordered={false} style={{ width: "100%" }}>
      <div ref={chartRef}></div>
    </Card>
  );
}
