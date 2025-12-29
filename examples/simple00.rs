use error_combinator::{
    check::{
        Check,        // trait
        CheckOutcome, // struct
        CheckState    // struct
    },
};

use std::marker::PhantomData;

struct Checked;
struct Unchecked;
struct ErrState<CheckStartsWithHello> {
    _check_starts_with_hello: PhantomData<CheckStartsWithHello>,
}

#[derive(Debug)]
enum ValidateErr {
    CheckStartsWithHelloErr,
}

// function that has `Fn(CheckState<T, Pre>) -> CheckOutcome<T, State, E>` type be implemented `Check trait` automatically
fn check_starts_with_hello(
    data: CheckState<&str, ErrState<Unchecked>>) 
-> 
CheckOutcome<&str, ErrState<Checked>, ValidateErr>
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

fn main() {
    let s= "hello abc world";
    let checker = check_starts_with_hello; 

    // call `Check` trait method
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
