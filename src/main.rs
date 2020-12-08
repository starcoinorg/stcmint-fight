use starcoin_vm_types::account_address::AccountAddress;
use stcmint_fight::chain::BlockSnapshot;
use stcmint_fight::{Address, Race};

fn main() {
    let black_list = vec![
        "8fed3341166cfe62a8a19380641ff7a7",
        "0f616dd670f3d978eb8e662ed0895614",
        "63af4e1cf4e6345df840f4c57597a0f6",
        "e2b2a491170934e8c72ef559405e1231",
        "7824cf43c641946aed9a2918f3a6c118",
        "b5f00297976c31923caaa39eab22cfed",
    ];

    let path = std::path::Path::new("./starcoindb/db");
    let start_timestamp = 1607090400;
    let end_timestamp = 1607392800;
    let top_n = 100;
    let lucky_n = 2;

    let black_list = black_list
        .into_iter()
        .map(|a| AccountAddress::from_hex_literal(a).unwrap())
        .collect();

    println!(
        "Select top {:?} winners and lucky {:?} winner, between timestamp: {} to {}",
        top_n, lucky_n, start_timestamp, end_timestamp
    );
    let chain = BlockSnapshot::load_from_db(path, start_timestamp, end_timestamp).unwrap();
    let luckies = Race::select(lucky_n, &chain, black_list);
    let mut topies: Vec<Address> = Race::top(top_n, &chain).into_iter().collect();
    topies.sort_by(|a, b| b.minted_blocks.cmp(&a.minted_blocks));
    println!("Top winners:");
    for t in topies {
        println!("{:?}", t);
    }
    println!("Lucky winners:");
    for l in luckies {
        println!("{:?}", l);
    }
}
