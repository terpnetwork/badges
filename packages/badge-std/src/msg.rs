use crate::route::BadgeRoute;
use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Coin, CosmosMsg, CustomMsg};
static MSG_DATA_VERSION: &str = "1.0.0";

/// BadgeMsg is an override of CosmosMsg::Custom to add support for Stargaze's custom message types
#[cw_serde]
pub struct BadgeMsgWrapper {
    pub route: BadgeRoute,
    pub msg_data: BadgeMsg,
    pub version: String,
}

impl From<BadgeMsgWrapper> for CosmosMsg<BadgeMsgWrapper> {
    fn from(original: BadgeMsgWrapper) -> Self {
        CosmosMsg::Custom(original)
    }
}

impl CustomMsg for BadgeMsgWrapper {}

#[cw_serde]
pub enum BadgeMsg {
    FundCommunityPool {
        amount: Vec<Coin>,
    },
}

#[cw_serde]
pub enum ClaimAction {
    #[serde(rename = "mint_nft")]
    MintNFT,
    #[serde(rename = "bid_nft")]
    BidNFT,
}


pub fn create_fund_community_pool_msg(amount: Vec<Coin>) -> CosmosMsg<BadgeMsgWrapper> {
    BadgeMsgWrapper {
        route: BadgeRoute::Distribution,
        msg_data: BadgeMsg::FundCommunityPool { amount },
        version: MSG_DATA_VERSION.to_owned(),
    }
    .into()
}
