mod lib;

use lib::*;

fn main() {
    let sites = [0; 100];
    let bounds = (10, 10);
    let runtimes = [Runtime::new(); 2];
    let physics = Physics { elements: &[] };

    let m = Tile::new(&sites, bounds, &physics);
}
