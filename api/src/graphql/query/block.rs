use async_graphql::{Context, Object, Result};
use entity::block;

#[derive(Default)]
pub struct BlockQuery;

#[Object]
impl BlockQuery {
    async fn get_block_by_id(&self, ctx: &Context<'_>, id: String) -> Result<block::Model> {
        todo!()
    }

    async fn get_block_by_parent_id(
        &self,
        ctx: &Context<'_>,
        parent_id: String,
    ) -> Result<Vec<block::Model>> {
        todo!()
    }

    async fn get_block_by_user_id(
        &self,
        ctx: &Context<'_>,
        user_id: String,
    ) -> Result<Vec<block::Model>> {
        todo!()
    }
}
