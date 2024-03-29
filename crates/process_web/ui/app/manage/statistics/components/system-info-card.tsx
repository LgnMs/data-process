"use client";

import React, { useRef } from "react";
import { Card } from "antd";
import useSWR from "swr";
import * as Statistics from "@/api/statistics";

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

  return (
    <Card
      bordered={false}
      style={{ width: "100%" }}
      styles={{ body: { padding: "20px 24px 8px" } }}
      ref={cardRef}
      loading={isLoading}
    >
      <div>性能监测</div>
      <div>物理内存: {bytesToGB(data?.data?.total_memory).toFixed(2)} GB</div>
      <div>已使用内存: {bytesToGB(data?.data?.used_memory).toFixed(2)} GB</div>
      <div>已使用的交换: {data?.data?.used_swap} 字节</div>
      <div>
        CPU: {data?.data?.cpu_uses.length}个{" "}
        {data?.data?.cpu_uses.map((item) => (item ? `${item}% ` : "0% "))}
      </div>
      <div
        style={{
          paddingTop: 8,
          marginTop: 8,
          borderTop: "1px solid rgba(5, 5, 5, 0.06)",
        }}
      >
        APP监测{" "}
        <span style={{ paddingLeft: 12 }}>
          CPU: {data?.data?.processes_cpu_usage} % 内存:{" "}
          {bytesToMB(data?.data?.processes_memory_usage).toFixed(2)} MB 磁盘: 读{" "}
          {bytesToMB(data?.data?.processes_disk_usage.read_bytes)} MB 写{" "}
          {bytesToMB(data?.data?.processes_disk_usage.written_bytes)} MB
        </span>
      </div>
    </Card>
  );
}
