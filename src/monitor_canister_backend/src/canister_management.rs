use candid::Principal;

use ic_cdk::api::management_canister::main::{
    deposit_cycles, install_code, CanisterIdRecord, CanisterInstallMode, CanisterSettings,
    CreateCanisterArgument, InstallCodeArgument, UpdateSettingsArgument,
};

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

    let create_response: CallResult<(CanisterIdRecord,)> = ic_cdk::call(
        Principal::management_canister(),
        "create_canister",
        (args, INITIAL_CYCLES),
    )
    .await;

    match create_response {
        Ok(res) => {
            let res = res.0;

            ic_cdk::println!(
                "newly created canister has this id {}",
                res.canister_id.clone()
            );

            // Deposit initial cycles to the new canister
            // let _deposit_cycle = deposit_initial_cycles(res.canister_id.clone()).await?;

            // Install the code on the new canister

            let install_code = install_code_on_new_canister(res.canister_id.clone()).await;

            ic_cdk::println!(
                "result after installing code to new_canister {:?}",
                install_code
            );

            Ok(res.canister_id.clone())
        }
        Err(err) => Err(format!("this is error {:?}", err)),
    }
}

// async fn deposit_initial_cycles(canister_id: Principal) -> Result<(), String> {
//     let id = CanisterIdRecord { canister_id };

//     let deposit_result = deposit_cycles(id, INITIAL_CYCLES)
//         .await
//         .map_err(|(code, msg)| {
//             format!(
//                 "Failed to deposit cycles to canister {}: {}: {}",
//                 canister_id, code as u8, msg
//             )
//         });

//     ic_cdk::println!(
//         "result after depositing cycles to the new_canister {:?}",
//         deposit_result
//     );

//     deposit_result
// }

async fn install_code_on_new_canister(canister_id: Principal) -> Result<(), String> {
    let wasm_module : Vec<u8> = include_bytes!("/home/harshpreet-singh/Documents/new-project/monitor_canister/target/wasm32-unknown-unknown/release/monitor_canister_backend_dddddd.wasm.gz").to_vec();

    // ic_cdk::println!("wasm module in vector form {:?}", wasm_module);

    ic_cdk::println!("wasm module size: {}", wasm_module.len());

    let install_args = InstallCodeArgument {
        mode: CanisterInstallMode::Install,
        canister_id: canister_id.clone(),
        wasm_module,
        arg: vec![],
    };

    ic_cdk::println!("before call install_code fn");

    install_code(install_args).await.map_err(|(code, msg)| {
        format!(
            "Failed to install code on canister {}: {}: {}",
            canister_id, code as u8, msg
        )
    })?;

    ic_cdk::println!("after calling install_code fn");

    Ok(())
}

#[update]
pub async fn update_canister_roles(new_master_id: Principal) -> Result<(), String> {
    let current_canister_id = ic_cdk::api::id();

    ic_cdk::println!(
        "LESS SEE NVE BNE CANISTER DA CONTROLLER KON BAN REA {}",
        ic_cdk::api::caller()
    );

    let my_identity : Principal = Principal::from_text("b5pqo-yef5a-lut3t-kmrpc-h7dnp-v3d2t-ls6di-y33wa-clrtb-xdhl4-dae").unwrap();

    // Update the current canister to slave
    // update_settings(current_canister_id, false).await?;
    update_settings(
        current_canister_id,
        vec![new_master_id.clone(), current_canister_id, my_identity],
        false,
    )
    .await?;



    // Update the new canister to master
    // update_settings(new_master_id, true).await?;


    update_settings(
        new_master_id,
        vec![ic_cdk::api::caller(), new_master_id, my_identity],
        true,
    )
    .await?;

    Ok(())
}

async fn update_settings(
    canister_id: Principal,
    controllers: Vec<Principal>,
    is_master: bool,
) -> Result<(), String> {
    let readable_controllers: Vec<String> = controllers.iter().map(|p| p.to_text()).collect();

    ic_cdk::println!(
        "Updating controllers for canister {}: {:?}",
        canister_id.to_text(),
        readable_controllers
    );

    let settings = CanisterSettings {
        controllers: Some(controllers.clone()),
        ..Default::default()
    };
    
    let args = UpdateSettingsArgument {
        canister_id,
        settings,
    };

    ic_cdk::call(Principal::management_canister(), "update_settings", (args,))
        .await
        .map_err(|(code, msg)| {
            format!(
                "Failed to update canister settings: {}: {}",
                code as u8, msg
            )
        })?;

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

// Function to update settings and manage state
// async fn update_settings(canister_id: Principal, is_master: bool) -> Result<(), String> {
//     // Prepare settings with controllers field as a vector containing the caller's Principal

//     let who_is_becoming_controller = ic_cdk::api::caller();

//     ic_cdk::println!(
//         "who_is_becoming_controller : dekhde aan kon bnda controller canister da {}",
//         who_is_becoming_controller
//     );

//     let settings = CanisterSettings {
//         controllers: Some(vec![ic_cdk::api::caller()]), // Wrap caller Principal in a vector
//         ..Default::default()
//     };

//     // Prepare arguments for the call

//     let args = UpdateSettingsArgument {
//         canister_id,
//         settings,
//     };

//     // Make the update_settings call asynchronously

//     ic_cdk::call(Principal::management_canister(), "update_settings", (args,))
//         .await
//         .map_err(|(code, msg)| {
//             format!(
//                 "Failed to update canister settings: {}: {}",
//                 code as u8, msg
//             )
//         })?;

//     // Update your application state based on whether it's a master or slave

//     STATE.with(|state| {
//         let mut state = state.borrow_mut();
//         if is_master {
//             state.set_master(canister_id);
//         } else {
//             state.add_slave(canister_id);
//         }
//     });

//     Ok(())
// }
