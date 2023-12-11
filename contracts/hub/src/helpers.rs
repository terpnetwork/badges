use std::fmt;

use cosmwasm_std::{Addr, Api, BlockInfo, Deps, Storage, Coin};
use sha2::{Digest, Sha256};

use tea::{Tea, MintRule};

use crate::{
    error::ContractError,
    state::{KEYS, OWNERS},
};

/// Length of a serialized compressed public key
const ECDSA_COMPRESSED_PUBKEY_LEN: usize = 33;
/// Length of a serialized uncompressed public key
const ECDSA_UNCOMPRESSED_PUBKEY_LEN: usize = 65;

/// Each NFT's token id is simply the tea id and the serial separated by a pipe.
pub fn token_id(id: u64, serial: u64) -> String {
    format!("{}|{}", id, serial)
}

/// The message the user needs to sign to claim the tea under "by key" or "by keys" rule
pub fn message(id: u64, user: impl fmt::Display) -> String {
    format!("claim tea {} for user {}", id, user)
}

/// The hash function to be used to sign a message before signing it. Here we use SHA256.
/// https://docs.rs/sha2/latest/sha2/#usage
pub fn hash(msg: &str) -> Vec<u8> {
    let mut hasher = Sha256::new();
    hasher.update(msg.as_bytes());
    hasher.finalize().to_vec()
}

/// A helper function to help casting Option to String
pub fn stringify_option(opt: Option<impl fmt::Display>) -> String {
    opt.map_or_else(|| "undefined".to_string(), |value| value.to_string())
}

/// Casting Vec<Coin> to a string.
///
/// If there is no fund (i.e. empty Vec), return the string `[]`. This is because wasm module does
/// not allow an empty string as event attribute.
pub fn stringify_funds(funds: &[Coin]) -> String {
    if funds.is_empty() {
        return "[]".to_string();
    }
    funds
        .iter()
        .map(|coin| coin.to_string())
        .collect::<Vec<_>>()
        .join(",")
}

/// This is basically a wrapper of `api.secp256k1_verify`, but instead of taking raw bytes in the
/// form of `&[u8]`, it takes the pubkey and signature as hex-encoded strings, and the original
/// message before hashing.
pub fn assert_valid_signature(
    api: &dyn Api,
    pubkey: &str,
    message: &str,
    signature: &str,
) -> Result<(), ContractError> {
    let msg_hash_bytes = hash(message);
    let key_bytes = hex::decode(pubkey)?;
    let sig_bytes = hex::decode(signature)?;

    if api.secp256k1_verify(&msg_hash_bytes, &sig_bytes, &key_bytes)? {
        Ok(())
    } else {
        Err(ContractError::InvalidSignature)
    }
}

// Assert the tea is available to be minted.
// Throw an error if the mint deadline or the max supply has been reached.
pub fn assert_available(
    tea: &Tea,
    block: &BlockInfo,
    amount: u64,
) -> Result<(), ContractError> {
    if let Some(expiry) = tea.expiry {
        if block.time.seconds() > expiry {
            return Err(ContractError::Expired);
        }
    }

    if let Some(max_supply) = tea.max_supply {
        if tea.current_supply + amount > max_supply {
            return Err(ContractError::SoldOut);
        }
    }

    Ok(())
}

// Assert the tea it NOT available to be minted. Throw an error if it is available.
pub fn assert_unavailable(tea: &Tea, block: &BlockInfo) -> Result<(), ContractError> {
    match assert_available(tea, block, 1) {
        Ok(_) => Err(ContractError::Available),
        Err(_) => Ok(()),
    }
}

/// Assert that an account has not already minted a tea.
pub fn assert_eligible(store: &dyn Storage, id: u64, user: &str) -> Result<(), ContractError> {
    if !OWNERS.contains(store, (id, user)) {
        Ok(())
    } else {
        Err(ContractError::already_claimed(id, user))
    }
}

/// Assert that a tea indeed uses the "by minter" rule, and that the sender is the minter.
pub fn assert_can_mint_by_minter(tea: &Tea, sender: &Addr) -> Result<(), ContractError> {
    match &tea.rule {
        MintRule::ByMinter(minter) => {
            if minter != sender {
                Err(ContractError::NotMinter)
            } else {
                Ok(())
            }
        },
        rule => Err(ContractError::wrong_mint_rule("by_minter", rule)),
    }
}

/// Assert that a tea indeed uses the "by key" rule, and the signature was produced by signing the
/// correct message with the correct privkey.
pub fn assert_can_mint_by_key(
    api: &dyn Api,
    id: u64,
    tea: &Tea,
    owner: &str,
    signature: &str,
) -> Result<(), ContractError> {
    // the tea must use the "by key" minting rule
    let pubkey = match &tea.rule {
        MintRule::ByKey(key) => key,
        rule => return Err(ContractError::wrong_mint_rule("by_key", rule)),
    };

    // the signature must be valid
    let message = message(id, owner);
    assert_valid_signature(api, pubkey, &message, signature)?;

    Ok(())
}

/// Assert that a tea indeed uses the "by keys" rule, and that the signature was produced by
/// signing the correct message using a whitelisted privkey.
pub fn assert_can_mint_by_keys(
    deps: Deps,
    id: u64,
    tea: &Tea,
    owner: &str,
    pubkey: &str,
    signature: &str,
) -> Result<(), ContractError> {
    // the tea must use the "by keys" minting rule
    match &tea.rule {
        MintRule::ByKeys => (),
        rule => return Err(ContractError::wrong_mint_rule("by_keys", rule)),
    }

    // the key must be whitelisted
    if !KEYS.contains(deps.storage, (id, pubkey)) {
        return Err(ContractError::key_does_not_exist(id));
    }

    // the signature must be valid
    let message = message(id, owner);
    assert_valid_signature(deps.api, pubkey, &message, signature)?;

    Ok(())
}

/// Assert that a byte array is a valid secp256k1 public key.
///
/// Copied from cosmwasm-crypto:
/// https://github.com/CosmWasm/cosmwasm/blob/v1.1.9/packages/crypto/src/secp256k1.rs#L140-L151
///
/// Previously I attempted to use the `k256` library for pubkey validation.
/// But it did not work because `rand` is a non-optional dependency for `k256`.
pub fn assert_valid_secp256k1_pubkey(bytes: &[u8]) -> Result<(), ContractError> {
    let ok = match bytes.first() {
        Some(0x02) | Some(0x03) => bytes.len() == ECDSA_COMPRESSED_PUBKEY_LEN,
        Some(0x04) => bytes.len() == ECDSA_UNCOMPRESSED_PUBKEY_LEN,
        _ => false,
    };
    if ok {
        Ok(())
    } else {
        Err(ContractError::InvalidPubkey)
    }
}
