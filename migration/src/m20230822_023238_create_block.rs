use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .create_table(
                Table::create()
                    .table(Block::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Block::Id).uuid().not_null())
                    .col(ColumnDef::new(Block::ParentId).uuid())
                    .col(ColumnDef::new(Block::Title).json())
                    .col(ColumnDef::new(Block::Body).json())
                    .col(
                        ColumnDef::new(Block::Type)
                            .string()
                            .not_null()
                            .default("page"),
                    )
                    .col(ColumnDef::new(Block::AuthorId).uuid())
                    .col(
                        ColumnDef::new(Block::Disabled)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .col(
                        ColumnDef::new(Block::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(ColumnDef::new(Block::UpdatedAt).timestamp_with_time_zone())
                    .col(ColumnDef::new(Block::DeletedAt).timestamp_with_time_zone())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .drop_table(Table::drop().table(Block::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Block {
    Table,
    Id,
    ParentId,
    Title,
    Body,
    Type,
    AuthorId,
    Disabled,
    CreatedAt,
    UpdatedAt,
    DeletedAt,
}
