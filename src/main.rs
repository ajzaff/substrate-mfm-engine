mod lib;

use lib::*;

fn main() {
    let mut records = [None; 4];
    let bounds = (2, 2);

    let m = Grid::new(&mut records[..], bounds);

    println!("Hello, world {:?}!", m);
}
