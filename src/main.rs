mod lib;

use lib::*;

fn main() {
    let records = [None; 4];
    let bounds = (2, 2);

    let m = Grid::new(&records[..], bounds);

    println!("Hello, world {:?}!", m);
}
