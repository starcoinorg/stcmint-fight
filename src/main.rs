use starcoin_vm_types::account_address::AccountAddress;
use stcmint_fight::chain::BlockSnapshot;
use stcmint_fight::{Address, Race};
use structopt::StructOpt;

#[derive(Debug, Clone, StructOpt, Default)]
#[structopt(name = "stcmint-fight", about = "Starcoin mining competition")]
pub struct Opt {
    #[structopt(long, short = "t", default_value = "10")]
    pub top: u16,
    #[structopt(long, short = "d")]
    pub data_dir: String,
    #[structopt(long, short = "s")]
    pub start_timestamp: Option<u64>,
    #[structopt(long, short = "e")]
    pub end_timestamp: Option<u64>,
    #[structopt(long, short = "l", default_value = "0")]
    pub luck: u16,
}

fn main() {
    let opts: Opt = Opt::from_args();
    let top_n = opts.top;
    let path = opts.data_dir;
    let end_timestamp = opts.end_timestamp;
    let luck_n = opts.luck;

    // Seed nodes maintain by starcoin-core
    let black_list = vec![
        "00000000000000000000000000000001",
        "8fed3341166cfe62a8a19380641ff7a7",
        "0f616dd670f3d978eb8e662ed0895614",
        "63af4e1cf4e6345df840f4c57597a0f6",
        "e2b2a491170934e8c72ef559405e1231",
        "7824cf43c641946aed9a2918f3a6c118",
        "b5f00297976c31923caaa39eab22cfed",
    ];
    let start_timestamp = match opts.start_timestamp {
        Some(t) => t,
        None => 1608220800,
    };

    let black_list = black_list
        .into_iter()
        .map(|a| AccountAddress::from_hex_literal(a).unwrap())
        .collect();

    let chain = BlockSnapshot::load_from_db(path, start_timestamp, end_timestamp).unwrap();
    if luck_n != 0 {
        println!("Lucky winners:");
        let luckies = Race::select(luck_n, &chain, black_list);
        for l in luckies {
            println!("{:?}", l);
        }
    } else {
        let mut topies: Vec<Address> = Race::top(top_n, &chain).into_iter().collect();

        topies.sort_by(|a, b| b.minted_blocks.cmp(&a.minted_blocks));
        println!(
            "Top {:?} winners from timestamp: {}",
            top_n, start_timestamp
        );
        for t in topies {
            println!("{:?}", t);
        }
    }
}
