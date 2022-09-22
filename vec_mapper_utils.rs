use elrond_wasm::{
    api::{ErrorApiImpl, StorageMapperApi},
    elrond_codec::{TopDecode, TopEncode},
    storage::mappers::VecMapper,
};

pub trait VecMapperUtils<SA, T>
where
    SA: StorageMapperApi,
    T: TopEncode + TopDecode + 'static,
{
    fn find_index(&self, item: &T) -> Option<usize>;
    fn has_item(&self, item: &T) -> bool;

    fn remove_item(&mut self, item: &T);
}

impl<SA, T> VecMapperUtils<SA, T> for VecMapper<SA, T>
where
    SA: StorageMapperApi,
    T: TopEncode + TopDecode + 'static + core::cmp::PartialEq,
{
    fn find_index(&self, item_to_find: &T) -> Option<usize> {
        for (index, curr_item) in self.iter().enumerate() {
            if &curr_item == item_to_find {
                return Option::Some(index + 1); // we add one because VecMapper index starts at 1, while iter starts at 0
            }
        }

        return Option::None;
    }

    /**
     * Panic if missing items
     */
    fn remove_item(&mut self, item: &T) {
        let opt_index = self.find_index(item);

        if let Some(index) = opt_index {
            self.swap_remove(index);
        } else {
            SA::error_api_impl().signal_error(b"Item not found in VecMapper")
        }
    }

    fn has_item(&self, item: &T) -> bool {
        return self.find_index(item).is_some();
    }
}
