mod msg;
mod query;
mod route;

pub const NATIVE_BOND_DENOM: &str = "uterp";
pub const NATIVE_FEE_DENOM: &str = "uthiol";
pub const NATIVE_TESTNET_BOND_DENOM: &str = "uterpx";
pub const NATIVE_TESTNET_FEE_DENOM: &str = "uthiolx";

// 3/11/2022 16:00:00 ET
pub const GENESIS_MINT_START_TIME: u64 = 1647032400000000000;

use cosmwasm_std::{coin, coins, Addr, BankMsg, Coin};
pub use msg::{
     create_fund_community_pool_msg,
    ClaimAction, BadgeMsg, BadgeMsgWrapper,
};

pub type Response = cosmwasm_std::Response<BadgeMsgWrapper>;
pub type SubMsg = cosmwasm_std::SubMsg<BadgeMsgWrapper>;
pub type CosmosMsg = cosmwasm_std::CosmosMsg<BadgeMsgWrapper>;

pub use query::BadgeQuery;
pub use route::BadgeRoute;


pub fn thiols(amount: impl Into<u128>) -> Vec<Coin> {
    coins(amount.into(), NATIVE_FEE_DENOM)
}

pub fn thiol(amount: impl Into<u128>) -> Coin {
    coin(amount.into(), NATIVE_FEE_DENOM)
}

pub fn send_msg(to_address: &Addr, amount: impl Into<u128>) -> BankMsg {
    BankMsg::Send {
        to_address: to_address.to_string(),
        amount: thiols(amount),
    }
}