use candid::Principal;

use ic_cdk::api::management_canister::main::{deposit_cycles, install_code, CanisterIdRecord, CanisterInstallMode, CanisterSettings, CreateCanisterArgument, InstallCodeArgument, UpdateSettingsArgument};

use crate::state::*;
use ic_cdk::api::call::CallResult;
use ic_cdk::update;
use std::cell::RefCell;

thread_local! {
    static STATE: RefCell<State> = RefCell::new(State::new());
}

const INITIAL_CYCLES: u128 = 1_000_000_000_000;

#[update]
pub async fn create_new_canister() -> Result<Principal, String> {
    let args = CreateCanisterArgument::default();

    let create_response: CallResult<(CanisterIdRecord,)> =
        ic_cdk::call(Principal::management_canister(), "create_canister", (args,)).await;

    match create_response {
        Ok(res) => {
            let res = res.0;

             // Deposit initial cycles to the new canister
             let _deposit_cycle = deposit_initial_cycles(res.canister_id.clone()).await?;

            // Install the code on the new canister
            let _install_code = install_code_on_new_canister(res.canister_id.clone()).await;

            Ok(res.canister_id.clone())
        }
        Err(err) => Err(format!("this is error {:?}", err)),
    }
}

async fn deposit_initial_cycles(canister_id: Principal) -> Result<(), String> {

    let id = CanisterIdRecord{
        canister_id
    };


    deposit_cycles(id, INITIAL_CYCLES).await.map_err(|(code, msg)| {
        format!("Failed to deposit cycles to canister {}: {}: {}", canister_id, code as u8, msg)
    })
}
async fn install_code_on_new_canister(canister_id: Principal) -> Result<(), String> {

    let wasm_module : Vec<u8> = include_bytes!("../../../.dfx/local/canisters/monitor_canister_backend/monitor_canister_backend.wasm").to_vec();
    

    let install_args = InstallCodeArgument {
        mode: CanisterInstallMode::Install,
        canister_id: canister_id.clone(),
        wasm_module,
        arg: vec![],
    };

    install_code(install_args).await.map_err(|(code, msg)| {
        format!("Failed to install code on canister {}: {}: {}", canister_id, code as u8, msg)
    })?;

    Ok(())
}

#[update]
pub async fn update_canister_roles(new_master_id: Principal) -> Result<(), String> {
    let current_canister_id = ic_cdk::api::id();

    // Update the current canister to slave
    update_settings(current_canister_id, false).await?;

    // Update the new canister to master
    update_settings(new_master_id, true).await?;

    Ok(())
}

// Function to update settings and manage state
async fn update_settings(canister_id: Principal, is_master: bool) -> Result<(), String> {
    // Prepare settings with controllers field as a vector containing the caller's Principal

    let who_is_becoming_controller = ic_cdk::api::caller();

    ic_cdk::println!("who_is_becoming_controller {}", who_is_becoming_controller);

    let settings = CanisterSettings {
        controllers: Some(vec![ic_cdk::api::caller()]), // Wrap caller Principal in a vector
        ..Default::default()
    };

    // Prepare arguments for the call

    let args = UpdateSettingsArgument {
        canister_id,
        settings,
    };

    // Make the update_settings call asynchronously

    ic_cdk::call(Principal::management_canister(), "update_settings", (args,))
        .await
        .map_err(|(code, msg)| {
            format!(
                "Failed to update canister settings: {}: {}",
                code as u8, msg
            )
        })?;

    // Update your application state based on whether it's a master or slave

    STATE.with(|state| {
        let mut state = state.borrow_mut();
        if is_master {
            state.set_master(canister_id);
        } else {
            state.add_slave(canister_id);
        }
    });

    Ok(())
}
