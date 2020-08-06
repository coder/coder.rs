imports!();

new_builder!(
    /// Queries an org by its id.
    Org,
    /// Queries all orgs the user belongs to, or all if the user is a site admin.
    Orgs,
    /// Queries a organization member by their id.
    Member,
    /// Queries all members in an organization.
    Members,
);

use crate::client::GetQueryBuilder;

exec!(
    Org -> crate::models::Organization
    Orgs -> Vec<crate::models::Organization>

    Member -> crate::models::OrgMember
    Members -> Vec<crate::models::OrgMember>
);

from!(
    @GetQuery
        => Org
        => Orgs

    @Org
        => Member
        => Members
);

impl_macro!(
    @GetQuery
        |-> orgs ["orgs"] -> Orgs
        |=> org  ["orgs"] -> Org  = id

    @Org
        |-> members ["members"] -> Members
        |=> member  ["members"] -> Member  = id
);
