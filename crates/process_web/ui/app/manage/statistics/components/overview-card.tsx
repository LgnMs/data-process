"use client";
import { Chart } from "@antv/g2";
import { Card, Col, Row, Tabs, TabsProps } from "antd";
import { useEffect, useRef } from "react";
import useSWR from "swr";
import * as Statistics from "@/api/statistics";
import dayjs from "dayjs";

import styles from "./overview.module.scss";

export default function OverviewCard() {
  const cardRef = useRef<any>(null);
  let isLoading = false;

  const onChange = (key: string) => {
    console.log(key);
  };

  const items: TabsProps["items"] = [
    {
      key: "1",
      label: "采集任务",
      children: <CollectTask />,
    },
    {
      key: "2",
      label: "同步任务",
      children: <SyncTask />,
    }
  ];
  return (
    <Card
      bordered={false}
      style={{ width: "100%" }}
      styles={{ body: { padding: "20px 24px 8px" } }}
      ref={cardRef}
      loading={isLoading}
    >
      <Tabs defaultActiveKey="1" items={items} onChange={onChange} />
    </Card>
  );
}

function RankList(props: {
  title: string;
  list?: Array<{ name: string; num_items: number }>;
}) {
  return (
    <div className={styles.rankList}>
      <div className={styles.title}>{props.title}</div>
      <div>
        {props.list?.map((value, index) => {
          return (
            <div className={styles.item} key={index}>
              <span>
                <span className={styles.index}>{index + 1} </span>
                <span>{value.name}</span>
              </span>
              <span>{value.num_items}</span>
            </div>
          );
        })}
      </div>
    </div>
  );
}

function CollectTask() {
  const chartRef = useRef<HTMLDivElement>(null);
  const chartIn = useRef<InstanceType<typeof Chart>>();
  const { data, isLoading } = useSWR(
    [Statistics.COLLECT_TASK_INFO_DAY_LIST],
    ([]) =>
      Statistics.collect_task_info_day_list({
        date: [dayjs().subtract(1, "year").valueOf(), dayjs().valueOf()],
      })
  );

  useEffect(() => {
    if (!chartRef.current) return;

    let list: Array<Record<string, any>> = [];
    if (data?.data) {
      list = data.data.list.map((item) => {
        item["运行次数"] = item.num_items;
        item["日期"] = item.date;
        return item;
      });
    }

    if (!chartIn.current) {
      chartIn.current = new Chart({
        container: chartRef.current,
        autoFit: true,
      });

      chartIn.current
        .interval()
        .data(list)
        .encode("x", "日期")
        .encode("y", "运行次数")
        .axis("x", {
          title: false,
        })
        .axis("y", {
          title: false,
        });

      chartIn.current.render();
    } else {
      chartIn.current.changeData(list);
    }
  }, [data]);

  return (
    <Row>
      <Col span={18}>
        <div ref={chartRef} style={{ height: 300 }}></div>
      </Col>
      <Col span={6}>
        <RankList title="采集任务运行次数" list={data?.data?.rank_list} />
      </Col>
    </Row>
  );
}

function SyncTask() {
  const chartRef = useRef<HTMLDivElement>(null);
  const chartIn = useRef<InstanceType<typeof Chart>>();
  const { data, isLoading } = useSWR(
    [Statistics.SYNC_TASK_INFO],
    ([]) =>
      Statistics.sync_task_info({
        date: [dayjs().subtract(1, "year").valueOf(), dayjs().valueOf()],
      })
  );

  useEffect(() => {
    if (!chartRef.current) return;

    let list: Array<Record<string, any>> = [];
    if (data?.data) {
      list = Object.keys(data.data.list).map((key) => {
        const obj: any = {}
        obj["运行次数"] = data.data?.list[key];
        obj["日期"] = key;
        return obj;
      });
    }

    if (!chartIn.current) {
      chartIn.current = new Chart({
        container: chartRef.current,
        autoFit: true,
      });

      chartIn.current
        .interval()
        .data(list)
        .encode("x", "日期")
        .encode("y", "运行次数")
        .axis("x", {
          title: false,
        })
        .axis("y", {
          title: false,
        });

      chartIn.current.render();
    } else {
      chartIn.current.changeData(list);
    }
  }, [data]);

  return (
    <Row>
      <Col span={18}>
        <div ref={chartRef} style={{ height: 300 }}></div>
      </Col>
      <Col span={6}>
        <RankList title="同步任务运行次数" list={data?.data?.rank_list} />
      </Col>
    </Row>
  );
}

