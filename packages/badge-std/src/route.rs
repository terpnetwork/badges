use cosmwasm_schema::cw_serde;

/// BadgeRoute is enum type to represent stargaze query route path
#[cw_serde]
pub enum BadgeRoute {
    Alloc,
    Claim,
    Distribution,
}