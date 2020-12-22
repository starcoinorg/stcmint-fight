use crate::{Address, AddressPool};
use anyhow::Result;
use starcoin_config::RocksdbConfig;
use starcoin_chain::BlockChain;
use starcoin_chain_api::ChainReader;
use starcoin_storage::cache_storage::CacheStorage;
use starcoin_storage::db_storage::DBStorage;
use starcoin_storage::storage::StorageInstance;
use starcoin_storage::{BlockStore, Storage, VEC_PREFIX_NAME};
use starcoin_vm_types::account_address::AccountAddress;
use starcoin_vm_types::time::TimeServiceType;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::path::Path;
use std::sync::Arc;

pub struct BlockSnapshot {
    address_blocks: HashMap<AccountAddress, u32>,
    start_block_num: u64,
    end_block_num: u64,
    chain: BlockChain,
}

impl BlockSnapshot {
    pub fn load_from_db<P: AsRef<Path> + Clone>(
        path: P,
        start_timestamp: u64,
        end_timestamp: Option<u64>,
    ) -> Result<Self> {
        let mut address_blocks = HashMap::new();
        let mut start_block_num = 0;
        let mut end_block_num = 0;
        let storage = Storage::new(StorageInstance::new_cache_and_db_instance(
            CacheStorage::new(),
            DBStorage::open_with_cfs(path, VEC_PREFIX_NAME.to_vec(), true, RocksdbConfig::default())?
        ))?;
        println!("{:?}", storage.get_block_by_number(1));
        let head_block_hash = storage
            .get_startup_info()?
            .ok_or(anyhow::anyhow!("Failed to get startup info"))?
            .main;
        let chain = BlockChain::new(
            TimeServiceType::RealTimeService.new_time_service(),
            head_block_hash,
            Arc::new(storage),
        )?;

        for number in 0.. {
            let header = if let Some(header) = chain
                .get_header_by_number(number)? {
                header
            } else {
                println!("Latest block number is {}", number - 1);
                break;
            };

            if header.timestamp / 1000 <= start_timestamp {
                continue;
            }
            if header.timestamp / 1000 > end_timestamp.unwrap_or(u64::max_value()) {
                break;
            }
            if start_block_num == 0 {
                start_block_num = header.number;
            }

            let author = header.author;
            if let Some(&blocks) = address_blocks.get(&author) {
                address_blocks.insert(author, blocks + 1);
            } else {
                address_blocks.insert(author, 1);
            }
            end_block_num = header.number;
        }
        Ok(Self {
            address_blocks,
            start_block_num,
            end_block_num,
            chain,
        })
    }
}

impl AddressPool for BlockSnapshot {
    fn get_pool(&self) -> Vec<Address> {
        let mut address_blocks: Vec<Address> = self
            .address_blocks
            .iter()
            .map(|(a, w)| Address {
                add: a.to_owned(),
                minted_blocks: w.to_owned(),
                weight: 1 + (w.to_owned() as f32).log2() as u32,
            })
            .collect();
        address_blocks.sort_by(|a, b| match a.minted_blocks.cmp(&b.minted_blocks) {
            Ordering::Equal => a.add.cmp(&b.add),
            other => other,
        });
        address_blocks
    }

    fn get_seeds(&self, c: u16) -> Result<Vec<u32>> {
        let mut nonces = vec![];
        let part = (self.end_block_num - self.start_block_num) / c as u64;
        for i in 1..c + 1 {
            let nonce = self
                .chain
                .get_header_by_number(part * i as u64)?
                .ok_or(anyhow::anyhow!("Failed to get header"))?
                .nonce;
            nonces.push(nonce);
        }
        Ok(nonces)
    }
}
