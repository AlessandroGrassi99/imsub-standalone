//! `SeaORM` Entity. Generated by sea-orm-codegen 0.10.4

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "twitch_subscriptions")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false, unique)]
    pub twitch_id: u32,
    pub active: String,
    pub expired_at: Option<DateTime>,
    pub updated_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::twitch_users::Entity",
        from = "Column::TwitchId",
        to = "super::twitch_users::Column::TwitchId",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    TwitchUsers,
}

impl Related<super::twitch_users::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::TwitchUsers.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
