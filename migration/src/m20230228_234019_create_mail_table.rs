use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Mail::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Mail::MessageId)
                            .text()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Mail::Subject).text().not_null())
                    .col(ColumnDef::new(Mail::Date).date().not_null())
                    .col(ColumnDef::new(Mail::From).text().not_null())
                    .col(ColumnDef::new(Mail::Recipients).text().not_null())
                    .col(ColumnDef::new(Mail::BelongsTo).text().not_null())
                    .col(ColumnDef::new(Mail::Content).text().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Mail::Table).to_owned())
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
enum Mail {
    Table,
    MessageId,
    Subject,
    Date,
    From,
    Recipients,
    /// An array that indicates which users on our server have this message in their mailbox
    BelongsTo,
    Content,
}
