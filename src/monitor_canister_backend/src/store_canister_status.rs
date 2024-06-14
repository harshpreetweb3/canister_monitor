use crate::{CanisterData, Principal, State};
use ic_cdk::{query, update};
use std::cell::RefCell;
use ic_cdk_timers::set_timer;

thread_local! {
    static STATE: RefCell<State> = RefCell::new(State::new());
}

#[update(name = "set_canister_map")]
fn set_canister_map(
    canister_id: Principal,
    can_data: CanisterData,
) -> Result<(), String> {
    STATE.with(|state| {
        let mut state = state.borrow_mut();
        state.insert_canister_info(canister_id, can_data)
    })
}

#[update(name = "set_canister_map_2")]
fn set_canister_map_2(
    canister_id: Principal,
    can_data: CanisterData,
) -> Option<CanisterData> {
    STATE.with(|state| {
        let mut state = state.borrow_mut();
        state.insert_canister_info_2(canister_id, can_data)
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

// set_timer(Duration::from_secs(1), || ic_cdk::println!("Hello from the future!"));