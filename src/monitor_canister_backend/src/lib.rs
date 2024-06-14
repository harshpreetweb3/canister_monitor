mod canister_status;
mod memory;
mod state;
mod store_canister_status;
mod types;

mod get_canister_status;

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

const N: Duration = Duration::from_secs(5);

// async fn ring() -> String {
//     ic_cdk::println!("Rust Timer Ring!");
//     "Ring".to_string()
// }

#[ic_cdk::init]
async fn init() {
    ic_cdk_timers::set_timer_interval(N, || {
        ic_cdk::spawn(get_canister_status());
    });
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

export_candid!();

// get remaining cycles from canister_status
//1. all matrices, fun for all mat..
//2. store in a stable memory
//3. each minute, call and store matric ; heartbeats
//4, show matric // 5 minute, 10 min , from  frontend | graph
//graph for each
//get storage, get cycles
