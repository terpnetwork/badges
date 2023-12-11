use cosmwasm_std::Addr;
use cw_item_set::Set;
use cw_storage_plus::{Item, Map};

use tea::{Tea, FeeRate};

/// Address of the developer
pub const DEVELOPER: Item<Addr> = Item::new("owner");

/// Address of tea nft contract
pub const NFT: Item<Addr> = Item::new("nft");

/// The fee rate, in uthiol per byte, charged for storing data on-chain
pub const FEE_RATE: Item<FeeRate> = Item::new("fee_rate");

/// Total number of tea
pub const TEA_COUNT: Item<u64> = Item::new("tea_count");

/// All tea tokens, indexed by ids
pub const ALL_TEA: Map<u64, Tea> = Map::new("tea");

/// Pubkeys that are whitelisted to mint a tea
pub const KEYS: Set<(u64, &str)> = Set::new("keys");

/// User addresses that have already claimed a tea. If a composite key {tea_id, user_addr}
/// exists in the map, then this user has already claimed.
///
/// Note that we don't verify the addresses here. The verifification is done by the NFT contract.
pub const OWNERS: Set<(u64, &str)> = Set::new("claimed");
