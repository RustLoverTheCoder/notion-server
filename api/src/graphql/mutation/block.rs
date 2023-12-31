use async_graphql::{Context, InputObject, Object, Result};
use config::contants::DB;
use entity::{block, block::Entity as Block};
use sea_orm::{
    prelude::{Json, Uuid},
    ActiveModelTrait, EntityTrait, Set,
};

#[derive(Default)]
pub struct BlockMutation;

#[derive(InputObject, Clone, Debug)]
pub struct NewBlockInput {
    pub parent_id: Option<Uuid>,
    pub title: Option<Json>,
    pub body: Option<Json>,
    pub r#type: Option<String>,
}

#[Object]
impl BlockMutation {
    async fn create_new_block(
        &self,
        ctx: &Context<'_>,
        new_block: NewBlockInput,
    ) -> Result<block::Model> {
        let user_id = ctx.data::<Uuid>().unwrap();
        let db = DB.get().unwrap();
        let block_data: block::Model = block::Model {
            id: Uuid::new_v4(),
            parent_id: new_block.parent_id,
            title: new_block.title,
            body: new_block.body,
            r#type: new_block.r#type.unwrap_or("page".to_owned()),
            author_id: Some(user_id.to_owned()),
            disabled: false,
            created_at: chrono::Utc::now().with_timezone(
                &chrono::FixedOffset::east_opt(0).unwrap_or(chrono::FixedOffset::east(0)),
            ),
            updated_at: None,
            deleted_at: None,
        };
        let active_model = block::ActiveModel {
            id: Set(block_data.id.to_owned()),
            parent_id: Set(block_data.parent_id.to_owned()),
            title: Set(block_data.title.to_owned()),
            body: Set(block_data.body.to_owned()),
            r#type: Set(block_data.r#type.to_owned()),
            author_id: Set(block_data.author_id.to_owned()),
            disabled: Set(block_data.disabled.to_owned()),
            created_at: Set(block_data.created_at.to_owned()),
            ..Default::default()
        };
        let res = Block::insert(active_model).exec(db).await?;
        Ok(block::Model {
            id: res.last_insert_id,
            ..block_data
        })
    }

    async fn update_block_by_id(
        &self,
        ctx: &Context<'_>,
        id: Uuid,
        new_block: NewBlockInput,
    ) -> Result<block::Model> {
        let user_id = ctx.data::<Uuid>().unwrap();
        let db = DB.get().unwrap();
        let block_data = Block::find_by_id_and_author_id(id, user_id.to_owned())
            .one(db)
            .await?
            .ok_or("Block not found")?;
        let mut block_active: block::ActiveModel = block_data.into();

        if let Some(parent_id) = new_block.parent_id {
            block_active.parent_id = Set(Some(parent_id.to_owned()));
        }
        if let Some(title) = new_block.title {
            block_active.title = Set(Some(title.to_owned()));
        }
        if let Some(body) = new_block.body {
            block_active.body = Set(Some(body.to_owned()));
        }
        block_active.updated_at = Set(Some(
            (chrono::Utc::now().with_timezone(
                &chrono::FixedOffset::east_opt(0).unwrap_or(chrono::FixedOffset::east(0)),
            )),
        ));
        let block_update_data = block_active.update(db).await?;

        Ok(block_update_data)
    }

    async fn delete_block_by_id(&self, ctx: &Context<'_>, id: Uuid) -> Result<bool> {
        let user_id = ctx.data::<Uuid>().unwrap();
        let db = DB.get().unwrap();
        let block_data = Block::find_by_id_and_author_id(id, user_id.to_owned())
            .one(db)
            .await?
            .ok_or("Block not found")?;
        let mut block_active: block::ActiveModel = block_data.into();
        block_active.disabled = Set(true);
        block_active.deleted_at = Set(Some(
            (chrono::Utc::now().with_timezone(
                &chrono::FixedOffset::east_opt(0).unwrap_or(chrono::FixedOffset::east(0)),
            )),
        ));
        block_active.update(db).await?;

        Ok(true)
    }
}
