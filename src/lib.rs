pub mod chain;

use anyhow::Result;
use starcoin_vm_types::account_address::AccountAddress;
use std::collections::HashSet;
use std::fmt;
use std::fmt::{Debug, Formatter};
use std::iter::FromIterator;

pub trait AddressPool {
    fn get_pool(&self) -> Vec<Address>;
    fn get_seeds(&self, c: u16) -> Result<Vec<u32>>;
}

#[derive(Eq, Ord, PartialOrd, PartialEq, Hash, Clone)]
pub struct Address {
    pub add: AccountAddress,
    pub minted_blocks: u32,
    pub weight: u32,
}

impl Debug for Address {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("Address")
            .field("address", &self.add.to_string())
            .field("minted_blocks", &self.minted_blocks)
            .field("weight", &self.weight)
            .finish()
    }
}

pub struct Race;

impl Race {
    pub fn top<T: AddressPool>(n: u16, pool: &T) -> HashSet<Address> {
        let mut pool = pool.get_pool();
        pool.reverse();
        pool.into_iter().take(n as usize).collect()
    }

    pub fn select<T: AddressPool>(
        n: u16,
        input: &T,
        black_list: Vec<AccountAddress>,
    ) -> HashSet<Address> {
        let seeds = input.get_seeds(n).unwrap();
        let mut pool = vec![];
        for address in input.get_pool().iter() {
            for _ in 0..address.weight {
                pool.push(address.clone());
            }
        }

        let mut selected: HashSet<Address> = HashSet::new();
        let pool_size = pool.len() as u32;
        let black_set: HashSet<AccountAddress> = HashSet::from_iter(black_list.iter().cloned());
        println!("{:?}", black_set);
        for mut nonce in seeds {
            loop {
                let address = pool.get((nonce % pool_size) as usize).unwrap();
                if !selected.contains(address) && !black_set.contains(&address.add) {
                    selected.insert(address.clone());
                    break;
                }
                nonce += 1;
            }
        }
        selected
    }
}
