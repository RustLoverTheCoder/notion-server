use async_graphql::{Context, InputObject, Object, Result, SimpleObject, ID};
use entity::block;

#[derive(Default)]
pub struct BlockMutation;

#[derive(InputObject, Clone, Debug)]
pub struct NewBlockInput {
    pub parent_id: Option<ID>,
    pub title: Option<String>,
    pub body: Option<String>,
    pub r#type: Option<String>,
    pub disabled: Option<bool>,
}

#[Object]
impl BlockMutation {
    async fn create_new_block(
        &self,
        ctx: &Context<'_>,
        new_block: NewBlockInput,
    ) -> Result<block::Model> {
        todo!()
    }

    async fn update_block_by_id(
        &self,
        ctx: &Context<'_>,
        id: ID,
        new_block: NewBlockInput,
    ) -> Result<block::Model> {
        todo!()
    }

    async fn delete_block_by_id(&self, ctx: &Context<'_>, id: ID) -> Result<bool> {
        todo!()
    }
}
