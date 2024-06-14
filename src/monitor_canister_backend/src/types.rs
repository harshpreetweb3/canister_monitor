use candid::{ CandidType, Decode, Encode, Nat};
use serde::Deserialize;
use std::borrow::Cow;
use ic_stable_structures::{storable::Bound,Storable};


#[derive( CandidType,PartialEq,Deserialize, Debug, Clone)]
pub struct CanisterData {
    pub cycles: Nat,
    pub memory_consumed: Nat,
    pub module_hash: Option<Vec<u8>>,
}

const MAX_VALUE_SIZE: u32 = 100;


impl Storable for CanisterData {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }

    const BOUND: Bound = Bound::Bounded {
        max_size: MAX_VALUE_SIZE,
        is_fixed_size: false,
    };
}


