use crate::{
    set_canister_map, CallResult, CanisterData, CanisterIdRecord, CanisterStatusResponse, Principal,
};

#[ic_cdk::update]
pub async fn get_canister_status() -> Result<CanisterData, String>{
    let arg = CanisterIdRecord {
        canister_id: ic_cdk::api::id(),
    };

    ic_cdk::println!("CANISTER STATUS OF CANISTER_ID IS BEING CHECKED {}", arg.canister_id.clone());

    let can_status: CallResult<(CanisterStatusResponse,)> =
        ic_cdk::call(Principal::management_canister(), "canister_status", (arg,)).await;

    ic_cdk::println!("status_of_canister {:?}", can_status);

    match can_status {
        Ok(result) => {
            let res = result.clone().0;

            let can_data = CanisterData {
                cycles: res.cycles.clone(),
                memory_consumed: res.memory_size.clone(),
                module_hash: res.module_hash.clone(),
            };

            let can_id = ic_cdk::api::id();

            let stored = set_canister_map(can_id, can_data.clone());

            ic_cdk::println!("after_storage_in_stableBTree {:?}", stored);

            Ok(can_data)
        }
        Err(err) => {
            ic_cdk::println!("this is error {:?}", err);
            Err(format!("this is error {:?}", err))
        }
    }
}

// #[ic_cdk::update]
// pub async fn get_canister_status() -> Result<(), String> {

//     let arg = CanisterIdRecord{
//         canister_id : ic_cdk::api::id()
//     };

//     let can_status: CallResult<(CanisterStatusResponse,)> =
//         ic_cdk::call(Principal::management_canister(), "canister_status", (arg,)).await;

//     match can_status {
//         Ok(result) => {
//             let res = result.clone().0;

//             let can_data = CanisterData {
//                 cycles: res.cycles.clone(),
//                 memory_consumed: res.memory_size.clone(),
//                 module_hash: res.module_hash.clone(),
//             };

//             let can_id = ic_cdk::api::id();

//             let stored = set_canister_map(can_id, can_data.clone());

//             match stored {
//                 Ok(stored) => Ok(stored),
//                 Err(err) => Err(format!("Err while storing {:?}", err)),
//             }
//             // Ok(stored)
//         }
//         Err(err) => Err(format!("Err while checking memory_size {:?}", err)),
//     }
// }
