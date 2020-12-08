use stcmint_fight::chain::BlockSnapshot;
use stcmint_fight::Race;

fn main() {
    let path = std::path::Path::new("/Users/fikgol/workspaces/stcmint-fight/starcoindb");
    let chain = BlockSnapshot::load_from_db(path, 1607090400, 1607392800).unwrap();
    let luckies = Race::select(2, &chain);
    let topies = Race::top(2, &chain);
    println!("Top winners: {:?}", topies);
    println!("Lucky winners: {:?}", luckies);
}
