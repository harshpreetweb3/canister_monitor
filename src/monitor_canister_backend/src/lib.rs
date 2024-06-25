mod state;
mod types;
mod memory;
mod canister_status;
mod get_canister_status;
mod store_canister_status;
mod canister_management;



use get_canister_status::get_canister_status;
use memory::*;
use state::*;
use store_canister_status::set_canister_map;
use types::*;

use candid::Principal;
use ic_cdk::{
    api::{
        call::CallResult,
        management_canister::{
            self,
            main::{canister_status, CanisterId, CanisterIdRecord, CanisterStatusResponse},
        },
    },
    export_candid,
};

use ic_cdk_timers;
use std::time::Duration;

const N: Duration = Duration::from_secs(150);

// const THRESHOLD: u64 = 350 * 1024 * 1024 * 1024; // 350GB in bytes

// const THRESHOLD: u64 = 20971520; // 20 MB in bytes

const THRESHOLD: u64 = 11534336;

// #[ic_cdk::init]
// async fn init() {
//     ic_cdk_timers::set_timer_interval(N, || {
//         ic_cdk::spawn(get_canister_status());
//     });
// }

#[ic_cdk::init]
async fn init() {
    ic_cdk_timers::set_timer_interval(N, || {
        ic_cdk::spawn(async {
            if let Err(err) = check_storage_threshold().await {
                ic_cdk::println!("Error in threshold check: {:?}", err);
            }
        });
    });
}

export_candid!();

async fn check_storage_threshold() -> Result<(), String> {
    let canister_status = get_canister_status().await?;
    let memory_used = canister_status.memory_consumed;

    if memory_used >= THRESHOLD {
        // Create a new master canister
        let new_canister_id = canister_management::create_new_canister().await?;
        canister_management::update_canister_roles(new_canister_id).await?;
    }

    Ok(())
}

























//without CallResult return type
// #[ic_cdk::query]
// async fn get_storage() -> Result<
//     (management_canister::main::CanisterStatusResponse,),
//     (ic_cdk::api::call::RejectionCode, String),
// > {
//     let arg = CanisterIdRecord {
//         canister_id: Principal::from_text("ss2fx-dyaaa-aaaar-qacoq-cai").unwrap(),
//     };
//     let result: Result<
//         (management_canister::main::CanisterStatusResponse,),
//         (ic_cdk::api::call::RejectionCode, String),
//     > = canister_status(arg).await;
//     result
// }

//without arg without intercanister-call
// #[ic_cdk::query]
// async fn get_storage_2() -> CallResult<(CanisterStatusResponse,)> {
//     let id : CanisterId = Principal::from_text("bkyz2-fmaaa-aaaaa-qaaaq-cai").unwrap();
//     ic_cdk::println!("id : {}", id);
//     let arg = CanisterIdRecord {
//         canister_id: id,
//     };
//     let result: CallResult<(CanisterStatusResponse,)> = canister_status(arg).await;
//     result
// }

//with arg but without intercanister-call
// #[ic_cdk::query]
// async fn get_storage_3(arg : CanisterIdRecord) -> CallResult<(CanisterStatusResponse,)> {
//     let result: CallResult<(CanisterStatusResponse,)> = canister_status(arg).await;
//     result
// }

//anybody will be able to check canister status
// #[ic_cdk::update]
// pub async fn get_canister_status(arg: CanisterIdRecord) -> CallResult<(CanisterStatusResponse,)> {
//     ic_cdk::call(Principal::management_canister(), "canister_status", (arg,)).await
// }

// get remaining cycles from canister_status
//1. all matrices, fun for all mat..
//2. store in a stable memory
//3. each minute, call and store matric ; heartbeats
//4, show matric // 5 minute, 10 min , from  frontend | graph
//graph for each
//get storage, get cycles
//timespan eg 30 days or 60 days 
//data of seconds

// params of get_canister_data
// no_of_secs
// timestamp
// return -> all the data between timestamp and timestamp - seconds

// O log n time complexity should be there; logic
