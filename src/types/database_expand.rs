#![cfg(feature = "database_expand")]
/// 数据库数据扩展，开启feature: database_expand后才有用

use sea_orm::FromJsonQueryResult;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize, FromJsonQueryResult)]
pub struct ListType<T>(pub Vec<T>);

// #[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize, FromJsonQueryResult)]
// pub struct U32List(pub Vec<u32>);

// #[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize, FromJsonQueryResult)]
// pub struct StringList(pub Vec<String>);
