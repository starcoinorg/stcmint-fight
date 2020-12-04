use std::collections::HashSet;

#[derive(Eq, Ord, PartialOrd, PartialEq, Hash, Debug)]
pub struct Address {
    add: Vec<u8>,
    weight: u32,
}

impl Address {
    fn get_weight(&self) -> u32 {
        self.weight
    }
}

struct AddressPool(Vec<Address>);

impl AddressPool {
    pub fn select(&mut self, nonces: Vec<u32>) -> HashSet<&Address> {
        assert!(nonces.len() < self.0.len());
        let mut selected = HashSet::new();
        let mut pool = vec![];
        self.0.sort_by(|a, b| a.get_weight().cmp(&b.get_weight()));
        for address in self.0.iter() {
            for _ in 0..address.get_weight() {
                pool.push(address);
            }
        }
        let pool_size = pool.len() as u32;
        for mut nonce in nonces {
            loop {
                let address = pool[(nonce % pool_size) as usize];
                if !selected.contains(address) {
                    selected.insert(address);
                    break;
                }
                nonce += 1;
            }
        }
        selected
    }
}

#[test]
fn test_select() {
    let mut pool = AddressPool {
        0: vec![Address { add: vec![0], weight: 1 },
                Address { add: vec![1], weight: 2 },
                Address { add: vec![2], weight: 3 },
                Address { add: vec![3], weight: 4 },
        ]
    };
    let adds = pool.select(vec![120]);
    assert_eq!(adds.contains(&Address { add: vec![0], weight: 1 }), true);
}
