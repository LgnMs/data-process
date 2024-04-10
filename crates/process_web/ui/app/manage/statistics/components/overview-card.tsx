"use client";
import { Chart } from "@antv/g2";
import { Card, Col, DatePicker, Row, Tabs, TabsProps, Typography } from "antd";
import { useEffect, useRef, useState } from "react";
import useSWR from "swr";
import * as Statistics from "@/api/statistics";
import dayjs, { Dayjs } from "dayjs";

import styles from "./overview.module.scss";

const { RangePicker } = DatePicker;

type PickerDate = [Dayjs | null, Dayjs| null]

export default function OverviewCard() {
  const cardRef = useRef<any>(null);
  let isLoading = false;
  const [date, setDate] = useState<PickerDate>([dayjs().subtract(1, "year"), dayjs()]);

  const items: TabsProps["items"] = [
    {
      key: "1",
      label: "采集任务",
      children: <CollectTask date={date} />,
    },
    {
      key: "2",
      label: "同步任务",
      children: <SyncTask date={date}/>,
    }
  ];
  return (
    <Card
      bordered={false}
      style={{ width: "100%" }}
      styles={{ body: { padding: "4px 24px 8px" } }}
      ref={cardRef}
      loading={isLoading}
    >
      <Tabs defaultActiveKey="1" items={items} tabBarExtraContent={
        {
          right: <RangePicker 
            value={date}
            onChange={(value) => {
              if (value) {
                setDate(value)
              } else {
                setDate([dayjs().subtract(1, "year"), dayjs()])
              }
            }}
          />
        }
      }/>
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
              <div style={{width: '90%'}}>
                <Typography.Text  className={styles.index}>{index + 1} </Typography.Text >
                <Typography.Text style={{width: '90%', verticalAlign: '-6px'}} ellipsis>{value.name}</Typography.Text>
              </div>
              <div style={{width: '10%', textAlign: 'right'}}>{value.num_items}</div>
            </div>
          );
        })}
      </div>
    </div>
  );
}

function CollectTask(props: {date: PickerDate}) {
  const chartRef = useRef<HTMLDivElement>(null);
  const chartIn = useRef<InstanceType<typeof Chart>>();
  const { data, isLoading } = useSWR(
    [Statistics.COLLECT_TASK_INFO_DAY_LIST, props.date],
    ([_, date]) => {
      if (date[0] && date[1]) {
        return Statistics.collect_task_info_day_list({
          date: [date[0].valueOf(), date[1].valueOf()],
        })
      }
    }
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
          labelFormatter: (d: any) => dayjs(d).format("DD"),
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

function SyncTask(props: {date: PickerDate}) {
  const chartRef = useRef<HTMLDivElement>(null);
  const chartIn = useRef<InstanceType<typeof Chart>>();
  const { data, isLoading } = useSWR(
    [Statistics.SYNC_TASK_INFO, props.date],
    ([_, date]) => {
      if (date[0] && date[1]) {
        return Statistics.sync_task_info({
          date: [date[0].valueOf(), date[1].valueOf()],
        })
      }

    }
  );

  useEffect(() => {
    if (!chartRef.current) return;

    let list: Array<Record<string, any>> = [];
    if (data?.data) {
      list = Object.keys(data.data.list).map((key) => {
        const obj: any = {}
        obj["运行次数"] = data.data?.list[key];
        obj["日期"] = key;
        obj["date"] = key;
        return obj;
      });
      list.sort((a, b) => dayjs(a.date).valueOf() - dayjs(b.date).valueOf());
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
          labelFormatter: (d: any) => dayjs(d).format("DD"),
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

