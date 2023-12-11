use std::collections::BTreeSet;

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use terp_metadata::Metadata;

use crate::{Tea, FeeRate, MintRule};

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct InstantiateMsg {
    /// The fee rate charged for when creating or editing tea, quoted in uthiol per byte
    pub fee_rate: FeeRate,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub enum SudoMsg {
    /// Set the fee rate for creating or editing tea. Callable by L1 governance.
    SetFeeRate {
        fee_rate: FeeRate,
    }
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
#[allow(clippy::large_enum_variant)]
pub enum ExecuteMsg {
    /// Create a new tea with the specified mint rule and metadata
    CreateTea {
        /// Manager is the account that can 1) change the tea's metadata, and 2) if using the "by
        /// keys" mint rule, whitelist pubkeys.
        ///
        /// TODO: Make mananger an optional parameter; setting it to None meaning no one can change
        /// the metadata. Also, allow transferring of manager power in the `edit_tea` method.
        ///
        /// NOTE: If using the "by keys" minting rule, manager cannot be None, because a manager is
        /// is needed to whitelist keys.
        manager: String,
        /// The tea's metadata, defined by the OpenSea standard
        metadata: Metadata,
        /// Whether this tea is transferrable
        transferrable: bool,
        /// The rule by which this tea is to be minted. There are three available rules; see the
        /// docs of `tea::MintRule` for details.
        rule: MintRule,
        /// A deadline only before which the tea can be minted.
        /// Setting this to None means there is no deadline.
        /// Can only be set once when creating the tea; cannot be changed later.
        expiry: Option<u64>,
        /// The maximum amount of tea that can be minted. Note, users burning minted tea does
        /// NOT free up slots for new tea to be minted.
        /// Setting this to None means there is no max supply.
        /// Can only be set once when creating the tea; cannot be changed later.
        max_supply: Option<u64>,
    },

    /// Edit the metadata of an existing tea; only the manager can call
    EditTea {
        id: u64,
        metadata: Metadata,
    },

    /// For a tea that uses the "by keys" mint rule, invoke this method to whitelist pubkeys.
    /// Only callable by the manager before the minting deadline or max supply has been reached.
    AddKeys {
        id: u64,
        /// NOTE: Use BTreeSet, because the order of items in a HashSet may not be deterministic.
        /// See: https://www.reddit.com/r/rust/comments/krgvcu/is_the_iteration_order_of_hashset_deterministic/
        keys: BTreeSet<String>,
    },

    /// Once a tea has expired or sold out, the whitelisted keys are no longer needed. Invoke this
    /// method to purge these keys from storage in order to reduce the chain's state size.
    PurgeKeys {
        id: u64,
        limit: Option<u32>,
    },

    /// Once a tea has expired or sold out, the list of users who have claimed it is no longer
    /// needed. Invoke this method to purge these user addresses in order to reduce the chain's
    /// state size.
    PurgeOwners {
        id: u64,
        limit: Option<u32>,
    },

    /// For a tea with the "by minter" mint rule, mint new tea to a set of owners.
    /// Can only be invoked by the designated minter.
    MintByMinter {
        id: u64,
        /// NOTE: User BTreeSet instead of HashSet, the same reason as discussed above
        owners: BTreeSet<String>,
    },

    /// For a tea with the "by key" mint rule, mint a tea to the specified owner.
    /// The caller must submit a signature to prove they have the minting key.
    MintByKey {
        id: u64,
        owner: String,
        signature: String,
    },

    /// For a tea with the "by keys" mint rule, mint a tea to the specified owner.
    /// The caller must submit a signature to prove they have one of the whitelisted minting keys.
    MintByKeys {
        id: u64,
        owner: String,
        pubkey: String,
        signature: String,
    },

    /// During deployment, once the NFT contract has been deployed, the developer informs Hub of the
    /// NFT contract's address.
    ///
    /// Can only be invoked once by the developer.
    ///
    /// Ideally, on a chain with permissionless contract deployment, we would have the Hub deploy
    /// the NFT contract, and get its address by parsing the reply. However, this doesn't work on
    /// chains with permissioned deployment such as Terp Network.
    SetNft {
        nft: String,
    },
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    /// The contract's configurations. Returns ConfigResponse
    Config {},

    /// Info about a tea. Returns TeaResponse
    Tea {
        id: u64,
    },

    /// Enumerate infos of all tea. Returns AllTeaResponse
    AllTea {
        start_after: Option<u64>,
        limit: Option<u32>,
    },

    /// Whether a pubkey can be used to mint a tea. Returns KeyResponse
    Key {
        id: u64,
        pubkey: String,
    },

    /// List all pubkeys that can be used to mint a tea. Returns KeysResponse
    Keys {
        id: u64,
        start_after: Option<String>,
        limit: Option<u32>,
    },

    /// Whether a user has claimed the specified tea. Returns OwnerResponse
    Owner {
        id: u64,
        user: String,
    },

    /// List a users that have claimed the specified tea. Returns OwnersResponse
    Owners {
        id: u64,
        start_after: Option<String>,
        limit: Option<u32>,
    },
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct ConfigResponse {
    pub developer: String,
    pub nft: String,
    pub tea_count: u64,
    pub fee_rate: FeeRate,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct TeaResponse {
    pub id: u64,
    pub manager: String,
    pub metadata: Metadata,
    pub transferrable: bool,
    pub rule: MintRule,
    pub expiry: Option<u64>,
    pub max_supply: Option<u64>,
    pub current_supply: u64,
}

impl From<(u64, Tea)> for TeaResponse {
    fn from(item: (u64, Tea)) -> Self {
        let (id, tea) = item;
        TeaResponse {
            id,
            manager: tea.manager.into(),
            metadata: tea.metadata,
            transferrable: tea.transferrable,
            rule: tea.rule,
            expiry: tea.expiry,
            max_supply: tea.max_supply,
            current_supply: tea.current_supply,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct AllTeaResponse {
    pub tea: Vec<TeaResponse>
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct KeyResponse {
    pub key: String,
    pub whitelisted: bool,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct KeysResponse {
    pub keys: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct OwnerResponse {
    pub user: String,
    pub claimed: bool,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct OwnersResponse {
    pub owners: Vec<String>,
}
