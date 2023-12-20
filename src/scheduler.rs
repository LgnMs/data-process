use anyhow::Result;

/// 任务调度器，用于单次或定时执行任务
pub trait Scheduler {
    /// 执行单次任务
    fn exec() -> Result<()>;

    /// 根据Cron表达式执行任务
    fn exec_with_cron(cron: String) -> Result<()>;
}