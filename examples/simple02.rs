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
fn check_length_10(
    data: CheckState<Vec<usize>, ErrState<Unchecked>>) 
-> 
CheckOutcome<Vec<usize>, ErrState<Checked>, ValidateErr>
{
    if data.value.len() == 10 {
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
    let s= vec![0,1,2,3,4,5,6,7,8,9];
    let checker = check_length_10; 

    // call `Check` trait method
    let r = checker.check(
        CheckState::new(s)
    );
    match r {
        CheckOutcome::Passed(v) => {
            println!("test case: {:?}", v.value);
            println!("Passed!");
        }
        CheckOutcome::Failed{state, err} => {
            println!("test case: {:?}", state.value);
            println!("{:?}", err)
        }
    }
    println!("---")
}
