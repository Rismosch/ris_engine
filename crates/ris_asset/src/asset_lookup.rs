use ris_error::prelude::*;

use ris_asset_data::AssetId;
use ris_async::OneshotReceiver;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct LookupId{
    index: usize,
}

pub struct Lookup<T> {
    entries: Vec<LookupEntry<T>>,
}

struct LookupEntry<T> {
    asset_id: AssetId,
    count: usize,
    value: Option<LookUpValue<T>>,
}

enum LookUpValue<T> {
    IsLoading(OneshotReceiver<RisResult<T>>),
    Loaded(T),
}

impl<T> Lookup<T> {
    pub fn new() -> Self {
        Self {entries: Vec::new()}
    }

    pub fn alloc<F: FnOnce(AssetId) -> OneshotReceiver<RisResult<T>>>(
        &mut self,
        asset_id: AssetId,
        callback: F,
    ) -> RisResult<LookupId> {
        match self.search_id(asset_id.clone()) {
            Some(id) => {
                let entry = &mut self.entries[id.index];
                entry.count += 1;
                Ok(id)
            },
            None => {
                let receiver = callback(asset_id.clone());

                let new_entry = LookupEntry{
                    asset_id,
                    count: 1,
                    value: Some(LookUpValue::IsLoading(receiver)),
                };

                let empty_entry = self.entries.iter_mut()
                    .filter(|x| x.count == 0)
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

    pub fn free<F: FnOnce(&mut T)>(
        &mut self,
        id: LookupId,
        callback: F,
    ) {
        let Some(entry) = self.entries.get_mut(id.index) else {
            return;
        };

        entry.count = entry.count.saturating_sub(1);
        if entry.count > 0 {
            return;
        }

        if let Some(LookUpValue::Loaded(mut asset)) = entry.value.take() {
            callback(&mut asset);
        } else {
            ris_log::warning!("lookup references dropped to 0 but free callback could not be called. this may indicate a leak.");
        }
    }

    pub fn get(&mut self, id: LookupId) -> RisResult<Option<&T>> {
        let Some(entry) = self.entries.get_mut(id.index) else {
            return Ok(None);
        };

        match entry.value.take() {
            Some(LookUpValue::IsLoading(receiver)) => {
                match receiver.receive() {
                    Ok(received) => {
                        let asset = received?;
                        entry.value = Some(LookUpValue::Loaded(asset));
                    },
                    Err(receiver) => {
                        entry.value = Some(LookUpValue::IsLoading(receiver));
                    },
                }
            },
            value => entry.value = value,
        }

        match entry.value.as_ref() {
            Some(LookUpValue::Loaded(asset)) => Ok(Some(asset)),
            _ => Ok(None),
        }
    }

    pub fn search_id(&self, asset_id: AssetId) -> Option<LookupId> {
        self.entries.iter()
            .position(|x| x.asset_id == asset_id)
            .map(|x| LookupId{index: x})
    }
}
