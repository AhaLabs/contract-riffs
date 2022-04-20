use near_sdk::{
    borsh::{BorshDeserialize, BorshSerialize},
    store::LazyOption,
    IntoStorageKey,
};

// pub mod lazy;

pub struct Component<T: BorshDeserialize + BorshSerialize> {
    state: LazyOption<T>,
}

impl<T> Component<T>
where
    T: BorshSerialize + BorshDeserialize,
{
    pub fn new<S: IntoStorageKey>(key: S) -> Self {
        Component {
            state: LazyOption::new(key, None),
        }
    }

    pub fn set(&mut self, value: Option<T>) {
        self.state.set(value)
    }

    pub fn get(&self) -> &Option<T> {
      self.state.get()
    }
}
