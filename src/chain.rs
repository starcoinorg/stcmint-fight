use starcoin_storage::{Storage, BlockStore};
use starcoin_storage::storage::StorageInstance;
use starcoin_storage::cache_storage::CacheStorage;
use starcoin_storage::db_storage::DBStorage;
use anyhow::Result;
use std::path::Path;
use std::collections::HashMap;
use crate::Address;


pub trait AddressPool {
    fn get_pool(&self) -> Vec<Address>;
    fn get_seeds(&self, c: u16) -> Result<Vec<u32>>;
}

pub struct BlockSnapshot {
    address_blocks: HashMap<Vec<u8>, u32>,
    start_block_num: u64,
    end_block_num: u64,
    storage: Storage,
}

impl BlockSnapshot {
    pub fn load_from_db<P: AsRef<Path> + Clone>(path: P, start_timestamp: u64, end_timestamp: u64) -> Result<Self> {
        let mut address_blocks = HashMap::new();
        let mut start_block_num = 0;
        let mut end_block_num = 0;
        let storage = Storage::new(StorageInstance::new_cache_and_db_instance(
            CacheStorage::new(),
            DBStorage::new(path)?,
        ))?;
        for number in 0.. {
            if let Ok(Some(header)) = storage.get_block_header_by_number(number) {
                if header.timestamp / 1000 < start_timestamp || header.timestamp > end_timestamp {
                    continue;
                }
                if start_block_num == 0 {
                    start_block_num = header.number;
                }
                let author = header.author.to_vec();
                if let Some(&blocks) = address_blocks.get(&author) {
                    address_blocks.insert(author, blocks + 1);
                } else {
                    address_blocks.insert(author, 0);
                }
                end_block_num = header.number;
            } else {
                break;
            }
        }
        Ok(Self {
            address_blocks,
            start_block_num,
            end_block_num,
            storage,
        })
    }
}

impl AddressPool for BlockSnapshot {
    fn get_pool(&self) -> Vec<Address> {
        let mut address_blocks: Vec<Address> = self.address_blocks.iter().map(|(a, w)| Address { add: a.to_owned(), weight: w.to_owned() }).collect();
        address_blocks.sort_by(|a, b| a.weight.cmp(&b.weight));
        address_blocks
    }

    fn get_seeds(&self, c: u16) -> Result<Vec<u32>> {
        let mut nonces = vec![];
        let part = (self.end_block_num - self.start_block_num) / c as u64;
        for i in 1..c + 1 {
            let nonce = self.storage.get_block_header_by_number(part * i as u64)?
                .ok_or(anyhow::anyhow!("Failed to get header"))?.nonce;
            nonces.push(nonce);
        }
        Ok(nonces)
    }
}