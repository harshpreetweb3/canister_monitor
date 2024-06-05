use ic_cdk::{update, query};
use crate::{CanisterData, State, Principal};
use std::cell::RefCell;

thread_local! {
    static STATE: RefCell<State> = RefCell::new(State::new());
}

#[update(name = "insert_canister_info")]
fn insert(canister_id : Principal, canister_data: CanisterData) -> Result<(), String> {
    
    STATE.with(|state| {
        let mut state = state.borrow_mut();
        state.set_canister_data(canister_id, canister_data)
    })
}

#[query(name = "get_canister_info")]
fn fetch(canister_id : Principal) -> Result<CanisterData, String> {
    // let user_principal = caller();
    STATE.with(|state| {
        let state = state.borrow();
        state.get_canister_data(canister_id)
    })
}
