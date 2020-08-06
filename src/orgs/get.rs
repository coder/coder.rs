imports!();

new_builder!(Org, Orgs, Member, Members);

use crate::client::GetQueryBuilder;

exec!(
    Org -> crate::models::Organization,
    Orgs -> Vec<crate::models::Organization>,

    Member -> crate::models::OrgMember,
    Members -> Vec<crate::models::OrgMember>,
);

from!(
    @GetQuery
        => Org,
        => Orgs,

    @Org
        => Member,
        => Members,
);

impl_macro!(
    @GetQuery
        /// Queries all orgs the user belongs to, or all if the user is a site admin.
        |-> orgs ["orgs"] -> Orgs,
        /// Queries an org by its id.
        |=> org  ["orgs"] -> Org  = id,

    @Org
        /// Queries all members in an organization.
        |-> members ["members"] -> Members,
        /// Queries a organization member by their id.
        |=> member  ["members"] -> Member  = id,
);
