use crate::types::CanisterData;
use crate::Memory;
use candid::Principal;
use ic_stable_structures::StableBTreeMap;

pub struct State {
    pub canister_data: StableBTreeMap<Principal, CanisterData, Memory>,
}
impl State {
    pub fn new() -> Self {
        Self {
            canister_data: init_file_contents(),
        }
    }

    pub fn set_canister_data(
        &mut self,
        canister_id: Principal,
        data: CanisterData,
    ) -> Result<(), String> {
        let response = self.canister_data.insert(canister_id, data);

        if let Some(_res) = response {
            Err("Not Able to upload canister data".to_string())
        } else {
            Ok(())
        }
    }

    pub fn get_canister_data(&self, canister_id: Principal) -> Result<CanisterData, String> {
        let canister_data = self.canister_data.get(&canister_id);

        if let Some(data) = canister_data {
            Ok(data)
        } else {
            Err("No Canister Data Exist!".to_string())
        }
    }
}

impl Default for State {
    fn default() -> Self {
        State::new()
    }
}

fn init_file_contents() -> StableBTreeMap<Principal, CanisterData, Memory> {
    StableBTreeMap::init(crate::memory::get_canister_data_memory())
}

