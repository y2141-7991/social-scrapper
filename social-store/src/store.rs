use diesel::query_dsl::methods::{FilterDsl, SelectDsl};
use diesel::{ExpressionMethods, Queryable, Selectable};
use diesel_async::pooled_connection::deadpool::{Object, Pool};
use diesel_async::pooled_connection::AsyncDieselConnectionManager;
use diesel_async::{AsyncConnection, AsyncPgConnection, RunQueryDsl};
use error_stack::{Report, ResultExt};

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
    pub async fn load_data(&self) {
        let mut conn = self.get_conn().await;
        let data = accounts::table
            .filter(accounts::id.gt(1))
            .load::<Account>(&mut conn)
            .await
            .unwrap();
        println!("{:?}", data);
    }
}

diesel::table! {
    accounts {
        id -> Integer,
        email -> Text,
        password -> Text,
    }
}

#[derive(Queryable, Selectable, PartialEq, Debug)]
#[diesel(table_name = accounts)]
pub struct Account {
    pub id: i32,
    pub email: String,
    pub password: String,
}
