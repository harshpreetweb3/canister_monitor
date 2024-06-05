use crate::types::CanisterData;
use crate::Memory;
use candid::Principal;
use ic_stable_structures::StableBTreeMap;

pub struct State {
    pub canister_data: StableBTreeMap<Principal, CanisterData, Memory>,
    // Generates aliases for file requests.
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
        data: CanisterData
    ) -> Result<(), String> {


        // let response = self.canister_data.entry(canister_id).or_insert_with(Vec::new).push(data);

        // state.entry(caller_id).or_insert_with(Vec::new).push(new_vc);

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

    // pub fn remove_user_post(&mut self, user_principal: Principal) -> Result<PostData, String> {
    //     let remove_success = self.postdata.remove(&user_principal);
    //     if let Some(remove) = remove_success {
    //         Ok(remove)
    //     } else {
    //         Err("no User found!".to_string())
    //     }
    // }
}

impl Default for State {
    fn default() -> Self {
        State::new()
    }
}

fn init_file_contents() -> StableBTreeMap<Principal, CanisterData, Memory> {
    StableBTreeMap::init(crate::memory::get_canister_data_memory())
}


// #[cfg(test)]
// mod tests {
//     use super::*;

//     fn get_principal() -> Principal {
//         Principal::from_text("bxquz-fu76r-igixs-bw537-mgkyg-h4goq-ldrwe-q4zg2-zxtqy-zupgm-nqe")
//             .unwrap()
//     }
//     fn generate_user_data() -> PostData {
//         let user_data = PostData {
//             title: "No".to_string(),
//             data: "New".to_string(),
//             created_by: "32334".to_string(),
//         };
//         return user_data;
//     }
//     #[test]
//     fn test_post_creation() {
//         let mut state = State::default();
//         let data = generate_user_data();
//         let response = state.set_post_data(get_principal(), data.clone());
//         match response {
//             Ok(res) => assert_eq!(res, ()),
//             Err(e) => assert_eq!(e, "Already uploaed the post of the User".to_string()),
//         }
//     }
//     #[test]
//     fn test_post_exist() {
//         let state = State::default();
//         let data = generate_user_data();
//         let user_data = state.get_post_data(get_principal());
//         match user_data {
//             Ok(res) => assert_eq!(res, data),
//             Err(e) => assert_eq!(e, "No data found".to_string()),
//         };
//     }
//     #[test]
//     fn test_post_exist_after_remove() {
//         let mut state = State::default();
//         let data = generate_user_data();
//         let user_data = state.remove_user_post(get_principal());
//         match user_data {
//             Ok(res) => assert_eq!(res, data),
//             Err(e) => assert_eq!(e, "no User found!".to_string()),
//         };
//     }
// }
