// pub type VMem = VirtualMemory<DefaultMemoryImpl>;

// pub type CheckingData = StableBtreeMap<StoredPrincipal, Candid<Vec<DataCheck>>, VMem>;

// pub struct State {
//     pub checking_data: CheckingData,
//    }

// thread_local! {
//     static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> = RefCell::new(
//         MemoryManager::init(DefaultMemoryImpl::default())
//     );

//     static STATE: RefCell<State> = RefCell::new(
//         MEMORY_MANAGER.with(|mm| State {
//                         checking_data: CheckingData::init(mm.borrow().get(CHECKING_DATA_MEMORY_ID)),
//     );
// }

// pub fn read_state<R>(f: impl FnOnce(&State) -> R) -> R {
//     STATE.with(|cell| f(&cell.borrow()))
// }

// pub fn mutate_state<R>(f: impl FnOnce(&mut State) -> R) -> R {
//     STATE.with(|cell| f(&mut cell.borrow_mut()))
// }

// #[derive(Default)]
// pub struct Candid<T>(pub T)
// where
//     T: CandidType + for<'de> Deserialize<'de>;

// impl<T> Candid<T>
// where
//     T: CandidType + for<'de> Deserialize<'de>,
// {
//     pub fn to_bytes(&self) -> Cow<'_, [u8]> {
//         Cow::Owned(candid::encode_one(&self.0).expect("encoding should always succeed"))
//     }

//     pub fn from_bytes(bytes: Cow<'_, [u8]>) -> Self {
//         Self(candid::decode_one(bytes.as_ref()).expect("decoding should succeed"))
//     }
// }

// impl<T> Storable for Candid<T>
// where
//     T: CandidType + for<'de> Deserialize<'de>,
// {
//     const BOUND: Bound = Bound::Unbounded;

//     fn to_bytes(&self) -> Cow<'_, [u8]> {
//         Self::to_bytes(self)
//     }

//     fn from_bytes(bytes: Cow<'_, [u8]>) -> Self {
//         Self::from_bytes(bytes)
//     }
// }

// #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, CandidType)]
// pub struct StoredPrincipal(pub Principal);

// impl Storable for StoredPrincipal {
//     const BOUND: Bound = Blob::<29>::BOUND;

//     fn to_bytes(&self) -> Cow<'_, [u8]> {
//         Cow::Owned(
//             Blob::<29>::try_from(self.0.as_slice())
//                 .expect("principal length should not exceed 29 bytes")
//                 .to_bytes()
//                 .into_owned(),
//         )
//     }

//     fn from_bytes(bytes: Cow<'_, [u8]>) -> Self {
//         Self(Principal::from_slice(
//             Blob::<29>::from_bytes(bytes).as_slice(),
//         ))
//     }
// }