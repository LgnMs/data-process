use sea_orm::*;

use crate::entity::collect_config;

pub struct CollectConfigService;

impl CollectConfigService {
    pub async fn list(
        db: &DbConn,
        page: u64,
        posts_per_page: u64,
    ) -> Result<(Vec<collect_config::Model>, u64), DbErr> {
        let paginator = collect_config::Entity::find()
            .order_by_asc(collect_config::Column::Id)
            .paginate(db, posts_per_page);

        let num_pages = paginator.num_pages().await?;

        // Fetch paginated posts
        paginator.fetch_page(page - 1).await.map(|p| (p, num_pages))
    }
}
