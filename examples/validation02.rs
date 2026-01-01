//!
//! ```sh
//! cargo run --example validation03
//! ```
//!

use error_combinator::{
    check::{
        Check,
        CheckState, 
        check_ref
    },
    cmberr::VecCombine
};
use std::{marker::PhantomData};

struct Checked;
struct Unchecked;
struct ErrState<CheckStartsWithHello, CheckMin3, CheckEndsWithWorld, CheckIncludesAbc> {
    _check_starts_with_hello: PhantomData<CheckStartsWithHello>,
    _check_min3: PhantomData<CheckMin3>,
    _check_ends_with_world: PhantomData<CheckEndsWithWorld>,
    _check_includes_abc: PhantomData<CheckIncludesAbc>
}

#[derive(Debug)]
enum ValidateErr {
    CheckStartsWithHelloErr,
    CheckMin6Err,
    CheckEndsWithWorldErr,
    CheckIncludesAbcErr,
}

fn check_starts_with_hello(
    data: &str
)
-> Result<(), ValidateErr>

{
    if data.starts_with("hello") {
        Ok(())
    } else {
        Err(ValidateErr::CheckStartsWithHelloErr)
    }
}

fn check_min6(
    data: &str
)
-> 
Result<(), ValidateErr>
{
    if 6 < data.len() {
        Ok(())
    }
    else {
        Err(ValidateErr::CheckMin6Err)
    }
}

fn check_ends_with_world(
    data: &str
)
-> 
Result<(), ValidateErr>
{
    if data.ends_with("world") {
        Ok(())
    } else {
        Err(ValidateErr::CheckEndsWithWorldErr)
    }
}

fn check_includes_abc(
    data: &str
)
-> 
Result<(), ValidateErr>
{
    if let Some(_) = data.find("abc") {
        Ok(())
    } else {
        Err(ValidateErr::CheckIncludesAbcErr)
    }
}

fn main() {
    let check_list  = [
        "hello abc world",
        "abc world",
        "hello world",
        "hello abc",
        "hello--",
        "abc----", 
        "world--"
    ];

    for s in check_list {
        let checker =
            check_ref::<
                str, 
                ErrState<Unchecked, Unchecked, Unchecked, Unchecked>,
                ErrState<Checked, Unchecked, Unchecked, Unchecked>,
                _, 
                _
            >(check_starts_with_hello)
            .or::<
                _, 
                VecCombine<ValidateErr>
            >(
            check_ref::<
                str,
                ErrState<Checked, Unchecked, Unchecked, Unchecked>,
                ErrState<Checked, Checked, Unchecked, Unchecked>,
                _,
                _
            >(check_min6)
            )
            .or::<
                _, 
                VecCombine<ValidateErr>>
            (
            check_ref::<
                str,
                ErrState<Checked, Checked, Unchecked, Unchecked>,
                ErrState<Checked, Checked, Checked, Unchecked>,
                _,
                _
            >(check_ends_with_world))
            .or::<
                _, 
                VecCombine<ValidateErr>>
            (
            check_ref::<
                str,
                ErrState<Checked, Checked, Checked, Unchecked>,
                ErrState<Checked, Checked, Checked, Checked>,
                _,
                _
            >(check_includes_abc));

        let r = checker.check(
            CheckState::new(s)
        );

        println!("test case: \"{}\"", s);
        match r.to_result() {
            Ok(v) => {
                println!("\"{}\" Passed!", v);
            }
            Err(err) => {
                println!("Failed because");
                println!("{:?}", err)
            }
        }
        println!("---")
    }
}
