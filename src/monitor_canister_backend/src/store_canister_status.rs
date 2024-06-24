use ic_cdk::{update, query};
use crate::{CanisterData, State, Principal};
use std::cell::RefCell;

thread_local! {

    static STATE: RefCell<State> = RefCell::new(State::new());
    
}

#[update(name = "set_canister_map")]
pub fn set_canister_map(
    canister_id: Principal,
    can_data: CanisterData,
) -> Result<(), String> {
    STATE.with(|state| {
        let mut state = state.borrow_mut();
        state.insert_canister_info(canister_id, can_data)
    })
}

#[query(name = "get_canister_map")]
fn get_canister_map(canister_id: Principal, time: u64) -> Option<CanisterData> {
    STATE.with(|state| {
        let state = state.borrow();
        state.get_canister_info(canister_id, time)
    })  
}

#[query(name = "get_all_timestamps")]
fn get_all_timestamps(canister_id: Principal) -> Vec<(u64, CanisterData)>  {
    STATE.with(|state| {
        let state = state.borrow();
        state.get_all_the_timestamps(canister_id).collect()
    })
}
