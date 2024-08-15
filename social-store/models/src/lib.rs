pub mod enums;
pub mod query;
pub mod schema;
pub mod social_account;
pub mod social_account1;

use diesel_async::pooled_connection::deadpool::Object;
use diesel_async::AsyncPgConnection;
pub type PgConn = Object<AsyncPgConnection>;
