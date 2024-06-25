use crate::types::CanisterData;
use crate::Memory;
use candid::Principal;
use ic_stable_structures::StableBTreeMap;
use ic_cdk::api::time;

pub struct State {
    pub canister_map: StableBTreeMap<(Principal, u64), CanisterData, Memory>,
    pub master_canister: StableBTreeMap<u8, Principal, Memory>,
    pub slave_canisters: StableBTreeMap<Principal, u8, Memory>
}
impl State {
    pub fn new() -> Self {
        Self {
            canister_map: init_canister_map(),
            master_canister: init_master_canister(),
            slave_canisters: init_slave_canisters(),
        }
    }

    pub fn set_master(&mut self, canister_id: Principal) {

        ic_cdk::println!("new master canister formed {}", canister_id.clone());

        self.master_canister.insert(0, canister_id);
    }

    pub fn add_slave(&mut self, canister_id: Principal) {

        ic_cdk::println!("new slave canister formed {}", canister_id.clone());

        self.slave_canisters.insert(canister_id, 0);
    }

    pub fn get_master(&self) -> Option<Principal> {

        self.master_canister.get(&0)

    }

    pub fn get_slaves(&self) -> Vec<Principal> {

        self.slave_canisters.iter()
            .map(|(canister_id, _)| canister_id.clone())
            .collect()
            
    }

    pub fn insert_canister_info(
        &mut self,
        canister_id: Principal,
        can_data: CanisterData,
    ) -> Result<(), String> {
        let insertion_time = time();
        ic_cdk::println!("{}", insertion_time);
        let res = self
            .canister_map
            .insert((canister_id, insertion_time), can_data);

        if let Some(_can_data) = res {
            Err("err inserting canister info".to_string())
        } else {
            Ok(())
        }
    }

    pub fn get_canister_info(
        &self,
        canister_id: Principal,
        insertion_time: u64,
    ) -> Option<CanisterData> {
        self.canister_map.get(&(canister_id, insertion_time))
    }

    pub fn get_all_the_timestamps(
        &self,
        id: Principal,
    ) -> impl Iterator<Item = (u64, CanisterData)> + '_ {

        self.canister_map
            .range((id, u64::default())..)
            .take_while(move |((p, _), _)| *p == id)
            .map(|((_, s), t)| (s, t))
            
    }

}

impl Default for State {
    fn default() -> Self {
        State::new()
    }
}

fn init_canister_map() -> StableBTreeMap<(Principal, u64), CanisterData, Memory> {
    StableBTreeMap::init(crate::memory::get_canister_map_memory())
}

fn init_master_canister() -> StableBTreeMap<u8, Principal, Memory> {
    StableBTreeMap::init(crate::memory::get_master_canister_memory())
}

fn init_slave_canisters() -> StableBTreeMap<Principal, u8, Memory> {
    StableBTreeMap::init(crate::memory::get_slave_canisters_memory())
}