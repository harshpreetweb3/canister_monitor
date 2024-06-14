use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::DefaultMemoryImpl;
use std::cell::RefCell;

const CANISTER_MAP: MemoryId = MemoryId::new(4);

pub type Memory = VirtualMemory<DefaultMemoryImpl>;

thread_local! {
   
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> =
        RefCell::new(MemoryManager::init(DefaultMemoryImpl::default()));
}

pub fn get_canister_map_memory() -> Memory {
    MEMORY_MANAGER.with(|mem_manager|{
        mem_manager.borrow().get(CANISTER_MAP)
    })
}
