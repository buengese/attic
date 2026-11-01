use sea_orm_migration::prelude::*;

use crate::database::entity::object::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20261101_000001_add_object_nar_id_index"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // `object.nar_id` is joined against by the orphan NAR reaper and checked
        // on every `nar` delete (ON DELETE CASCADE), but was never indexed.
        //
        // Since idx-nar-state-holders exists, SQLite (without ANALYZE stats)
        // drives the reaper's LEFT JOIN from that index and no longer builds an
        // automatic index on `object`, scanning the whole table once per NAR.
        // That is O(nars * objects) while holding the write lock.
        //
        // `if_not_exists` so installs that already added it by hand still migrate.
        manager
            .create_index(
                Index::create()
                    .name("idx-object-nar-id")
                    .table(Entity)
                    .col(Column::NarId)
                    .if_not_exists()
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_index(
                Index::drop()
                    .name("idx-object-nar-id")
                    .table(Entity)
                    .to_owned(),
            )
            .await
    }
}
