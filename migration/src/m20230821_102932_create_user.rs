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
                    .table(User::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(User::Id).uuid().not_null().primary_key())
                    .col(
                        ColumnDef::new(User::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(ColumnDef::new(User::UpdatedAt).timestamp_with_time_zone())
                    .col(ColumnDef::new(User::DeletedAt).timestamp_with_time_zone())
                    .col(
                        ColumnDef::new(User::Disabled)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .col(ColumnDef::new(User::Email).string())
                    .col(ColumnDef::new(User::AvatarUrl).string())
                    .col(ColumnDef::new(User::Locale).string().default("en"))
                    .col(ColumnDef::new(User::PhoneNumber).string())
                    .col(ColumnDef::new(User::PasswordHash).string())
                    .col(ColumnDef::new(User::EmailVerified).boolean().default(false))
                    .col(
                        ColumnDef::new(User::PhoneNumberVerified)
                            .boolean()
                            .default(false),
                    )
                    .col(ColumnDef::new(User::Metadata).json())
                    .col(ColumnDef::new(User::FirstName).string())
                    .col(ColumnDef::new(User::LastName).string())
                    .col(ColumnDef::new(User::SettingsId).uuid())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .drop_table(Table::drop().table(User::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum User {
    Table,
    Id,
    CreatedAt,
    UpdatedAt,
    DeletedAt,
    Disabled,
    Email,
    AvatarUrl,
    Locale,
    PhoneNumber,
    PasswordHash,
    EmailVerified,
    PhoneNumberVerified,
    Metadata,
    FirstName,
    LastName,
    SettingsId,
}
