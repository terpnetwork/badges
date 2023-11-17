use cosmwasm_schema::cw_serde;

use cosmwasm_std::CustomQuery;

#[cw_serde]
pub enum BadgeQuery {}

impl CustomQuery for BadgeQuery {}