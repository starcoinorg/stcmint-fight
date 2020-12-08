pub mod db;

use std::collections::HashSet;
use std::fmt::{Debug, Formatter};
use std::fmt;
use anyhow::Result;

pub trait AddressPool {
    fn get_pool(&self) -> Vec<Address>;
    fn get_seeds(&self, c: u16) -> Result<Vec<u32>>;
}

#[derive(Eq, Ord, PartialOrd, PartialEq, Hash, Clone)]
pub struct Address {
    pub add: Vec<u8>,
    pub minted_blocks: u32,
    pub weight: u32,
}

impl Debug for Address {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("Address")
            .field("address", &hex::encode(self.add.clone()))
            .field("minted_blocks", &self.minted_blocks)
            .field("weight", &self.weight)
            .finish()
    }
}

pub struct Race;

impl Race {
    pub fn top<T: AddressPool>(n: u16, pool: &T) -> HashSet<Address> {
        let mut ret = HashSet::new();
        let mut pool = pool.get_pool();
        pool.reverse();
        for i in 0..n {
            ret.insert(pool[i as usize].clone());
        }
        ret
    }

    pub fn select<T: AddressPool>(n: u16, input: &T, black_list: HashSet<Address>) -> HashSet<Address> {
        let seeds = input.get_seeds(n).unwrap();
        let mut pool = vec![];
        for address in input.get_pool().iter() {
            for _ in 0..address.weight {
                pool.push(address.clone());
            }
        }

        let mut selected: HashSet<Address> = HashSet::new();
        let pool_size = pool.len() as u32;
        for mut nonce in seeds {
            loop {
                let address = pool.get((nonce % pool_size) as usize).unwrap();
                if black_list.contains(address){
                    continue;
                }
                if !selected.contains(address){
                    selected.insert(address.clone());
                    break;
                }
                nonce += 1;
            }
        }
        selected
    }
}