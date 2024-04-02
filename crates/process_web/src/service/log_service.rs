use sea_orm::{ColumnTrait, DbConn, DbErr, EntityTrait, QueryFilter};

use crate::entity::{collect_log, sync_log};

use super::{collect_log_service::CollectLogService, sync_log_service::SyncLogService};

pub struct LogService;

impl LogService {
    /// 查看符合条件的日志将其状态修改
    pub async fn reset_log_status(db: &DbConn, pre_status: i32, status: i32, msg: &str) -> Result<bool, DbErr> {
        let log1 = collect_log::Entity::find()
            .filter(collect_log::Column::Status.eq(pre_status))
            .all(db)
            .await?;

        for item in log1 {
            CollectLogService::update_by_id(db, item.id, collect_log::Model { status, running_log: msg.to_string(), ..Default::default() }).await?;
        }

        let log2 = sync_log::Entity::find()
            .filter(sync_log::Column::Status.eq(pre_status))
            .all(db)
            .await?;

        for item in log2 {
            SyncLogService::update_by_id(db, item.id, sync_log::Model { status, running_log: msg.to_string(), ..Default::default() }).await?;
        }

        Ok(true)
    }
}