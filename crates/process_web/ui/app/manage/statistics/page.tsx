// 报表关键信息： 采集数据量，采集任务数，采集成功次数，同步数据量，同步任务数，同步成功次数，共享接口数，共享接口调用次数
import { Col, Flex, Grid, Row } from "antd";
import MainStat from "@/app/manage/statistics/components/main-stat";
import SectionOneStat from "@/app/manage/statistics/components/section-one-stat";
import SectionTwoStat from "@/app/manage/statistics/components/section-two-stat";
import SectionThreeStat from "@/app/manage/statistics/components/section-three-stat";
import SectionFourStat from "@/app/manage/statistics/components/section-four-stat";
import SectionFiveStat from "@/app/manage/statistics/components/section-five-stat";
import styles from "./page.module.scss";

export default function pages() {
  return (
    <div className={styles['statistics-main']}>
      <div className={styles['statistics-left']}>
        <div className={styles['statistics-left__top']}>
          <MainStat />
        </div>
        <div className={styles['statistics-left__bottom']}>
          <SectionOneStat />
          <SectionTwoStat />
        </div>
      </div>
      <div className={styles['statistics-right']}>
        <div className={styles['statistics-right__item']}><SectionThreeStat /></div>
        <div className={styles['statistics-right__item']}><SectionFourStat /></div>
        <div className={styles['statistics-right__item']}><SectionFiveStat /></div>
      </div>
    </div>
  );
}
