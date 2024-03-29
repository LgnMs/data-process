"use client";

import React, { useEffect, useRef } from "react";
import { Card, Statistic } from "antd";
import { Chart } from "@antv/g2";
import useSWR from "swr";
import * as Statistics from "@/api/statistics";
import dayjs from "dayjs";

export default function SharingTaskCard() {
  const chartRef = useRef<HTMLDivElement>(null);
  const cardRef = useRef<any>(null);
  const chartIn = useRef<InstanceType<typeof Chart>>();

  const { data, isLoading } = useSWR(
    [Statistics.SHARING_TASK_INFO],
    ([]) => Statistics.sharing_task_info({
      date: [dayjs().subtract(1, "year").valueOf(), dayjs().valueOf()]
    })
  );

  let resData: Array<Record<string, any>> = [];

  if (data?.data?.list) {
    Object.keys(data.data.list).forEach(key => {
      resData.push({ date: key,  '访问量': data.data?.list[key], type: '每日访问量'})
    })
    resData.sort((a, b) => dayjs(a.date).valueOf() - dayjs(b.date).valueOf())
  }

  useEffect(() => {
    if (!chartRef.current) return;

    if (!chartIn.current) {
      chartIn.current = new Chart({
        container: chartRef.current,
        autoFit: true,
        margin: 0,
      });

      chartIn.current
        .interval()
        .encode("x", "date")
        .encode("y", "访问量")
        .axis("y", {
          line: false,
          tick: false,
          title: false,
          label: false,
          grid: false,
        })
        .axis("x", {
          line: false,
          tick: false,
          title: false,
          label: false,
          grid: false,
        });

      chartIn.current.data(
        resData?.map((item) => {
          item.type = "调用次数";
          return item;
        })
      );

      chartIn.current.render();
    }
    chartIn.current.data(resData);

    chartIn.current.render();
  }, [data]);


  let tody_num = 0;
  const len = resData.length;

  if (len > 0) {
    if (resData[len - 1].date === dayjs().format("YYYY-MM-DD")) {
      tody_num = resData[len - 1]['访问量']
    }
  }

  return (
    <Card
      bordered={false}
      style={{ width: "100%" }}
      styles={{ body: { padding: "20px 24px 8px" } }}
      ref={cardRef}
      loading={isLoading}
    >
      <Statistic title="共享接口被调用的次数" value={data?.data?.num_items} />
      <div ref={chartRef} style={{ height: 50 }}></div>
      <div
        style={{
          paddingTop: 8,
          marginTop: 8,
          borderTop: "1px solid rgba(5, 5, 5, 0.06)",
        }}
      >
        日调用次数 <span style={{ paddingLeft: 12 }}>{tody_num}</span>
      </div>
    </Card>
  );
}
