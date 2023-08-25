use async_graphql::{Context, Object, Result};
use config::contants::DB;
use entity::{block, block::Entity as Block};
use sea_orm::prelude::Uuid;

#[derive(Default)]
pub struct BlockQuery;

#[Object]
impl BlockQuery {
    async fn get_block_by_id(&self, ctx: &Context<'_>, id: Uuid) -> Result<Option<block::Model>> {
        let user_id = ctx.data::<Uuid>().unwrap();
        let db = DB.get().unwrap();
        let block_data = Block::find_by_id_and_author_id(id, user_id.to_owned())
            .one(db)
            .await?;
        Ok(block_data)
    }

    async fn get_block_by_parent_id(
        &self,
        ctx: &Context<'_>,
        parent_id: Uuid,
    ) -> Result<Vec<block::Model>> {
        let user_id = ctx.data::<Uuid>().unwrap();
        let db = DB.get().unwrap();
        let block_data = Block::find_by_author_id_and_parent_id(user_id.to_owned(), parent_id)
            .all(db)
            .await?;
        Ok(block_data)
    }

    async fn get_blocks(&self, ctx: &Context<'_>) -> Result<Vec<block::Model>> {
        let user_id = ctx.data::<Uuid>().unwrap();
        let db = DB.get().unwrap();
        let block_data = Block::find_by_author_id(user_id.to_owned()).all(db).await?;
        Ok(block_data)
    }
}
