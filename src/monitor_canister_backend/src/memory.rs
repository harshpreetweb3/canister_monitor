use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::DefaultMemoryImpl;
use std::cell::RefCell;
// use std::ptr::metadata;

const CANISTER_DATA: MemoryId = MemoryId::new(0);
const CANISTER_MAP : MemoryId = MemoryId::new(1);

pub type Memory = VirtualMemory<DefaultMemoryImpl>;

thread_local! {
    // The memory manager is used for simulating multiple memories. Given a `MemoryId` it can
    // return a memory that can be used by stable structures.
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> =
        RefCell::new(MemoryManager::init(DefaultMemoryImpl::default()));
}

pub fn get_canister_data_memory() -> Memory {
    MEMORY_MANAGER.with(|m| m.borrow().get(CANISTER_DATA))
}

pub fn get_canister_map_memory() -> Memory {
    MEMORY_MANAGER.with(|mem_manager|{
        mem_manager.borrow().get(CANISTER_MAP)
    })
}
