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
                    .table(UserSetting::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(UserSetting::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(UserSetting::ColorScheme)
                            .string()
                            .not_null()
                            .default("system"),
                    )
                    .col(
                        ColumnDef::new(UserSetting::Language)
                            .text()
                            .not_null()
                            .default("en"),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .drop_table(Table::drop().table(UserSetting::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum UserSetting {
    Table,
    Id,
    ColorScheme,
    Language,
}
