use diesel::{Queryable, Selectable};

use crate::schema::accounts;

#[derive(Queryable, Selectable, PartialEq, Debug)]
#[diesel(table_name = accounts)]
pub struct Account {
    pub id: i32,
    pub email: String,
    pub password: String,
}
