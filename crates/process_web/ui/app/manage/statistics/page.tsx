
/// 1. 总采集量
///     - 通过采集任务采集到的数据总量
///     - 时间维度下每日采集任务执行次数（面积图形式展示）
///     - 当日执行的采集任务次数
/// 2. 总访问数
///     - 共享接口被调用的次数
///     - 时间维度下每日被调用的次数（面积图形式展示）
///     - 当日被调用的次数
/// 3. 总同步次数
///     - 同步任务运行次数
///     - 时间维度下每日运行次数（面积图形式展示）
///     - 当日运行次数
/// 4. 性能监测
///     - CPU占比
///     - 内存占用
/// 5. 任务执行状况概览
///     - 年、月、日采集量统计
///     - 年、月、日同步任务运行次数统计
///     - 前x名运行次数的采集任务
///     - 前x名运行次数的同步任务
/// 6. 共享接口调用
///     - 访问用户数
///     - 人均调用次数
///     - 前x名接口调用
///         - 调用量
/// 7. 任务占比
///     - 三种类型任务再系统中的占比详情
import CollectTaskCard from "./components/collect-task-card";
import styles from "./page.module.scss";
import { Col, Row } from "antd";

export default function pages() {
  return (
    <Row gutter={12} className={styles[`statistics-main`]}>
      <Col span={6}>
        <CollectTaskCard />
      </Col>
      <Col span={6}>
        <CollectTaskCard />
      </Col>
      <Col span={6}>
        <CollectTaskCard />
      </Col>
      <Col span={6}>
        <CollectTaskCard />
      </Col>
    </Row>
  );
}
