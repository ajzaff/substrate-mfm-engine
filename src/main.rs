mod lib;

use lib::*;

fn main() {
    let x = Site::from_usize(411);

    println!("{:?}", x);
}
