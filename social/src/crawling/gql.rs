pub const TWITCH_BASE_URL: &str = "https://gql.twitch.tv/gql";
pub const PROFILE_GRAPHQL_FUNCTION: &str = "
query ChannelHome_Query($login: String!) {
    user(login: $login) {
    ...ProfileBanner_channel

    }
}

fragment ProfileBanner_channel on User {
    roles {
        isAffiliate
        isGlobalMod 
        isPartner 
        isSiteAdmin 
        isStaff
    }
    createdAt
    displayName
    id
    login
    profileImageURL(width: 300)
    followers { totalCount }
    description
    channel {
        __typename
        socialMedias 
        {
            __typename
            id
            name
            title
            url
        }
    }

}
";