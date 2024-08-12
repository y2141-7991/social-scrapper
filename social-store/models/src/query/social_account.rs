use diesel::{query_dsl::methods::FilterDsl, ExpressionMethods};

use crate::{schema::social_account, social_account::SocialAccount, PgConn};


impl SocialAccount {
    pub async fn insert(&self, conn: &mut PgConn) -> Result<usize, diesel::result::Error> {
        use crate::schema::social_account::dsl;
        use diesel_async::RunQueryDsl;

        let query = diesel::insert_into(dsl::social_account).values(self);

        query.execute(conn).await
    }

    pub async fn find_social_profile_by_social_account_id(conn: &mut PgConn, social_account_id: (String, String)){
        use crate::schema::social_account::dsl as sa_dsl;
        use diesel_async::RunQueryDsl;

        let (social_name, social_id) = social_account_id;

        let query = sa_dsl::social_account.filter(sa_dsl::social_name.eq(social_name));

    }
    
}