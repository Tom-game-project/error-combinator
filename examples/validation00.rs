//!
//! ```sh
//! cargo run --example validation00
//! ```
//!

use error_combinator::{
    check::{
        Check, CheckOutcome, CheckState
    },
    cmberr::{
        VecCombine
    }
};
use std::marker::PhantomData;

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
    data: CheckState<&str, ErrState<Unchecked, Unchecked, Unchecked, Unchecked>>) 
-> 
CheckOutcome<&str, ErrState<Checked, Unchecked, Unchecked, Unchecked>, ValidateErr>
{
    if data.value.starts_with("hello") {
        CheckOutcome::Passed(
            CheckState::new(data.value)
        )
    } else {
        CheckOutcome::Failed{
            state: CheckState::new(data.value),
            err: ValidateErr::CheckStartsWithHelloErr
        }
    }
}

fn check_min6(
    data: CheckState<&str, ErrState<Checked, Unchecked, Unchecked, Unchecked>>) 
-> 
CheckOutcome<&str, ErrState<Checked, Checked, Unchecked, Unchecked>, ValidateErr>
{
    if 6 < data.value.len() {
        CheckOutcome::Passed(
            CheckState::new(data.value)
        )
    }
    else {
        CheckOutcome::Failed{
            state: CheckState::new(data.value),
            err: ValidateErr::CheckMin6Err
        }
    }
}

fn check_ends_with_world(
    data: CheckState<&str, ErrState<Checked, Checked, Unchecked, Unchecked>>) 
-> 
CheckOutcome<&str, ErrState<Checked, Checked, Checked, Unchecked>, ValidateErr>
{
    if data.value.ends_with("world") {
        CheckOutcome::Passed(
            CheckState::new(data.value),
        )
    } else {
        CheckOutcome::Failed{
            state: CheckState::new(data.value),
            err: ValidateErr::CheckEndsWithWorldErr
        }
    }
}

fn check_includes_abc(
    data: CheckState<&str, ErrState<Checked, Checked, Checked, Unchecked>>) 
-> 
CheckOutcome<&str, ErrState<Checked, Checked, Checked, Checked>, ValidateErr>
{
    if let Some(_) = data.value.find("abc") {
        CheckOutcome::Passed(
            CheckState::new(data.value)
        )
    } else {
        CheckOutcome::Failed{
            state: CheckState::new(data.value),
            err: ValidateErr::CheckIncludesAbcErr
        }
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
            check_starts_with_hello
            .and::<_, VecCombine<ValidateErr>>(check_min6)
            .and::<_, VecCombine<ValidateErr>>(check_ends_with_world)
            .and::<_, VecCombine<ValidateErr>>(check_includes_abc);

        let r = checker.check(
            CheckState::new(s)
        );

        println!("test case: \"{}\"", s);
        match r {
            CheckOutcome::Passed(_v) => {
                println!("Passed!");
            }
            CheckOutcome::Failed{state:_, err} => {
                println!("Failed because");
                println!("{:?}", err)
            }
        }
        println!("---")
    }
}
