use elrond_wasm::{
    api::ManagedTypeApi,
    types::{ManagedVec, ManagedVecItem},
};

pub trait EqUtils<M, T>
where
    M: ManagedTypeApi,
    T: ManagedVecItem,
{
    /// Check if two unordered vec are equals
    fn eq_unorder(&self, other: &Self) -> bool;
}

impl<M, T> EqUtils<M, T> for ManagedVec<M, T>
where
    M: ManagedTypeApi,
    T: ManagedVecItem + PartialEq + Clone,
{
    fn eq_unorder(&self, other: &Self) -> bool {
        if self.len() != other.len() {
            return false;
        }

        let mut other_copy: ManagedVec<M, T> = other.clone();

        for item in self.into_iter() {
            let opt_item_index = other_copy.find(&item);

            if opt_item_index.is_none() {
                return false;
            }

            other_copy.remove(opt_item_index.unwrap());
        }

        return other_copy.len() == 0;
    }
}
