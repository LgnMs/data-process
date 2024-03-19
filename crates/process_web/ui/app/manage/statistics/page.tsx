// 报表关键信息： 采集数据量，采集任务数，采集成功次数，同步数据量，同步任务数，同步成功次数，共享接口数，共享接口调用次数

import { Flex } from "antd";
import MainStat from "@/app/manage/statistics/components/main-stat";
import SectionOneStat from "@/app/manage/statistics/components/section-one-stat";
import SectionTwoStat from "@/app/manage/statistics/components/section-two-stat";
import SectionThreeStat from "@/app/manage/statistics/components/section-three-stat";
import SectionFourStat from "@/app/manage/statistics/components/section-five-stat";
import SectionFiveStat from "@/app/manage/statistics/components/section-five-stat";

export default function pages() {
  return (
    <Flex gap={12} style={{margin: 12}}>
      <Flex flex={3} vertical gap={12}>
        <MainStat />
        <Flex gap={12}>
          <Flex flex={1}>
            <SectionOneStat />
          </Flex>
          <Flex flex={2}>
            <SectionTwoStat />
          </Flex>
        </Flex>
      </Flex>
      <Flex flex={1} gap={12} vertical>
        <SectionThreeStat />
        <SectionFourStat />
        <SectionFiveStat />
      </Flex>
    </Flex>
  );
}
