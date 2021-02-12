mod lib;

use lib::*;

fn main() {
    let sites = [0; 100];
    let bounds = (10, 10);
    let runtimes = [RuntimeState::new(); 2];

    let m = Tile::new(&sites, bounds, &runtimes);
}
