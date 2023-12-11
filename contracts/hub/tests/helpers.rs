use cosmwasm_std::testing::mock_dependencies;
use cosmwasm_std::Addr;
use k256::ecdsa::VerifyingKey;
use terp_metadata::Metadata;

use tea_hub::error::ContractError;
use tea_hub::helpers::*;
use tea_hub::state::{KEYS, OWNERS};
use tea::{Tea, MintRule};

mod utils;

fn mock_tea(rule: Option<MintRule>, expiry: Option<u64>, max_supply: Option<u64>) -> Tea {
    Tea {
        manager: Addr::unchecked("larry"),
        metadata: Metadata::default(),
        transferrable: true,
        rule: rule.unwrap_or(MintRule::ByKeys),
        expiry,
        max_supply,
        current_supply: 99,
    }
}

#[test]
fn hashing() {
    let msg = "The quick brown fox jumps over the lazy dog";
    // hash generated at https://emn178.github.io/online-tools/sha256
    let msg_hash = "d7a8fbb307d7809469ca9abcb0082e4f8d5651e46d3cdb762d02d0bf37c9e592";
    let msg_hash_bytes = hex::decode(msg_hash).unwrap();
    assert_eq!(hash(msg), msg_hash_bytes);
}

/// Tea has no minting deadline or max supply
#[test]
fn asserting_availability_no_limit() {
    let tea = mock_tea(None, None, None);

    let env = utils::mock_env_at_timestamp(10000);

    // assert available should always succeed
    assert_eq!(assert_available(&tea, &env.block, 888), Ok(()));

    // assert unavailable should always fail
    assert_eq!(assert_unavailable(&tea, &env.block), Err(ContractError::Available));
}

/// Tea has a minting deadline but no max supply
#[test]
fn asserting_availability_deadline() {
    let tea = mock_tea(None, Some(10000), None);

    // deadline is not reached
    let env = utils::mock_env_at_timestamp(9999);
    assert_eq!(assert_available(&tea, &env.block, 888), Ok(()));
    assert_eq!(assert_unavailable(&tea, &env.block), Err(ContractError::Available));

    // deadline is reached
    let env = utils::mock_env_at_timestamp(10001);
    assert_eq!(assert_available(&tea, &env.block, 888), Err(ContractError::Expired));
    assert_eq!(assert_unavailable(&tea, &env.block), Ok(()));
}

/// Tea has a max supply but no minting limit
#[test]
fn asserting_availability_max_supply() {
    let env = utils::mock_env_at_timestamp(10000);

    // mock tea has a current supply of 99
    // set a max supply of 100
    // can mint one, but minting two should fail
    let mut tea = mock_tea(None, None, Some(100));
    assert_eq!(assert_available(&tea, &env.block, 1), Ok(()));
    assert_eq!(assert_available(&tea, &env.block, 2), Err(ContractError::SoldOut));
    assert_eq!(assert_unavailable(&tea, &env.block), Err(ContractError::Available));

    // set current cupply to 100
    tea.current_supply = 100;
    assert_eq!(assert_available(&tea, &env.block, 1), Err(ContractError::SoldOut));
    assert_eq!(assert_unavailable(&tea, &env.block), Ok(()));
}

#[test]
fn asserting_eligible() {
    let mut deps = mock_dependencies();

    let id = 1;
    let user = "larry";

    // user has not claimed
    {
        assert_eq!(assert_eligible(deps.as_ref().storage, id, user), Ok(()));
    }

    // user has already claimed
    {
        OWNERS.insert(deps.as_mut().storage, (id, user)).unwrap();
        assert_eq!(
            assert_eligible(deps.as_ref().storage, id, user),
            Err(ContractError::already_claimed(id, user)),
        );
    }
}

#[test]
fn asserting_user_can_mint() {
    let minter = Addr::unchecked("larry");
    let tea = mock_tea(Some(MintRule::ByMinter(minter.to_string())), None, None);

    // minter can mint
    {
        assert_eq!(assert_can_mint_by_minter(&tea, &minter), Ok(()));
    }

    // non-minter cannot mint
    {
        let non_minter = Addr::unchecked("jake");
        assert_eq!(assert_can_mint_by_minter(&tea, &non_minter), Err(ContractError::NotMinter));
    }
}

#[test]
fn asserting_can_mint_by_key() {
    let deps = mock_dependencies();

    let privkey = utils::mock_privkey();
    let pubkey = VerifyingKey::from(&privkey);
    let pubkey_str = hex::encode(pubkey.to_bytes());

    let rule = MintRule::ByKey(pubkey_str);
    let id = 1;
    let tea = mock_tea(Some(rule), None, None);

    let owner = "larry";
    let msg = message(id, owner);
    let signature = utils::sign(&privkey, &msg);

    // use the correct privkey, msg, and an unused salts
    {
        assert_eq!(assert_can_mint_by_key(deps.as_ref().api, id, &tea, owner, &signature), Ok(()));
    }

    // use the correct privkey but sign the wrong message
    {
        let false_msg = message(id, "jake");
        let signature = utils::sign(&privkey, &false_msg);
        assert_eq!(
            assert_can_mint_by_key(deps.as_ref().api, id, &tea, owner, &signature),
            Err(ContractError::InvalidSignature),
        );
    }

    // sign the correct msg but with the wrong privkey
    {
        let false_privkey = utils::random_privkey();
        let signature = utils::sign(&false_privkey, &msg);
        assert_eq!(
            assert_can_mint_by_key(deps.as_ref().api, id, &tea, owner, &signature),
            Err(ContractError::InvalidSignature),
        );
    }
}

#[test]
fn asserting_can_mint_by_keys() {
    let mut deps = mock_dependencies();

    let privkey = utils::mock_privkey();
    let pubkey = VerifyingKey::from(&privkey);
    let pubkey_bytes = pubkey.to_bytes().to_vec();
    let pubkey_str = hex::encode(&pubkey_bytes);

    let rule = MintRule::ByKeys;
    let id = 1;
    let tea = mock_tea(Some(rule), None, None);

    let owner = "larry";
    let msg = message(id, owner);
    let signature = utils::sign(&privkey, &msg);

    KEYS.insert(deps.as_mut().storage, (id, &pubkey_str)).unwrap();

    // use a whitelisted key and sign the correct message
    {
        assert_eq!(
            assert_can_mint_by_keys(deps.as_ref(), id, &tea, owner, &pubkey_str, &signature),
            Ok(()),
        );
    }

    // use a whitelisted key but sign the wrong message
    {
        let false_msg = "ngmi";
        let signature = utils::sign(&privkey, false_msg);
        assert_eq!(
            assert_can_mint_by_keys(deps.as_ref(), id, &tea, owner, &pubkey_str, &signature),
            Err(ContractError::InvalidSignature),
        );
    }

    // use the correct message but a non-whitelisted key
    {
        let false_privkey = utils::random_privkey();
        let false_pubkey = VerifyingKey::from(&false_privkey);
        let false_pubkey_str = hex::encode(false_pubkey.to_bytes());
        let signature = utils::sign(&false_privkey, &msg);
        assert_eq!(
            assert_can_mint_by_keys(deps.as_ref(), id, &tea, owner, &false_pubkey_str, &signature),
            Err(ContractError::key_does_not_exist(id)),
        );
    }
}
