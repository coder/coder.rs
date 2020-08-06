imports!();

new_builder!(Org, Orgs, Member, Members);

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
