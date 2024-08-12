use diesel::query_dsl::methods::{FilterDsl, SelectDsl};
use diesel::{ExpressionMethods, Queryable, Selectable};
use diesel_async::pooled_connection::deadpool::{Object, Pool};
use diesel_async::pooled_connection::AsyncDieselConnectionManager;
use diesel_async::{AsyncConnection, AsyncPgConnection, RunQueryDsl};

use crate::schema::accounts;

#[derive(Queryable, Selectable, PartialEq, Debug)]
#[diesel(table_name = accounts)]
pub struct Account {
    pub id: i32,
    pub email: String,
    pub password: String,
}