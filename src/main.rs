mod arith;
mod base;
mod mfm;
mod stack;

use arith::*;
use stack::*;
use std::str::FromStr;

#[macro_use]
extern crate bitflags;

#[macro_use]
extern crate lalrpop_util;

lalrpop_mod!(pub substrate); // syntesized by LALRPOP

fn main() {
    //     let x = U96(5);
    //     let y = U96(10);

    //     println!("{}", x + y)
    let p = substrate::DefaultSymmetriesParser::new();

    let res = p.parse(".symmetries R000L | R090L");

    if res.is_ok() {
        println!("{:?}", res.unwrap())
    } else {
        println!("{}", res.unwrap_err())
    }
}
