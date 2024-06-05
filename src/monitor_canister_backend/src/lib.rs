mod canister_status;
mod store_canister_status;
mod types;
mod memory;
mod state;

use state::*;
use types::*;
use memory::*;

use candid::Principal;
use ic_cdk::{
    api::{call::CallResult, management_canister::{
        self,
        main::{canister_status, CanisterId, CanisterIdRecord, CanisterStatusResponse}, 
    }},
    export_candid,
};

#[ic_cdk::query]
fn greet(name: String) -> String {
    format!("Hello, {}!", name)
}

// get storage from canister_status 

#[ic_cdk::query]
async fn get_storage() -> Result<
    (management_canister::main::CanisterStatusResponse,),
    (ic_cdk::api::call::RejectionCode, String),
> {

    let arg = CanisterIdRecord {
        canister_id: Principal::from_text("ss2fx-dyaaa-aaaar-qacoq-cai").unwrap(),
    };

    let result: Result<
        (management_canister::main::CanisterStatusResponse,),
        (ic_cdk::api::call::RejectionCode, String),
    > = canister_status(arg).await;

    result
}

#[ic_cdk::query]
async fn get_storage_2() -> CallResult<(CanisterStatusResponse,)> {

    let id : CanisterId = Principal::from_text("br5f7-7uaaa-aaaaa-qaaca-cai").unwrap();

    ic_cdk::println!("id : {}", id);    

    let arg = CanisterIdRecord {
        canister_id: id,
    };

    let result: CallResult<(CanisterStatusResponse,)> = canister_status(arg).await;
    
    result
}

#[ic_cdk::query]
async fn get_storage_3(arg : CanisterIdRecord) -> CallResult<(CanisterStatusResponse,)> {
    let result: CallResult<(CanisterStatusResponse,)> = canister_status(arg).await;
    result
}

#[ic_cdk::update]
pub async fn get_canister_status(arg: CanisterIdRecord) -> CallResult<(CanisterStatusResponse,)> {
    ic_cdk::call(Principal::management_canister(), "canister_status", (arg,)).await
}




// get remaining cycles from canister_status 





//1. all matrices, fun for all mat..
//2. store in a stable memory
//3. each minute, call and store matric ; heartbeats
//4, show matric // 5 minute, 10 min , from  frontend | graph
//graph for each

//get storage, get cycles

export_candid!();
