use serde::{Serialize, Deserialize};
// use serde::de::value::StrDeserializer;
// use sqlx::{PgPool, query_as, FromRow};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Todo {
    pub id: i32,
    pub title: String,
    pub completed: bool,
}

// #[derive(Deserialize, Serialize, Debug)]
// pub struct CreateTodo {
//     pub title: String,
// }
//
// #[derive(Deserialize, Serialize, Debug)]
// pub struct UpdateTodo {
//     // pub title: String,
//     pub completed: bool,
// }
//
// #[derive(Deserialize, Serialize, Debug)]
// pub struct Book {
//     pub id: i32,
//     pub title: String,
//     pub author: String,
//     pub description: String,
// }

// impl Todo {
//     pub fn new(id: i32, title: String, completed: bool) -> Self {
//         Self {
//             id,
//             title,
//             completed,
//         }
//     }
// }

// #[derive(Clone)]
// pub(crate) struct Store {
//     pub pool: PgPool,
// }
//
// impl Store {
//     pub fn new(pool: PgPool) -> Self {
//         Self { pool }
//     }
// }