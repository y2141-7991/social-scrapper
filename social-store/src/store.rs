use diesel_async::pooled_connection::deadpool::{Object, Pool};
use diesel_async::pooled_connection::AsyncDieselConnectionManager;
use diesel_async::{AsyncConnection, AsyncPgConnection};


pub type PgPool = Pool<AsyncPgConnection>;
pub type PgConn = Object<AsyncPgConnection>;

pub struct Store {
    pool: PgPool,
}

pub fn diesel_make_pg_pool(db_url: String) -> PgPool {
    let manager = AsyncDieselConnectionManager::<AsyncPgConnection>::new(db_url);
    let builder = Pool::builder(manager);

    builder.build().unwrap()
}

impl Store {
    pub fn new(database_url: String) -> Self {
        let pool = diesel_make_pg_pool(database_url);
        Store { pool }
    }
    pub async fn get_conn(&self) -> PgConn {
        self.pool.get().await.unwrap()
    }
}
