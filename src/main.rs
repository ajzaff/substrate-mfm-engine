mod ast;
mod base;
mod mfm;
mod stack;

#[macro_use]
extern crate bitflags;

#[macro_use]
extern crate lalrpop_util;

lalrpop_mod!(pub substrate); // syntesized by LALRPOP

fn main() {
    //     let x = U96(5);
    //     let y = U96(10);

    //     println!("{}", x + y)
    let p = substrate::InstructionParser::new();

    let res = p.parse("swap c,$b");

    if res.is_ok() {
        println!("{:?}", res.unwrap())
    } else {
        println!("{}", res.unwrap_err())
    }
}
