pub mod schema;
pub mod social_account1;
pub mod social_account;
pub mod query;


use diesel_async::pooled_connection::deadpool::Object;
use diesel_async::AsyncPgConnection;
pub type PgConn = Object<AsyncPgConnection>;