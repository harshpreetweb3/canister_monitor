use crate::{CallResult, CanisterId, CanisterIdRecord, CanisterStatusResponse, Principal, CanisterData};

// #[ic_cdk::update]
// pub async fn get_canister_status_for_this_canister_id() -> CallResult<(CanisterStatusResponse,)> {
//     let id: CanisterId = Principal::from_text("br5f7-7uaaa-aaaaa-qaaca-cai").unwrap();

//     ic_cdk::println!("id : {}", id);

//     let arg = CanisterIdRecord { canister_id: id };

//     ic_cdk::call(Principal::management_canister(), "canister_status", (arg,)).await
// }

// #[ic_cdk::update]
// pub async fn get_canister_status_for_some_canister_id(id : CanisterId) -> CallResult<(CanisterStatusResponse,)> {
//     // let id: CanisterId = Principal::from_text("br5f7-7uaaa-aaaaa-qaaca-cai").unwrap();

//     ic_cdk::println!("id : {}", id);

//     let arg = CanisterIdRecord { canister_id: id };

//     ic_cdk::call(Principal::management_canister(), "canister_status", (arg,)).await
// }

// #[ic_cdk::update]
// pub async fn get_memory_used_by_canister() -> Result<String, String> {
//     let id: CanisterId = Principal::from_text("br5f7-7uaaa-aaaaa-qaaca-cai").unwrap();

//     ic_cdk::println!("id : {}", id);

//     let arg = CanisterIdRecord { canister_id: id };

//     let status: CallResult<(CanisterStatusResponse,)> =
//         ic_cdk::call(Principal::management_canister(), "canister_status", (arg,)).await;

//     match status {
//         Ok(result) => {
//             let res = result.clone().0;
//             let memory_size = res.memory_size;

//             Ok(format!("memory size : {}", memory_size))
//         }
//         Err(err) => Err(format!("Err while checking memory_size {:?}", err)),
//     }
// }

// #[ic_cdk::update]
// pub async fn get_cycles_balance_of_canister() -> Result<String, String> {
//     let id: CanisterId = Principal::from_text("br5f7-7uaaa-aaaaa-qaaca-cai").unwrap();

//     ic_cdk::println!("id : {}", id);

//     let arg = CanisterIdRecord { canister_id: id };

//     let status: CallResult<(CanisterStatusResponse,)> =
//         ic_cdk::call(Principal::management_canister(), "canister_status", (arg,)).await;

//     match status {
//         Ok(result) => {
//             let res = result.clone().0;
//             let cycles = res.cycles;

//             Ok(format!("cycles : {}", cycles))
//         }
//         Err(err) => Err(format!("Err while checking cycles {:?}", err)),
//     }
// }

// #[ic_cdk::update]
// pub async fn get_module_hash_of_canister() -> Result<Vec<u8>, String> {
//     let id: CanisterId = Principal::from_text("br5f7-7uaaa-aaaaa-qaaca-cai").unwrap();

//     ic_cdk::println!("id : {}", id);

//     let arg = CanisterIdRecord { canister_id: id };

//     let status: CallResult<(CanisterStatusResponse,)> =
//         ic_cdk::call(Principal::management_canister(), "canister_status", (arg,)).await;

//     match status {
//         Ok(result) => {
//             let res = result.clone().0;
//             let module_hash = res.module_hash;

//             match module_hash {
//                 Some(hash) => Ok(hash),

//                 None => Err("unable to get memory hash".to_string()),
//             }

//             // Ok(format!("cycles : {}", cycles))
//         }
//         Err(err) => Err(format!("Err while checking cycles {:?}", err)),
//     }
// }


// #[ic_cdk::update]
// pub async fn get_canister_matrices() -> Result<CanisterData, String> {
//     let id: CanisterId = Principal::from_text("br5f7-7uaaa-aaaaa-qaaca-cai").unwrap();

//     ic_cdk::println!("id : {}", id);

//     let arg = CanisterIdRecord { canister_id: id };

//     let status: CallResult<(CanisterStatusResponse,)> =
//         ic_cdk::call(Principal::management_canister(), "canister_status", (arg,)).await;

//     match status {
//         Ok(result) => {
//             let res = result.clone().0;
//             let module_hash = res.module_hash;

//             match module_hash {
//                 Some(hash) => {

//                     let info = CanisterData{
//                         cycles : res.cycles.clone(),
//                         memory_consumed : res.cycles.clone(),
//                         module_hash : hash
//                     };

//                     Ok(info)

//                 },

//                 None => Err("unable to get memory hash".to_string()),
//             }

//             // Ok(format!("cycles : {}", cycles))
//         }
//         Err(err) => Err(format!("Err while checking cycles {:?}", err)),
//     }
// }

// ddate
// b5pqo-yef5a-lut3t-kmrpc-h7dnp-v3d2t-ls6di-y33wa-clrtb-xdhl4-dae