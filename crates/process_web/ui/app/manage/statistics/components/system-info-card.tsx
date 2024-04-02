"use client";

import React, { useRef } from "react";
import { Card, Progress, Statistic } from "antd";
import useSWR from "swr";
import * as Statistics from "@/api/statistics";

import styles from "./system-info-card.module.scss";

export default function SystemInfoCard() {
  const cardRef = useRef<any>(null);

  const { data, isLoading } = useSWR(
    [Statistics.GET_SYS_INFO],
    ([]) => Statistics.get_sys_info(),
    { refreshInterval: 1000 }
  );

  function bytesToGB(bytes?: number) {
    if (!bytes) return 0;
    return bytes / Math.pow(1024, 3);
  }

  function bytesToMB(bytes?: number | null) {
    if (!bytes) return 0;
    return bytes / Math.pow(1024, 2);
  }

  function getCPU(cpu_uses?: number[]) {
    if (!cpu_uses) return 0;
    let sum = cpu_uses?.reduce((pre, current) => {
      if (current) {
        return pre + current
      }
      return pre
    }, 0)

    return (sum / cpu_uses.length).toFixed(2)
  }

  function getMemory(use?: number, total?: number) {
    if (!use || !total) return 0;

    return ((use / total) * 100).toFixed(2)
  }

  return (
    <Card
      bordered={false}
      style={{ width: "100%" }}
      styles={{ body: { padding: "20px 24px 8px" } }}
      ref={cardRef}
      loading={isLoading}
      className={styles.sysinfo}
    >
      <div className={styles.title}>系统资源监测</div>
      <div className={styles.cpu}>
        <span>{getCPU(data?.data?.cpu_uses)}%</span>
        <span className={styles['cpu-sub']}> cpu</span>
      </div>
      <div style={{height: 50}}>
        内存 <span className={styles['cpu-sub']}>{bytesToGB(data?.data?.used_memory).toFixed(2)}/{bytesToGB(data?.data?.total_memory).toFixed(2)} GB</span>
        <Progress style={{width: '90%'}} percent={getMemory(data?.data?.used_memory, data?.data?.total_memory) as number} status="active" strokeColor={{ from: '#108ee9', to: '#87d068' }} />
      </div>
      {/* <div>物理内存: {bytesToGB(data?.data?.total_memory).toFixed(2)} GB</div>
      <div>已使用内存: {bytesToGB(data?.data?.used_memory).toFixed(2)} GB</div>
      <div>已使用的交换: {data?.data?.used_swap} 字节</div> */}
      {/* <div>
        CPU: {data?.data?.cpu_uses.length}个{" "}
        {data?.data?.cpu_uses.map((item) => (item ? `${item}% ` : "0% "))}
      </div> */}
      <div
        style={{
          paddingTop: 8,
          marginTop: 8,
          borderTop: "1px solid rgba(5, 5, 5, 0.06)",
        }}
      >
        应用
        <span style={{ paddingLeft: 12 }}>
          CPU: {data?.data?.processes_cpu_usage?.toFixed(2)} % 内存:
          {bytesToMB(data?.data?.processes_memory_usage).toFixed(2)} MB 
          {/* 磁盘: 读{" "}
          {bytesToMB(data?.data?.processes_disk_usage.read_bytes)} MB 写{" "}
          {bytesToMB(data?.data?.processes_disk_usage.written_bytes)} MB */}
        </span>
      </div>
    </Card>
  );
}
