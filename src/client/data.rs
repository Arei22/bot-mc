use crate::database::postgresql::PgPool;
use serenity::prelude::TypeMapKey;

pub struct PgPoolData;

impl TypeMapKey for PgPoolData {
    type Value = PgPool;
}
