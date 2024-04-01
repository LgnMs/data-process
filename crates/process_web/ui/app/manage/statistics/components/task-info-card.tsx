'use client'
import { useEffect, useRef } from "react";
import useSWR from "swr";
import * as Statistics from "@/api/statistics";
import { Chart } from "@antv/g2";
import { Card } from "antd";

export default function TaskInfoCard() {
  const cardRef = useRef<HTMLDivElement>(null);
  const chartRef = useRef<HTMLDivElement>(null);
  const chartIn = useRef<InstanceType<typeof Chart>>();
  const { data, isLoading } = useSWR(
    [Statistics.GET_TASK_INFO],
    ([]) =>
      Statistics.get_task_info()
  );

  useEffect(() => {
    if (!chartRef.current) return;
    
    let taskinfo: { item: string; count: number; percent: number }[] = [];

    if (data?.data) {
      const { collect_num, sync_num, sharing_num } = data.data;
      const sum = collect_num + sync_num + sharing_num;

      const getPercent = (num: number) => {
        return Number((num / sum).toFixed(2))
      }

      taskinfo = [
        { item: '采集任务', count: collect_num, percent: getPercent(collect_num) },
        { item: '同步任务', count: sync_num,  percent: getPercent(sync_num) },
        { item: '共享任务', count: sharing_num,  percent: getPercent(sharing_num) },
      ]
    }

    if (!chartIn.current) {
      chartIn.current = new Chart({
        container: chartRef.current,
        autoFit: true,
      });

      chartIn.current.coordinate({ type: 'theta', outerRadius: 0.8 });

      chartIn.current
        .interval()
        .data(taskinfo)
        .transform({ type: 'stackY' })
        .encode('y', 'percent')
        .encode('color', 'item')
        .legend('color', { position: 'bottom', layout: { justifyContent: 'center' } })
        .label({
          position: 'outside',
          text: (data: { item: any; percent: number; }) => `${data.item}: ${data.percent * 100}%`,
        })
        .tooltip((data) => ({
          name: data.item,
          value: `${data.percent * 100}%`,
        }));

      chartIn.current.render();
    } else {
      chartIn.current.changeData(taskinfo);
    }
  }, [data]);

  return (
    <Card
      bordered={false}
      style={{ width: "100%" }}
      styles={{ body: { padding: "20px 24px 8px" } }}
      ref={cardRef}
      loading={isLoading}
      title="任务占比"
    >
      <div ref={chartRef} style={{height: 400}}></div>
    </Card>
  );
}