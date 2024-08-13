#[derive(diesel_derive_enum::DbEnum, Debug, Clone, PartialEq)]
#[ExistingTypePath = "crate::schema::sql_types::SocialAccountStatus"]
pub enum SocialAccountStatus {
    Inactive,
    Active
}