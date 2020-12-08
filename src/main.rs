use stcmint_fight::chain::BlockSnapshot;
use stcmint_fight::Race;

fn main() {
    let path = std::path::Path::new("/Users/fikgol/workspaces/stcmint-fight/starcoindb");
    let start_timestamp = 1607090400;
    let end_timestamp = 1607392800;
    let top_n = 10;
    let lucky_n = 2;

    let chain = BlockSnapshot::load_from_db(path, start_timestamp, end_timestamp).unwrap();
    let luckies = Race::select(top_n, &chain);
    let topies = Race::top(lucky_n, &chain);
    println!("Top winners: {:?}", topies);
    println!("Lucky winners: {:?}", luckies);
}
