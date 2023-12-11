// use cosmwasm_std::{Decimal, DepsMut, StdResult, Storage};
// use cw_storage_plus::Item;
// use terp_sdk::Response;

// use tea::FeeRate;

// use crate::{
//     contract::{CONTRACT_NAME, CONTRACT_VERSION},
//     state::{ALL_TEA, FEE_RATE},
// };

// const LEGACY_FEE_PER_BYTE: Item<Decimal> = Item::new("fee_per_byte");

// /// Date and time (GMT): Wednesday, December 31, 2022 11:59:59 PM
// const NEW_BADGE_3_EXPIRY: u64 = 1672531199;

// /// This is the new fee rate that will be updated to
// fn new_fee_rate() -> FeeRate {
//     FeeRate {
//         metadata: Decimal::from_ratio(200000u128, 1u128),
//         key: Decimal::from_ratio(10000u128, 1u128),
//     }
// }

// pub fn migrate(deps: DepsMut) -> StdResult<Response> {
//     // let new_fee_rate = new_fee_rate();

//     // // set separate fee rates for metadata and keys
//     // update_fee_rate(deps.storage, &new_fee_rate)?;

//     // // // extend the minting deadline for tea 3
//     // // update_tea_3_expiry(deps.storage)?;

//     // // set the contract version to v1.1.0
//     // cw2::set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

//     Ok(Response::new()
//         .add_attribute("action", "tea/hub/migrate"))
//     //     .add_attribute("from_version", "1.0.0")
//     //     .add_attribute("to_version", "1.1.0")
//     //     .add_attribute("metadata_fee_rate", new_fee_rate.metadata.to_string())
//     //     .add_attribute("key_fee_rate", new_fee_rate.key.to_string()))
// }

// // fn update_fee_rate(store: &mut dyn Storage, fee_rate: &FeeRate) -> StdResult<()> {
// //     LEGACY_FEE_PER_BYTE.remove(store);
// //     FEE_RATE.save(store, fee_rate)
// // }

// // fn update_tea_3_expiry(store: &mut dyn Storage) -> StdResult<()> {
//     ALL_TEA.update(
//         store,
//         3,
//         |opt| {
//             let mut tea = opt.unwrap();
//             tea.expiry = Some(NEW_BADGE_3_EXPIRY);
//             Ok(tea)
//         },
//     )
//     .map(|_| ())
// }