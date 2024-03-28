"use client";

import React, { useEffect, useRef } from "react";
import { Card, Statistic } from "antd";
import { Chart } from "@antv/g2";
import useSWR from "swr";
import * as Statistics from "@/api/statistics";
import dayjs from "dayjs";

export default function CollectTaskCard() {
    const chartRef = useRef<HTMLDivElement>(null);
    const cardRef = useRef<any>(null);
    const chartIn = useRef<InstanceType<typeof Chart>>();
    // let chart: InstanceType<typeof Chart> | undefined;

    const { data: collenct_task_info, isLoading: isLoading1 } = useSWR(
        [Statistics.COLLECT_TASK_INFO],
        ([]) => Statistics.collect_task_info()
    );

    const { data: collect_task_info_day_list, isLoading: isLoading2 } = useSWR(
        [Statistics.COLLECT_TASK_INFO_DAY_LIST],
        ([]) => Statistics.collect_task_info_day_list({
            date: [dayjs().subtract(1, 'year').valueOf(), dayjs().valueOf()]
        })
    );

    useEffect(() => {
        if (!chartRef.current) return;

        if (!chartIn.current) {
            chartIn.current = new Chart({
                container: chartRef.current,
                autoFit: true,
                margin: 0
            });

            chartIn.current
                .area()
                .encode('x', (d: any) => d.date)
                .encode('y', 'value')
                .encode('shape', 'area') // 'area', 'smooth', 'hvh', 'vh', 'hv'
                .style('fill', 'linear-gradient(-90deg, white 0%, #c3a3f0 100%)')
                .axis('y', { line: false, tick: false, title: false, label: false, grid: false})
                .axis('x', { line: false, tick: false, title: false, label: false, grid: false })

                const data: Array<{date: string, value: number}> = [];
                const obj = collect_task_info_day_list?.data?.list;
                if (obj) {
                    Object.keys(obj).map(key => {
                        data.push({ date: key, value: obj[key] })
                    })
                }
                chartIn.current.data(data);
        
                chartIn.current.render();
        };
        const data: Array<{date: string, value: number}> = [];
        const obj = collect_task_info_day_list?.data?.list;
        if (obj) {
            Object.keys(obj).map(key => {
                data.push({ date: key, value: obj[key] })
            })
        }
        chartIn.current.data(data);
        chartIn.current.render();
        


    }, [collenct_task_info, collect_task_info_day_list]);

  return (
    <Card
      bordered={false}
      style={{ width: "100%" }}
      styles={{ body: {padding: "20px 24px 8px"} }}
      ref={cardRef}
      loading={isLoading1 && isLoading2}
      
    >
      <Statistic title="总采集量" value={collenct_task_info?.data?.num_items} />
      <div ref={chartRef} style={{height: 50}}></div>
      <div style={{paddingTop: 8, marginTop: 8, borderTop: '1px solid rgba(5, 5, 5, 0.06)'}}>日采集量 <span style={{paddingLeft: 12}}>11111</span></div>
    </Card>
  );
}
