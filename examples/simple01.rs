use error_combinator::{check::{
        Check,        // trait
        CheckOutcome, // struct
        CheckState,   // struct
        ResultCheck,  // struct
        ResultCheckBuilder,  // struct
    }, cmberr::VecCombine};

use std::marker::PhantomData;

struct Checked;
struct Unchecked;
struct ErrState<CheckLength10, CheckStartsWith0> {
    _check_starts_with_hello: PhantomData<CheckLength10>,
    _check_starts_with_0: PhantomData<CheckStartsWith0>,
}

#[derive(Debug)]
enum ValidateErr {
    CheckLength10,
}

fn check_length_10(data: &Vec<i32>) -> Result<(), ValidateErr> {
    if data.len() == 10 {
        Ok(())
    } else {
        Err(ValidateErr::CheckLength10)
    }
}

fn check_start_with_0(data: &Vec<i32>) -> Result<(), ValidateErr> {
    if data[0] == 0 {
        Ok(())
    } else {
        Err(ValidateErr::CheckLength10)
    }
}

fn main() {
    let s= vec![0,1,2,3,4,5,6,7,8,9];
    let checker0 = 
        ResultCheckBuilder::<ErrState<Unchecked, Unchecked>, ErrState<Checked, Unchecked>>::build(check_length_10);
    let checker1 = 
        ResultCheckBuilder::<ErrState<Checked, Unchecked>, ErrState<Checked, Checked>>::build(check_start_with_0);

    let checker = 
        checker0
        .and::<_, VecCombine<ValidateErr>>(checker1);

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
