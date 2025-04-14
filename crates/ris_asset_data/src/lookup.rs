use ris_error::prelude::*;

use crate::asset_id::AssetId;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct LookupId{
    index: usize,
}

pub struct Lookup<
    T,
    Constructor: Fn() -> RisResult<T>,
    Destructor: Fn(&mut T),
> {
    entries: Vec<LookupEntry<T>>,
    constructor: Constructor,
    destructor: Destructor,
}

struct LookupEntry<T> {
    asset_id: AssetId,
    count: usize,
    value: Option<T>,
}

impl<
    T,
    Constructor: Fn() -> RisResult<T>,
    Destructor: Fn(&mut T),
> Lookup<T, Constructor, Destructor> {
    pub fn new(constructor: Constructor, destructor: Destructor) -> Self {
        Self {
            entries: Vec::new(),
            constructor,
            destructor,
        }
    }

    pub fn alloc(&mut self, asset_id: AssetId) -> RisResult<LookupId> {
        match self.search_id(asset_id.clone()) {
            Some(id) => {
                let entry = &mut self.entries[id.index];
                entry.count += 1;
                Ok(id)
            },
            None => {
                let value = (self.constructor)()?;
                let new_entry = LookupEntry{
                    asset_id,
                    count: 1,
                    value: Some(value),
                };

                let empty_entry = self.entries.iter_mut()
                    .filter(|x| x.value.is_none())
                    .enumerate()
                    .next();

                match empty_entry {
                    Some((index, empty_entry)) => {
                        let id = LookupId{index};
                        *empty_entry = new_entry;
                        Ok(id)
                    },
                    None => {
                        let id = LookupId{
                            index: self.entries.len()
                        };
                        self.entries.push(new_entry);
                        Ok(id)
                    },
                }
            },
        }
    }

    pub fn free(&mut self, id: LookupId) {
        let Some(entry) = self.entries.get_mut(id.index) else {
            return;
        };

        entry.count = entry.count.saturating_sub(1);
        if entry.count > 0 {
            return;
        }

        if let Some(mut value) = entry.value.take() {
            (self.destructor)(&mut value)
        }
    }

    pub fn get(&self, id: LookupId) -> Option<&T> {
        self.entries
            .get(id.index)
            .map(|x| x.value.as_ref())
            .flatten()
    }

    pub fn search_id(&self, asset_id: AssetId) -> Option<LookupId> {
        self.entries.iter()
            .position(|x| x.asset_id == asset_id)
            .map(|x| LookupId{index: x})
    }
}
