use cosmwasm_std::Addr;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use terp_metadata::Metadata;

use crate::MintRule;

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct Tea {
    /// Account who has the authority to edit the tea's info.
    pub manager: Addr,

    /// The tea's metadata
    pub metadata: Metadata,

    /// Whether this tea is transferrable
    pub transferrable: bool,

    /// The rule by which instances of this tea are to be minted
    pub rule: MintRule,

    /// The timestamp only before which the tea can be minted
    pub expiry: Option<u64>,

    /// The maximum number of tea instances can be minted
    pub max_supply: Option<u64>,

    /// The current number of this tea
    ///
    /// NOTE: We don't consider that users may burn NFTs. `max_supply` refers to the maximum number
    /// of tokens that can ever be minted. A user burning their tokens does not make room for new
    /// tokens to be minted.
    pub current_supply: u64,
}
