use crate::{
    social_account::{SocialAccount, SocialAccountNew},
    PgConn,
};
use diesel::{upsert::on_constraint, ExpressionMethods, QueryDsl, SelectableHelper};

impl SocialAccountNew {
    pub async fn upsert(&self, conn: &mut PgConn) -> Result<usize, diesel::result::Error> {
        use crate::schema::social_account::dsl;
        use diesel_async::RunQueryDsl;

        let query = diesel::insert_into(dsl::social_account)
            .values(self)
            .on_conflict(on_constraint("social_account_pkey"))
            .do_update()
            .set(self);
        query.execute(conn).await
    }
}

impl SocialAccount {
    pub async fn find_social_profile_by_social_account_id(
        conn: &mut PgConn,
        social_account_id: (String, String),
    ) {
        use crate::schema::social_account::dsl as sa_dsl;
        use diesel_async::RunQueryDsl;

        let (_social_name, _social_id) = social_account_id;

        let query = sa_dsl::social_account
            .filter(sa_dsl::social_name.eq(_social_name))
            .filter(sa_dsl::social_id.eq(_social_id))
            .select(SocialAccount::as_select())
            .load::<SocialAccount>(conn)
            .await
            .expect("");
        println!("{:?}", query);
    }
}
