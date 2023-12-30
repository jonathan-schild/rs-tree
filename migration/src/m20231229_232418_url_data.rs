use sea_orm_migration::prelude::*;

use crate::m20231229_022203_user_table::User;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Url::Url)
                    .if_not_exists()
                    .col(ColumnDef::new(Url::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(Url::Short).string().not_null())
                    .col(ColumnDef::new(Url::Target).string())
                    .col(ColumnDef::new(Url::Author).uuid().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .from(Url::Url, Url::Author)
                            .to(User::User, User::Id),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(TreeEntry::TreeEntry)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(TreeEntry::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(TreeEntry::Name).string().not_null())
                    .col(ColumnDef::new(TreeEntry::Target).string())
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Tree::Tree)
                    .if_not_exists()
                    .col(ColumnDef::new(Tree::UrlId).uuid().not_null())
                    .col(ColumnDef::new(Tree::TreeEntryId).uuid().not_null())
                    .primary_key(Index::create().col(Tree::UrlId).col(Tree::TreeEntryId))
                    .foreign_key(
                        ForeignKey::create()
                            .from(Tree::Tree, Tree::TreeEntryId)
                            .to(TreeEntry::TreeEntry, TreeEntry::Id),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(Tree::Tree, Tree::UrlId)
                            .to(Url::Url, Url::Id),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Url::Url).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Url {
    Url,
    Id,
    Short,
    Target,
    Author,
}

#[derive(DeriveIden)]
pub enum TreeEntry {
    TreeEntry,
    Id,
    Name,
    Target,
}

#[derive(DeriveIden)]
pub enum Tree {
    Tree,
    UrlId,
    TreeEntryId,
}
