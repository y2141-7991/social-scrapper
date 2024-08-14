#[derive(diesel_derive_enum::DbEnum, Debug, Clone, PartialEq, Default)]
#[ExistingTypePath = "crate::schema::sql_types::SocialAccountEnum"]
#[DbValueStyle = "SCREAMING_SNAKE_CASE"]
pub enum SocialAccountEnum {
    Inactive,
    #[default]
    Active
}